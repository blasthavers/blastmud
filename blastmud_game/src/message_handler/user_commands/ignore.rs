use super::{VerbContext, UserVerb, UserVerbRef, UResult};
use async_trait::async_trait;

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, _ctx: &VerbContext, _verb: &str, _remaining: &str) -> UResult<()> {
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
