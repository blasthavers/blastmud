use super::{TaskHandler, TaskRunContext};
use async_trait::async_trait;
use std::time;
use chrono::Utc;
use crate::DResult;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use crate::models::task::{
    Task,
    TaskMeta,
    TaskDetails,
};
use crate::message_handler::user_commands::{
    VerbContext,
    CommandHandlingError,
    UResult,
    movement,
    user_error,
    get_user_or_fail
};
use crate::static_content::room::Direction;
use once_cell::sync::OnceCell;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueCommand {
    Movement { direction: Direction },
}
impl QueueCommand {
    pub fn name(&self) -> &'static str {
        use QueueCommand::*;
        match self {
            Movement {..} => "Movement"
        }
    }
}

#[async_trait]
pub trait QueueCommandHandler {
    async fn start_command(&self, ctx: &mut VerbContext<'_>, command: &QueueCommand) -> UResult<time::Duration>;
    async fn finish_command(&self, ctx: &mut VerbContext<'_>, command: &QueueCommand) -> UResult<()>;
}

fn queue_command_registry() -> &'static BTreeMap<&'static str, &'static (dyn QueueCommandHandler + Sync + Send)> {
    static REGISTRY: OnceCell<BTreeMap<&'static str, &'static (dyn QueueCommandHandler + Sync + Send)>> =
        OnceCell::new();
    REGISTRY.get_or_init(|| vec!(
        ("Movement", &movement::QueueHandler as &(dyn QueueCommandHandler + Sync + Send))
    ).into_iter().collect())
}

pub async fn queue_command(ctx: &mut VerbContext<'_>, command: &QueueCommand) -> UResult<()> {
    let was_empty = ctx.session_dat.queue.is_empty();
    let username = get_user_or_fail(ctx)?.username.to_lowercase();
    if ctx.session_dat.queue.len() >= 20 {
        user_error("Can't queue more than 20 actions\n".to_owned())?;
    }
    ctx.session_dat.queue.push_back(command.clone());
    if was_empty {
        match queue_command_registry()
            .get(&command.name())
            .expect("QueueCommand to have been registered")
            .start_command(ctx, &command).await {
                Err(CommandHandlingError::UserError(err_msg)) => {
                    ctx.session_dat.queue.truncate(0);
                    ctx.trans.save_session_model(ctx.session, ctx.session_dat).await?;
                    user_error(err_msg)?;
                }
                Err(CommandHandlingError::SystemError(e)) => Err(e)?,
                Ok(dur) => {
                    ctx.trans.save_session_model(ctx.session, ctx.session_dat).await?;
                    ctx.trans.upsert_task(&Task {
                        meta: TaskMeta {
                            task_code: username,
                            next_scheduled: Utc::now() + chrono::Duration::from_std(dur)?,
                            ..Default::default()
                        },
                        details: TaskDetails::RunQueuedCommand
                    }).await?;
                }
            }
        
    } else {
        ctx.trans.queue_for_session(ctx.session, Some("[queued]\n")).await?;
        ctx.trans.save_session_model(ctx.session, ctx.session_dat).await?;
    }
    Ok(())
}

pub struct RunQueuedCommandTaskHandler;
#[async_trait]
impl TaskHandler for RunQueuedCommandTaskHandler {
    async fn do_task(&self, ctx: &mut TaskRunContext) -> DResult<Option<time::Duration>> {
        let username: &str = ctx.task.meta.task_code.as_str();
        let (listener_sess, mut sess_dets) =
            match ctx.trans.find_session_for_player(username).await? {
                None => {
                    // Queue is gone if session is gone, and don't schedule another
                    // job, but otherwise this is a successful run.
                    return Ok(None);
                },
                Some(x) => x
            };
        let queue_command = match sess_dets.queue.pop_front() {
            None => { return Ok(None); }
            Some(x) => x
        };
        let mut user = ctx.trans.find_by_username(username).await?;
        let mut verbcontext = VerbContext {
            session: &listener_sess,
            session_dat: &mut sess_dets,
            user_dat: &mut user,
            trans: ctx.trans
        };
        let uresult_finish =
            queue_command_registry()
            .get(&queue_command.name())
            .expect("QueueCommand to have been registered")
            .finish_command(&mut verbcontext, &queue_command).await;
        match uresult_finish {
            Ok(()) => {}
            Err(CommandHandlingError::UserError(err_msg)) => {
                ctx.trans.queue_for_session(&listener_sess, Some(&(err_msg + "\r\n"))).await?;
                sess_dets.queue.truncate(0);
                ctx.trans.save_session_model(&listener_sess, &sess_dets).await?;
                return Ok(None);
            }
            Err(CommandHandlingError::SystemError(e)) => Err(e)?
        };

        let next_command_opt = verbcontext.session_dat.queue.front().cloned();
        let result = match next_command_opt {
            None => None,
            Some(next_command) => {
                match queue_command_registry()
                    .get(&next_command.name())
                    .expect("QueueCommand to have been registered")
                    .start_command(&mut verbcontext, &next_command).await {
                        Err(CommandHandlingError::UserError(err_msg)) => {
                            ctx.trans.queue_for_session(&listener_sess, Some(&(err_msg + "\r\n"))).await?;
                            sess_dets.queue.truncate(0);
                            ctx.trans.save_session_model(&listener_sess, &sess_dets).await?;
                            None
                        }
                        Err(CommandHandlingError::SystemError(e)) => Err(e)?,
                        Ok(dur) => Some(dur)
                    }
            }
        };
        ctx.trans.save_session_model(&listener_sess, &sess_dets).await?;

        Ok(result)
    }
}

pub static HANDLER: &'static (dyn TaskHandler + Sync + Send) = &RunQueuedCommandTaskHandler;
