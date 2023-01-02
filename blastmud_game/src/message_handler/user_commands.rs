use super::ListenerSession;
use crate::DResult;
use crate::db::{DBTrans, DBPool, ItemSearchParams};
use ansi::ansi;
use phf::phf_map;
use async_trait::async_trait;
use crate::models::{session::Session, user::User, item::Item};
use log::warn;
use once_cell::sync::OnceCell;
use std::sync::Arc;

mod agree;
mod describe;
mod help;
mod ignore;
mod less_explicit_mode;
mod login;
mod look;
pub mod movement;
pub mod parsing;
mod quit;
mod register;
mod whisper;

pub struct VerbContext<'l> {
    pub session: &'l ListenerSession,
    pub session_dat: &'l mut Session,
    pub user_dat: &'l mut Option<User>,
    pub trans: &'l DBTrans
}

pub enum CommandHandlingError {
    UserError(String),
    SystemError(Box<dyn std::error::Error + Send + Sync>)
}
use CommandHandlingError::*;

#[async_trait]
pub trait UserVerb {
    async fn handle(self: &Self, ctx: &mut VerbContext, verb: &str, remaining: &str) -> UResult<()>;
}

pub type UResult<A> = Result<A, CommandHandlingError>;


impl<T> From<T> for CommandHandlingError where T: Into<Box<dyn std::error::Error + Send + Sync>> {
    fn from(input: T) -> CommandHandlingError {
        SystemError(input.into())
    }
}

pub fn user_error<A>(msg: String) -> UResult<A> {
    Err(UserError(msg))
}


/* Verb registries list types of commands available in different circumstances. */
pub type UserVerbRef = &'static (dyn UserVerb + Sync + Send);
type UserVerbRegistry = phf::Map<&'static str, UserVerbRef>;

static ALWAYS_AVAILABLE_COMMANDS: UserVerbRegistry = phf_map! {
    "" => ignore::VERB,
    "help" => help::VERB,
    "quit" => quit::VERB,
};

static UNREGISTERED_COMMANDS: UserVerbRegistry = phf_map! {
    "agree" => agree::VERB,
    "connect" => login::VERB,
    "less_explicit_mode" => less_explicit_mode::VERB,
    "login" => login::VERB,
    "register" => register::VERB,
};

static REGISTERED_COMMANDS: UserVerbRegistry = phf_map! {
    // Movement comments first:
    "north" => movement::VERB,
    "n" => movement::VERB,
    "northeast" => movement::VERB,
    "ne" => movement::VERB,
    "east" => movement::VERB,
    "e" => movement::VERB,
    "southeast" => movement::VERB,
    "se" => movement::VERB,
    "south" => movement::VERB,
    "s" => movement::VERB,
    "southwest" => movement::VERB,
    "sw" => movement::VERB,
    "west" => movement::VERB,
    "w" => movement::VERB,
    "northwest" => movement::VERB,
    "nw" => movement::VERB,
    "up" => movement::VERB,
    "down" => movement::VERB,
    
    // Other commands (alphabetical except aliases grouped):
    "describe" => describe::VERB,
    "l" => look::VERB,
    "look" => look::VERB,
    "read" => look::VERB,
    "-" => whisper::VERB,
    "whisper" => whisper::VERB,
};

fn resolve_handler(ctx: &VerbContext, cmd: &str) -> Option<&'static UserVerbRef> {
    let mut result = ALWAYS_AVAILABLE_COMMANDS.get(cmd);

    match &ctx.user_dat {
        None => {
            result = result.or_else(|| UNREGISTERED_COMMANDS.get(cmd));
        }
        Some(user_dat) => {
            if user_dat.terms.terms_complete {
                result = result.or_else(|| REGISTERED_COMMANDS.get(cmd));
            } else if cmd == "agree" {
                result = Some(&agree::VERB);
            }
        }
    }

    result
}

pub async fn handle(session: &ListenerSession, msg: &str, pool: &DBPool) -> DResult<()> {
    let (cmd, params) = parsing::parse_command_name(msg);
    let trans = pool.start_transaction().await?;
    let (mut session_dat, mut user_dat) = match trans.get_session_user_model(session).await? {
        None => {
            // If the session has been cleaned up from the database, there is
            // nowhere to go from here, so just ignore it.
            warn!("Got command from session not in database: {}", session.session);
            return Ok(());
        }
        Some(v) => v
    };
  
    let mut ctx = VerbContext { session, trans: &trans, session_dat: &mut session_dat,
                                user_dat: &mut user_dat };
    let handler_opt = resolve_handler(&ctx, cmd);
    
    match handler_opt {
        None => {
            trans.queue_for_session(session,
                                    Some(ansi!(
                                        "That's not a command I know. Try <bold>help<reset>\r\n"
                                    ))
            ).await?;
            trans.commit().await?;
        }
        Some(handler) => {
            match handler.handle(&mut ctx, cmd, params).await {
                Ok(()) => {
                    trans.commit().await?;
                }
                Err(UserError(err_msg)) => {
                    pool.queue_for_session(session, Some(&(err_msg + "\r\n"))).await?;
                }
                Err(SystemError(e)) => Err(e)?
            }
        }
    }
    Ok(())
}

pub fn is_likely_explicit(msg: &str) -> bool {
    static EXPLICIT_MARKER_WORDS: OnceCell<Vec<&'static str>> =
        OnceCell::new();
    let markers = EXPLICIT_MARKER_WORDS.get_or_init(||
        vec!("fuck", "sex", "cock", "cunt", "dick", "pussy", "whore",
             "orgasm", "erection", "nipple", "boob", "tit"));
    for word in markers {
        if msg.contains(word) {
            return true
        }
    }
    false
}

pub fn get_user_or_fail<'l>(ctx: &'l VerbContext) -> UResult<&'l User> { 
    ctx.user_dat.as_ref()
        .ok_or_else(|| UserError("Not logged in".to_owned()))
}

pub fn get_user_or_fail_mut<'l>(ctx: &'l mut VerbContext) -> UResult<&'l mut User> { 
    ctx.user_dat.as_mut()
        .ok_or_else(|| UserError("Not logged in".to_owned()))
}

pub async fn get_player_item_or_fail(ctx: &VerbContext<'_>) -> UResult<Arc<Item>> {
    Ok(ctx.trans.find_item_by_type_code(
        "player", &get_user_or_fail(ctx)?.username.to_lowercase()).await?
       .ok_or_else(|| UserError("Your character is gone, you'll need to re-register or ask an admin".to_owned()))?)
}

pub async fn search_item_for_user<'l>(ctx: &'l VerbContext<'l>, search: &'l ItemSearchParams<'l>) ->
    UResult<Arc<Item>> {
        Ok(match &ctx.trans.resolve_items_by_display_name_for_player(search).await?[..] {
            [] => user_error("Sorry, I couldn't find anything matching.".to_owned())?,
            [match_it] => match_it.clone(),
            [item1, ..] =>
                item1.clone(),
        })
}
