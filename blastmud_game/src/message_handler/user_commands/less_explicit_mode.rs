use super::{
    VerbContext, UserVerb, UserVerbRef, UResult
};
use async_trait::async_trait;

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, _remaining: &str) -> UResult<()> {
        (*ctx.session_dat).less_explicit_mode = true;
        ctx.trans.save_session_model(ctx.session, ctx.session_dat).await?;
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
