use super::{VerbContext, UserVerb, UserVerbRef, UResult, user_error};
use async_trait::async_trait;

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        user_error("Not implemented yet\r\n".to_string())?;
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
