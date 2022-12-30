use super::{VerbContext, UserVerb, UserVerbRef, UResult, UserError, user_error,
            get_player_item_or_fail, search_item_for_user};
use async_trait::async_trait;
use ansi::{ansi, flow_around, word_wrap};
use crate::db::ItemSearchParams;
use crate::models::{item::{Item, LocationActionType, Subattack}};
use crate::static_content::room::{self, Direction};
use itertools::Itertools;
use std::sync::Arc;
use crate::language::pluralise;

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
                      item.display_for_session(&ctx.session_dat),
                      item.details_for_session(&ctx.session_dat).unwrap_or("")
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

pub async fn describe_room(ctx: &VerbContext<'_>, item: &Item,
                           room: &room::Room, contents: &str) -> UResult<()> {
    let zone = room::zone_details().get(room.zone).map(|z|z.display).unwrap_or("Outside of time");
    ctx.trans.queue_for_session(
        ctx.session,
        Some(&flow_around(&render_map(room, 5, 5), 10, "  ",
                          &word_wrap(&format!("{} ({})\n{}.{}\n{}\n",
                                              item.display_for_session(&ctx.session_dat),
                                              zone,
                                              item.details_for_session(
                                                  &ctx.session_dat).unwrap_or(""),
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

    let all_groups: Vec<Vec<&Arc<Item>>> = items
        .iter()
        .group_by(|i| &i.display)
        .into_iter()
        .map(|(_, g)|g.collect::<Vec<&Arc<Item>>>())
        .collect::<Vec<Vec<&Arc<Item>>>>();
    
    for group_items in all_groups {
        let head = &group_items[0];
        let is_creature = head.item_type == "player" || head.item_type.starts_with("npc");
        buf.push(' ');
        if group_items.len() > 1 {
            buf.push_str(&format!("{} ", group_items.len()))
        } else if !head.pronouns.is_proper {
            buf.push_str("A ");
        }
        let mut disp = head.display_for_session(&ctx.session_dat).to_owned();
        if group_items.len() > 1 {
            disp = pluralise(&disp);
        }   
        buf.push_str(&disp);
        buf.push_str(if group_items.len() > 1 { " are " } else { " is "});
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
                                it.display_for_session(&ctx.session_dat)
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
        let item: Arc<Item> = if rem_trim == "" {
            ctx.trans.find_item_by_type_code(heretype, herecode).await?
                .ok_or_else(|| UserError("Sorry, that no longer exists".to_owned()))?
        } else if let Some(dir) = Direction::parse(&rem_trim) {
            if heretype != "room" {
                // Fix this when we have planes / boats / roomkits.
                user_error("Navigating outside rooms not yet supported.".to_owned())?
            } else {
                if let Some(room) = room::room_map_by_code().get(herecode) {
                    match room.exits.iter().find(|ex| ex.direction == *dir) {
                        None => user_error("There is nothing in that direction".to_owned())?,
                        Some(exit) => {
                            match room::resolve_exit(room, exit) {
                                None => user_error("There is nothing in that direction".to_owned())?,
                                Some(room2) =>
                                    ctx.trans.find_item_by_type_code("room", room2.code).await?
                                    .ok_or_else(|| UserError("Sorry, that no longer exists".to_owned()))?

                            }
                        }
                    }
                } else {
                    user_error("Can't find your current location".to_owned())?
                }
            }
        } else if rem_trim == "me" || rem_trim == "self" {
            player_item.clone()
        } else {
            search_item_for_user(
                &ctx,
                &ItemSearchParams {
                    include_contents: true,
                    include_loc_contents: true,
                    ..ItemSearchParams::base(&player_item, &rem_trim)
                }
            ).await?
        };
        if item.item_type != "room" {
            describe_normal_item(ctx, &item).await?;
        } else {
            let room =
                room::room_map_by_code().get(item.item_code.as_str())
                .ok_or_else(|| UserError("Sorry, that room no longer exists".to_owned()))?;
            describe_room(ctx, &item, &room, &list_item_contents(ctx, &item).await?).await?;
        }
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
