use super::{VerbContext, UserVerb, UserVerbRef, UResult, user_error};
use super::look;
use async_trait::async_trait;
use tokio::time;

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let (username, password) = match remaining.split_whitespace().collect::<Vec<&str>>()[..] {
            [] => user_error("Too few options to login".to_owned())?,
            [username, password] => (username, password),
            _ => user_error("Too many options to login".to_owned())?,
        };

        match ctx.trans.find_by_username(username).await? {
            None => user_error("No such user.".to_owned())?,
            Some(user) => {
                time::sleep(time::Duration::from_secs(5)).await;
                if !bcrypt::verify(password, &user.password_hash)? {
                    user_error("Invalid password.".to_owned())?
                }
                *ctx.user_dat = Some(user);
            }
        }

        ctx.trans.attach_user_to_session(username, ctx.session).await?;
        super::agree::check_and_notify_accepts(ctx).await?;
        if let Some(user) = ctx.user_dat {
            ctx.trans.save_user_model(user).await?;
            look::VERB.handle(ctx, "look", "").await?;
        }
        
        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
