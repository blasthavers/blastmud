use super::{VerbContext, UserVerb, UserVerbRef, UResult,
            ItemSearchParams, user_error,
            get_player_item_or_fail, is_likely_explicit,
            search_item_for_user,
            parsing::parse_to_space};
use crate::static_content::npc::npc_by_code;
use async_trait::async_trait;
use ansi::{ignore_special_characters, ansi};

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let (to_whom_name, say_what_raw) = parse_to_space(remaining);
        let say_what = ignore_special_characters(say_what_raw);
        if say_what == "" {
            user_error("You need to provide a message to send.".to_owned())?;
        }
        let player_item = get_player_item_or_fail(ctx).await?;
        let to_whom = search_item_for_user(ctx, &ItemSearchParams {
            include_loc_contents: true,
            ..ItemSearchParams::base(&player_item, &to_whom_name)
        }).await?;

        match to_whom.item_type.as_str() {
            "npc" => {}
            "player" => {},
            _ => user_error("Only characters (players / NPCs) accept whispers".to_string())?
        }

        ctx.trans.queue_for_session(ctx.session, Some(&format!(
            ansi!("<blue>{} whispers to {}: \"{}\"<reset>\n"),
            player_item.display_for_session(&ctx.session_dat),
            to_whom.display_for_session(&ctx.session_dat),
            say_what
        ))).await?;

        if player_item == to_whom {
            return Ok(());
        }

        match to_whom.item_type.as_str() {
            "npc" => {
                let npc = npc_by_code().get(to_whom.item_code.as_str())
                    .map(Ok)
                    .unwrap_or_else(|| user_error("That NPC is no longer available".to_owned()))?;
                if let Some(handler) = npc.message_handler {
                    handler.handle(ctx, &player_item, &to_whom, &say_what).await?;
                }
            }
            "player" => {
                match ctx.trans.find_session_for_player(&to_whom.item_code).await? {
                    None => user_error("That character is asleep.".to_string())?,
                    Some((other_session, other_session_dets)) => {
                        if other_session_dets.less_explicit_mode && is_likely_explicit(&say_what) {
                            user_error("That player is on a client that doesn't allow explicit \
                                        content, and your message looked explicit, so it wasn't sent."
                                       .to_owned())?
                        } else {
                            ctx.trans.queue_for_session(&other_session, Some(&format!(
                                ansi!("<blue>{} whispers to {}: \"{}\"<reset>\n"),
                                player_item.display_for_session(&ctx.session_dat),
                                to_whom.display_for_session(&ctx.session_dat),
                                say_what
                            ))).await?;
                        }
                    }
                }
            },
            _ => {}
        }
        
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
