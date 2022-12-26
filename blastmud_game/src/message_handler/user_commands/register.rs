use super::{VerbContext, UserVerb, UserVerbRef, UResult};
use async_trait::async_trait;
use super::{user_error, parsing::parse_username};
use crate::models::{user::User, item::Item};
use chrono::Utc;
use ansi_macro::ansi;

pub struct Verb;
#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, _verb: &str, remaining: &str) -> UResult<()> {
        let (username, mut password) = match parse_username(remaining) {
            Err(e) => user_error("Invalid username: ".to_owned() + e)?,
            Ok(r) => r
        };
        password = password.trim();
        if ctx.trans.find_by_username(username).await?.is_some() {
            user_error("Username already exists".to_owned())?;
        }
        if password.contains(" ") || password.contains("\t") {
            user_error("To avoid future confusion, password can't contain spaces / tabs".to_owned())?;
        } else if password.len() < 6 {
            user_error("Password must be 6 characters long or longer".to_owned())?;
        }
        
        let player_item_id = ctx.trans.create_item(&Item {
            item_type: "player".to_owned(),
            item_code: username.to_lowercase(),
            display: username.to_owned(),
            location: "room/chargen_room".to_owned(),
            ..Item::default()
        }).await?;
        let password_hash = bcrypt::hash(password, 10).expect("hash not to fail");
        let user_dat = User {
            username: username.to_owned(),
            password_hash: password_hash.to_owned(),
            player_item_id,
            registered_at: Some(Utc::now()),
            ..User::default()
        };
        *ctx.user_dat = Some(user_dat);

        ctx.trans.queue_for_session(
            ctx.session,
            Some(&format!(ansi!("Welcome <bold>{}<reset>, you are now officially registered.\r\n"),
                         &username))
        ).await?;        
        super::agree::check_and_notify_accepts(ctx).await?;
        ctx.trans.create_user(ctx.session, ctx.user_dat.as_ref().unwrap()).await?;

        Ok(())
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
