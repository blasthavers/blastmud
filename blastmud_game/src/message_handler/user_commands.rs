use super::ListenerSession;
use crate::DResult;
use crate::db::{DBTrans, DBPool};
use ansi_macro::ansi;
use phf::phf_map;
use async_trait::async_trait;
use crate::models::session::Session;
use log::warn;

mod parsing;
mod ignore;
mod help;
mod quit;

pub struct VerbContext<'l> {
    session: &'l ListenerSession,
    session_dat: &'l mut Session,
    trans: &'l DBTrans
}

pub enum CommandHandlingError {
    UserError(String),
    SystemError(Box<dyn std::error::Error + Send + Sync>)
}
use CommandHandlingError::*;

#[async_trait]
pub trait UserVerb {
    async fn handle(self: &Self, ctx: &VerbContext, verb: &str, remaining: &str) -> UResult<()>;
}

pub type UserVerbRef = &'static (dyn UserVerb + Sync + Send);
pub type UResult<A> = Result<A, CommandHandlingError>;

impl From<Box<dyn std::error::Error + Send + Sync>> for CommandHandlingError {
    fn from(input: Box<dyn std::error::Error + Send + Sync>) -> CommandHandlingError {
        SystemError(input)
    }
}

pub fn user_error<A>(msg: String) -> UResult<A> {
    Err(UserError(msg))
}

type UserVerbRegistry = phf::Map<&'static str, UserVerbRef>;

static ALWAYS_AVAILABLE_COMMANDS: UserVerbRegistry = phf_map! {
    "" => ignore::VERB,
    "help" => help::VERB,
    "quit" => quit::VERB,
};

pub async fn handle(session: &ListenerSession, msg: &str, pool: &DBPool) -> DResult<()> {
    let (cmd, params) = parsing::parse_command_name(msg);
    let trans = pool.start_transaction().await?;
    let mut session_dat = match trans.get_session_model(session).await? {
        None => {
            // If the session has been cleaned up from the database, there is
            // nowhere to go from here, so just ignore it.
            warn!("Got command from session not in database: {}", session.session);
            return Ok(());
        }
        Some(v) => v
    };
    let handler_opt = ALWAYS_AVAILABLE_COMMANDS.get(cmd);
    let ctx = VerbContext { session, trans: &trans, session_dat: &mut session_dat };
    
    match handler_opt {
        None => {
            trans.queue_for_session(session,
                                    Some(ansi!(
                                        "That's not a command I know. Try <bold>help<reset>\r\n"
                                    ))
            ).await?;
        }
        Some(handler) => {
            match handler.handle(&ctx, cmd, params).await {
                Ok(()) => {}
                Err(UserError(err_msg)) => {
                    trans.queue_for_session(session, Some(&(err_msg + "\r\n"))).await?;
                }
                Err(SystemError(e)) => Err(e)?
            }
        }
    }
    trans.commit().await?;
    Ok(())
}
