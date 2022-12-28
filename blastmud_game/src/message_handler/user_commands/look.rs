use super::{VerbContext, UserVerb, UserVerbRef, UResult, UserError};
use async_trait::async_trait;

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, _remaining: &str) -> UResult<()> {
        let user = ctx.user_dat.as_ref()
            .ok_or_else(|| UserError("Not logged in".to_owned()))?;
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
