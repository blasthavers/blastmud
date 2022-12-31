use super::{
    VerbContext,
    UserVerb,
    UserVerbRef,
    UResult,
    parsing::parse_to_space,
    user_error,
    get_player_item_or_fail
};
use async_trait::async_trait;
use ansi::{ansi, ignore_special_characters};

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let (me, remaining) = parse_to_space(remaining);
        let (as_word, remaining) = parse_to_space(remaining);
        let remaining = ignore_special_characters(remaining.trim());
        if me != "me" || as_word != "as" || remaining == "" {
            user_error(ansi!("Try <bold>describe me as <lt>something><reset>").to_owned())?;
        }

        if remaining.len() < 40 {
            user_error(format!("That's too short by {} characters.", 40 - remaining.len()))?;
        }
        if remaining.len() > 255 {
            user_error(format!("That's too short by {} characters.", remaining.len() - 255))?;
        }

        let mut item = (*get_player_item_or_fail(ctx).await?).clone();
        item.details = Some(remaining);
        ctx.trans.save_item_model(&item).await?;

        ctx.trans.queue_for_session(ctx.session, Some(ansi!("<green>Character description updated.<reset>\n"))).await?;
        
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
