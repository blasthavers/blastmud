use super::{VerbContext, UserVerb, UserVerbRef, UResult};
use async_trait::async_trait;
use super::{user_error, parsing::parse_username};

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, _ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let (username, password) = match parse_username(remaining) {
            Err(e) => user_error("Invalid username: ".to_owned() + e)?,
            Ok(r) => r
        };
        let _pwhash = bcrypt::hash(password, 10);
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
