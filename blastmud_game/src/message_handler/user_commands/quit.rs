use super::{
    VerbContext, UserVerb, UserVerbRef, UResult
};
use async_trait::async_trait;
use ansi_macro::ansi;

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &VerbContext, _verb: &str, _remaining: &str) -> UResult<()> {
        ctx.trans.queue_for_session(ctx.session,
                                    Some(ansi!("<red>Bye!<reset>\r\n"))).await?;
        ctx.trans.queue_for_session(ctx.session, None).await?;
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
