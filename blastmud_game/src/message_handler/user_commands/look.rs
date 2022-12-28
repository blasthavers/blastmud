use super::{VerbContext, UserVerb, UserVerbRef, UResult, UserError, explicit_if_allowed};
use async_trait::async_trait;
use ansi::{ansi, flow_around};
use crate::models::{user::User, item::Item};
use crate::static_content::room;

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
    for x in min_x..max_x {
        for y in min_y..max_y {
            if my_loc.x == x && my_loc.y == y {
                buf.push_str(ansi!("<bgblue><red>()<reset>"))
            } else {
                buf.push_str(room::room_map_by_loc()
                             .get(&room::GridCoords { x, y, z: my_loc.z })
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
        Some(explicit_if_allowed(
            ctx,
            &item.display,
            item.display_less_explicit.as_ref().map(|s|&**s)))
    ).await?;
    Ok(())
}

pub async fn describe_room(ctx: &VerbContext<'_>, room: &room::Room) -> UResult<()> {
    let zone = room::zone_details().get(room.zone).map(|z|z.display).unwrap_or("Outside of time");
    ctx.trans.queue_for_session(
        ctx.session,
        Some(&flow_around(&render_map(room, 5, 5), 10, "  ",
                          &format!("{} ({})\n{}\n", room.name, zone,
                                   explicit_if_allowed(ctx, room.description,
                                                       room.description_less_explicit)), 68))
    ).await?;
    Ok(())
}

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, _remaining: &str) -> UResult<()> {
        let player_item = get_player_item_or_fail(ctx).await?;
        let (loctype, loccode) = player_item.location.split_once("/").unwrap_or(("room", "repro_xv_chargen"));
        if loctype != "room" {
            let item = ctx.trans.find_item_by_type_code(loctype, loccode).await?
                .ok_or_else(|| UserError("Sorry, that no longer exists".to_owned()))?;
            describe_normal_item(ctx, &item).await?;
        } else {
            let room =
                room::room_map_by_code().get(loccode).ok_or_else(|| UserError("Sorry, that room no longer exists".to_owned()))?;
            describe_room(ctx, &room).await?;
        }
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
