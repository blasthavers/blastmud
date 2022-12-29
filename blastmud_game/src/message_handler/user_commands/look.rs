use super::{VerbContext, UserVerb, UserVerbRef, UResult, UserError, user_error, explicit_if_allowed};
use async_trait::async_trait;
use ansi::{ansi, flow_around, word_wrap};
use crate::models::{user::User, item::Item};
use crate::static_content::room::{self, Direction};

pub fn get_user_or_fail<'l>(ctx: &'l VerbContext) -> UResult<&'l User> { 
    ctx.user_dat.as_ref()
        .ok_or_else(|| UserError("Not logged in".to_owned()))
}

pub async fn get_player_item_or_fail(ctx: &VerbContext<'_>) -> UResult<Item> {
    Ok(ctx.trans.find_item_by_type_code(
        "player", &get_user_or_fail(ctx)?.username.to_lowercase()).await?
       .ok_or_else(|| UserError("Your player is gone, you'll need to re-register or ask an admin".to_owned()))?)
}

pub fn render_map(room: &room::Room, width: usize, height: usize) -> String {
    let mut buf = String::new();
    let my_loc = &room.grid_coords;
    let min_x = my_loc.x - (width as i64) / 2;
    let max_x = min_x + (width as i64);
    let min_y = my_loc.y - (height as i64) / 2;
    let max_y = min_y + (height as i64);
    for y in min_y..max_y {
        for x in min_x..max_x {
            if my_loc.x == x && my_loc.y == y {
                buf.push_str(ansi!("<bgblue><red>()<reset>"))
            } else {
                buf.push_str(room::room_map_by_zloc()
                             .get(&(&room.zone, &room::GridCoords { x, y, z: my_loc.z }))
                             .map(|r| r.short)
                             .unwrap_or("  "));
            }
        }
        buf.push('\n');
    }
    buf
}

pub async fn describe_normal_item(ctx: &VerbContext<'_>, item: &Item) -> UResult<()> {
    ctx.trans.queue_for_session(
        ctx.session,
        Some(&format!("{}\n{}",
                      explicit_if_allowed(
                          ctx,
                          &item.display,
                          item.display_less_explicit.as_ref().map(|s|&**s)),
                      explicit_if_allowed(
                          ctx,
                          item.details.as_ref().map(|v|&**v).unwrap_or(""),
                          item.details_less_explicit.as_ref().map(|s|&**s)),
        ))
    ).await?;
    Ok(())
}

fn exits_for(room: &room::Room) -> String {
    let exit_text: Vec<String> =
        room.exits.iter().map(|ex| format!(ansi!("<yellow>{}"),
                                           ex.direction.describe())).collect();
    format!(ansi!("<cyan>[ Exits: <bold>{} <reset><cyan>]<reset>"), exit_text.join(" "))
}

pub async fn describe_room(ctx: &VerbContext<'_>, room: &room::Room) -> UResult<()> {
    let zone = room::zone_details().get(room.zone).map(|z|z.display).unwrap_or("Outside of time");
    ctx.trans.queue_for_session(
        ctx.session,
        Some(&flow_around(&render_map(room, 5, 5), 10, "  ",
                          &word_wrap(&format!("{} ({})\n{}\n{}\n", room.name, zone,
                                              explicit_if_allowed(ctx, room.description,
                                                                  room.description_less_explicit),
                                              exits_for(room)),
                                     |row| if row >= 5 { 80 } else { 68 }), 68))
    ).await?;
    Ok(())
}

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let player_item = get_player_item_or_fail(ctx).await?;

        let (heretype, herecode) = player_item.location.split_once("/").unwrap_or(("room", "repro_xv_chargen"));
        let (itype, icode) = if remaining == "" {
            Ok((heretype, herecode))
        } else if let Some(dir) = Direction::parse(remaining) {
            if heretype != "room" {
                // Fix this when we have planes / boats / roomkits.
                user_error("Navigating outside rooms not yet supported.".to_owned())
            } else {
                if let Some(room) = room::room_map_by_code().get(herecode) {
                    match room.exits.iter().find(|ex| ex.direction == *dir) {
                        None => user_error("There is nothing in that direction".to_owned()),
                        Some(exit) => {
                            match room::resolve_exit(room, exit) {
                                None => user_error("There is nothing in that direction".to_owned()),
                                Some(room2) => Ok(("room", room2.code))
                            }
                        }
                    }
                } else {
                    user_error("Can't find your current location".to_owned())
                }
            }
        } else {
            user_error("Sorry, I don't understand what you want to look at.".to_owned())
        }?;
        if itype != "room" {
            let item = ctx.trans.find_item_by_type_code(itype, icode).await?
                .ok_or_else(|| UserError("Sorry, that no longer exists".to_owned()))?;
            describe_normal_item(ctx, &item).await?;
        } else {
            let room =
                room::room_map_by_code().get(icode).ok_or_else(|| UserError("Sorry, that room no longer exists".to_owned()))?;
            describe_room(ctx, &room).await?;
        }
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
