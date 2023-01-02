use super::{VerbContext, UserVerb, UserVerbRef, UResult, UserError, user_error,
            get_player_item_or_fail};
use async_trait::async_trait;
use ansi::{ansi, flow_around};
use crate::{
    models::item::Item,
    static_content::room::{self, Direction}
};
use std::sync::Arc;

pub fn render_lmap(room: &room::Room, width: usize, height: usize,
                   captions_needed: &mut Vec<(usize, &'static str, &'static str)>) -> String {
    let mut buf = String::new();
    let my_loc = &room.grid_coords;
    let min_x = my_loc.x - (width as i64) / 2;
    let max_x = min_x + (width as i64);
    let min_y = my_loc.y - (height as i64) / 2;
    let max_y = min_y + (height as i64);
    for y in min_y..max_y {
        for x in min_x..max_x {
            let coord = room::GridCoords { x, y, z: my_loc.z };
            let coord_room = room::room_map_by_zloc()
                .get(&(&room.zone, &coord));
            if my_loc.x == x && my_loc.y == y {
                buf.push_str(ansi!("<bgblue><red> () <reset>"))
            } else {
                let code_capt_opt = coord_room.map(
                    |r| if room.zone == r.zone {
                        (r.short, if r.should_caption {
                            Some((r.name, ((my_loc.x as i64 - r.grid_coords.x).abs() +
                                          (my_loc.y as i64 - r.grid_coords.y).abs()
                            ) as usize)) } else { None })
                    } else {
                        r.secondary_zones.iter()
                            .find(|sz| sz.zone == room.zone)
                            .map(|sz| (sz.short, sz.caption.map(
                                |c| (c, ((my_loc.x as i64 - r.grid_coords.x).abs() +
                                         (my_loc.y as i64 - r.grid_coords.y).abs())
                                     as usize))))
                            .expect("Secondary zone missing")
                    });
                match code_capt_opt {
                    None => buf.push_str("    "),
                    Some((code, capt_opt)) => {
                        if let Some((capt, closeness)) = capt_opt {
                            captions_needed.push((closeness, code, capt));
                        }
                        buf.push('[');
                        buf.push_str(code);
                        buf.push(']');
                    }
                }
            }
            match coord_room.and_then(
                |r| r.exits.iter().find(|ex| ex.direction == Direction::EAST)) {
                None => buf.push(' '),
                Some(_) => buf.push('-')
            }
        }
        for x in min_x..max_x {
            let mut coord = room::GridCoords { x, y, z: my_loc.z };
            let coord_room = room::room_map_by_zloc()
                .get(&(&room.zone, &coord));
            match coord_room.and_then(
                |r| r.exits.iter().find(|ex| ex.direction == Direction::SOUTH)) {
                None => buf.push_str("    "),
                Some(_) => buf.push_str(" |  ")
            }
            let has_se = coord_room.and_then(
                |r| r.exits.iter().find(|ex| ex.direction == Direction::SOUTHEAST))
                .is_some();
            coord.y += 1;
            let coord_room_s = room::room_map_by_zloc()
                .get(&(&room.zone, &coord));
            let has_ne = coord_room_s.and_then(
                |r| r.exits.iter().find(|ex| ex.direction == Direction::NORTHEAST))
                .is_some();
            if has_se && has_ne {
                buf.push('X');
            } else if has_se {
                buf.push('\\');
            } else if has_ne {
                buf.push('/');
            } else {
                buf.push(' ');
            }
        }
        buf.push('\n');
    }
    captions_needed.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    buf
}

pub fn caption_lmap(captions: &Vec<(usize, &'static str, &'static str)>, width: usize, height: usize) -> String {
    let mut buf = String::new();
    for room in captions.iter().take(height) {
        buf.push_str(&format!(ansi!("{}<bold>: {:.*}<reset>\n"), room.1, width, room.2));
    }
    buf
}

pub async fn lmap_room(ctx: &VerbContext<'_>,
                       room: &room::Room) -> UResult<()> {
    let mut captions: Vec<(usize, &'static str, &'static str)> = Vec::new();
    ctx.trans.queue_for_session(
        ctx.session,
        Some(&flow_around(&render_lmap(room, 9, 7, &mut captions), 45, ansi!("<reset>  "),
                          &caption_lmap(&captions, 14, 27), 31
                         ))
    ).await?;
    Ok(())
}

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        if remaining.trim() != "" {
            user_error("map commands don't take anything after them".to_owned())?;
        }
        let player_item = get_player_item_or_fail(ctx).await?;
        let (heretype, herecode) = player_item.location.split_once("/").unwrap_or(("room", "repro_xv_chargen"));
        let room_item: Arc<Item> = ctx.trans.find_item_by_type_code(heretype, herecode).await?
            .ok_or_else(|| UserError("Sorry, that no longer exists".to_owned()))?;
        if room_item.item_type != "room" {
            user_error("Can't map here".to_owned())?;
        } else {
            let room =
                room::room_map_by_code().get(room_item.item_code.as_str())
                .ok_or_else(|| UserError("Sorry, that room no longer exists".to_owned()))?;
            lmap_room(ctx, &room).await?;
        }
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
