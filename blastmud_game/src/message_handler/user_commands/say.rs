use super::{VerbContext, UserVerb, UserVerbRef, UResult, UserError,
            user_error,
            get_player_item_or_fail, is_likely_explicit};
use crate::models::item::{Item, ItemFlag};
use crate::db::DBTrans;
use async_trait::async_trait;
use ansi::{ignore_special_characters, ansi};

pub async fn say_to_room<'l>(
    trans: &DBTrans,
    from_item: &Item,
    location: &str,
    say_what: &str,
    is_explicit: bool
) -> UResult<()> {
    let (loc_type, loc_code) = location.split_once("/")
        .ok_or_else(|| UserError("Invalid location".to_owned()))?;
    let room_item = trans.find_item_by_type_code(loc_type, loc_code).await?
        .ok_or_else(|| UserError("Room missing".to_owned()))?;
    if room_item.flags.contains(&ItemFlag::NoSay) {
        user_error("Your wristpad vibrates and flashes up an error - apparently it has \
                    been programmed to block your voice from working here.".to_owned())?
    }
    for item in trans.find_items_by_location(location).await? {
        if item.item_type != "player" {
            continue;
        }
        if let Some((session, session_dat)) = trans.find_session_for_player(&item.item_code).await? {
            if session_dat.less_explicit_mode && is_explicit && from_item.item_code != item.item_code {
                continue;
            }
            trans.queue_for_session(&session, Some(&format!(
                ansi!("<yellow>{} says: <reset><bold>\"{}\"<reset>\n"),
                from_item.display_for_session(&session_dat),
                say_what
            ))).await?;
        }
    }
    Ok(())
}

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let say_what = ignore_special_characters(remaining);
        if say_what == "" {
            user_error("You need to provide a message to send.".to_owned())?;
        }
        let player_item = get_player_item_or_fail(ctx).await?;
        say_to_room(ctx.trans, &player_item, &player_item.location,
                    &say_what, is_likely_explicit(&say_what)).await
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
