use super::{VerbContext, UserVerb, UserVerbRef, UResult, UserError, user_error, explicit_if_allowed};
use async_trait::async_trait;
use ansi::{ansi, flow_around, word_wrap};
use crate::models::{user::User, item::{Item, LocationActionType, Subattack}};
use crate::static_content::room::{self, Direction};
use itertools::Itertools;

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
        Some(&format!("{}\n{}\n",
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

pub async fn describe_room(ctx: &VerbContext<'_>, room: &room::Room, contents: &str) -> UResult<()> {
    let zone = room::zone_details().get(room.zone).map(|z|z.display).unwrap_or("Outside of time");
    ctx.trans.queue_for_session(
        ctx.session,
        Some(&flow_around(&render_map(room, 5, 5), 10, "  ",
                          &word_wrap(&format!("{} ({})\n{}.{}\n{}\n", room.name, zone,
                                              explicit_if_allowed(ctx, room.description,
                                                                  room.description_less_explicit),
                                              contents, exits_for(room)),
                                     |row| if row >= 5 { 80 } else { 68 }), 68))
    ).await?;
    Ok(())
}

async fn list_item_contents<'l>(ctx: &'l VerbContext<'_>, item: &'l Item) -> UResult<String> {
    let mut buf = String::new();
    let mut items = ctx.trans.find_items_by_location(&format!("{}/{}",
                                                          item.item_type, item.item_code)).await?;
    items.sort_unstable_by(|it1, it2| (&it1.display).cmp(&it2.display));

    let all_groups: Vec<Vec<&Item>> = items
        .iter()
        .group_by(|i| &i.display)
        .into_iter()
        .map(|(_, g)|g.collect::<Vec<&Item>>())
        .collect::<Vec<Vec<&Item>>>();
    
    for group_items in all_groups {
        let head = &group_items[0];
        let is_creature = head.item_type == "player" || head.item_type.starts_with("npc");
        buf.push(' ');
        if group_items.len() > 1 {
            buf.push_str(&format!("{} ", group_items.len()))
        } else if !is_creature {
            buf.push_str("A ");
        }
        buf.push_str(
            &explicit_if_allowed(ctx,
                                 &head.display,
                                 head.display_less_explicit.as_ref().map(|v|&**v)));
        buf.push_str(" is ");
        match head.action_type {
            LocationActionType::Sitting => buf.push_str("sitting "),
            LocationActionType::Reclining => buf.push_str("reclining "),
            LocationActionType::Normal | LocationActionType::Attacking(_) if is_creature =>
                buf.push_str("standing "),
            _ => {}
        }
        buf.push_str("here");
        if let LocationActionType::Attacking(subattack) = &head.action_type {
            match subattack {
                Subattack::Powerattacking => buf.push_str(", powerattacking "),
                Subattack::Feinting => buf.push_str(", feinting "),
                Subattack::Grabbing => buf.push_str(", grabbing "),
                Subattack::Wrestling => buf.push_str(", wrestling "),
                _ => buf.push_str(", attacking ")
            }
            match &head.presence_target {
                None => buf.push_str("someone"),
                Some(who) => match who.split_once("/") {
                    None => buf.push_str("someone"),
                    Some((ttype, tcode)) =>
                        match ctx.trans.find_item_by_type_code(ttype, tcode).await? {
                            None => buf.push_str("someone"),
                            Some(it) => buf.push_str(
                                &explicit_if_allowed(ctx,
                                                     &it.display,
                                                     it.display_less_explicit.as_ref().map(|v|&**v))
                            )
                        }
                }
            }
        }
        buf.push('.');
    }
    Ok(buf)
}

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let player_item = get_player_item_or_fail(ctx).await?;

        let rem_trim = remaining.trim().to_lowercase();
        let (heretype, herecode) = player_item.location.split_once("/").unwrap_or(("room", "repro_xv_chargen"));
        let (itype, icode): (String, String) = if rem_trim == "" {
            Ok((heretype.to_owned(), herecode.to_owned()))
        } else if let Some(dir) = Direction::parse(&rem_trim) {
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
                                Some(room2) => Ok(("room".to_owned(), room2.code.to_owned()))
                            }
                        }
                    }
                } else {
                    user_error("Can't find your current location".to_owned())
                }
            }
        } else if rem_trim == "me" || rem_trim == "self" {
            Ok((player_item.item_type.clone(), player_item.item_code.clone()))
        } else {
            match &ctx.trans.resolve_items_by_display_name_for_player(
                &player_item,
                &rem_trim,
                true, true, false, false
            ).await?[..] {
                [] => user_error("Sorry, I couldn't find anything matching.".to_owned()),
                [match_it] => Ok((match_it.item_type.clone(), match_it.item_code.clone())),
                [item1, ..] if item1.display.to_lowercase() == rem_trim ||
                    item1.display_less_explicit.as_ref().map(|x|x.to_lowercase()) == Some(rem_trim) =>
                    Ok((item1.item_type.clone(), item1.item_code.clone())),
                _ => user_error("Sorry, that name is ambiguous, please be more specific.".to_owned())
            }
        }?;
        let item = ctx.trans.find_item_by_type_code(&itype, &icode).await?
            .ok_or_else(|| UserError("Sorry, that no longer exists".to_owned()))?;
        if itype != "room" {
            describe_normal_item(ctx, &item).await?;
        } else {
            let room =
                room::room_map_by_code().get(icode.as_str())
                .ok_or_else(|| UserError("Sorry, that room no longer exists".to_owned()))?;
            describe_room(ctx, &room, &list_item_contents(ctx, &item).await?).await?;
        }
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
