use super::ListenerSession;
use crate::DResult;
use crate::db::{DBTrans, DBPool};
use ansi::ansi;
use phf::phf_map;
use async_trait::async_trait;
use crate::models::{session::Session, user::User};
use log::warn;

mod agree;
mod help;
mod ignore;
mod less_explicit_mode;
mod login;
mod look;
mod parsing;
mod quit;
mod register;

pub struct VerbContext<'l> {
    session: &'l ListenerSession,
    session_dat: &'l mut Session,
    user_dat: &'l mut Option<User>,
    trans: &'l DBTrans
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
    "l" => look::VERB,
    "look" => look::VERB,
};

pub fn explicit_if_allowed<'l>(ctx: &VerbContext, explicit: &'l str, non_explicit: Option<&'l str>) -> &'l str {
    if ctx.session_dat.less_explicit_mode {
        non_explicit.unwrap_or(explicit)
    } else {
        explicit
    }
}

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
