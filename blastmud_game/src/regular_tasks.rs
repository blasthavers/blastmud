use tokio::{task, time, sync::oneshot};
use async_trait::async_trait;
use crate::{DResult, db, models::task::{Task, TaskParse, TaskRecurrence}};
use crate::listener::{ListenerMap, ListenerSend};
use blastmud_interfaces::MessageToListener;
use log::warn;
use once_cell::sync::OnceCell;
use std::ops::AddAssign;
use std::collections::BTreeMap;
use chrono::Utc;
use crate::static_content::npc;

pub mod queued_command;

pub struct TaskRunContext<'l> {
    pub trans: &'l db::DBTrans,
    pub task: &'l mut Task
}

#[async_trait]
pub trait TaskHandler {
    async fn do_task(&self, ctx: &mut TaskRunContext) -> DResult<Option<time::Duration>>;
}

fn task_handler_registry() -> &'static BTreeMap<&'static str, &'static (dyn TaskHandler + Sync + Send)> {
    static TASK_HANDLER_REGISTRY: OnceCell<BTreeMap<&'static str, &'static (dyn TaskHandler + Sync + Send)>> =
        OnceCell::new();
    TASK_HANDLER_REGISTRY.get_or_init(
        || vec!(
            ("RunQueuedCommand", queued_command::HANDLER.clone()), 
            ("NPCSay", npc::SAY_HANDLER.clone()),
        ).into_iter().collect()
    )
}

async fn cleanup_session_once(pool: db::DBPool) -> DResult<()> {
    for listener in pool.get_dead_listeners().await? {
        pool.cleanup_listener(listener).await?;
    }
    Ok(())
}

fn start_session_cleanup_task(pool: db::DBPool) {
    task::spawn(async move {
        loop {
            time::sleep(time::Duration::from_secs(60)).await;
            match cleanup_session_once(pool.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("Error cleaning up sessions: {}", e);
                }
            }
        }
    });
}

async fn process_sendqueue_once(pool: db::DBPool, listener_map: ListenerMap) -> DResult<()> {
    loop {
        let q = pool.get_from_sendqueue().await?;
        for item in &q {
            match listener_map.lock().await.get(&item.session.listener).map(|l| l.clone()) {
                None => {}
                Some(listener_sender) => {
                    let (tx, rx) = oneshot::channel();
                    listener_sender.send(
                        ListenerSend {
                            message: match item.message.clone() {
                                None => MessageToListener::DisconnectSession {
                                    session: item.session.session.clone()
                                },
                                Some(msg) => MessageToListener::SendToSession {
                                    session: item.session.session.clone(),
                                    msg: msg
                                }
                            },
                            ack_notify: tx
                        }
                    ).await.unwrap_or(());
                    rx.await.unwrap_or(());
                    pool.delete_from_sendqueue(&item).await?;
                }
            }
        }
        if q.len() <= 9 {
            break;
        }
    }
    Ok(())
}

fn start_send_queue_task(pool: db::DBPool, listener_map: ListenerMap) {
    task::spawn(async move {
        loop {
            time::sleep(time::Duration::from_millis(500)).await;
            match process_sendqueue_once(pool.clone(), listener_map.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("Error processing sendqueue: {}", e);
                }
            }
        }
    });
}

async fn process_tasks_once(pool: db::DBPool) -> DResult<()> {
    loop {
        let tx = pool.start_transaction().await?;
        match tx.get_next_scheduled_task().await? {
            None => { break; }
            Some(task_parse) => {
                match task_parse {
                    TaskParse::Known(mut task) => {
                        match task_handler_registry().get(task.details.name()) {
                            None => {
                                warn!("Found a known but unregistered task type: {}",
                                      task.details.name());
                                // This is always a logic error, so just delete the task
                                // to help with recovery.
                                tx.delete_task(&task.details.name(), &task.meta.task_code).await?;
                                tx.commit().await?;
                            }
                            Some(handler) => {
                                let mut ctx = TaskRunContext { trans: &tx, task: &mut task };
                                match handler.do_task(&mut ctx).await {
                                    Err(e) => {
                                        task.meta.consecutive_failure_count += 1;
                                        warn!("Error handling event of type {} code {} (consecutive count: {}): {:?}",
                                              &task.details.name(), &task.meta.task_code,
                                              task.meta.consecutive_failure_count, e);
                                        if task.meta.consecutive_failure_count > 3 && !task.meta.is_static {
                                            tx.delete_task(&task.details.name(), &task.meta.task_code).await?;
                                        } else {
                                            task.meta.next_scheduled = Utc::now() + chrono::Duration::seconds(60);
                                            tx.update_task(&task.details.name(), &task.meta.task_code,
                                                           &TaskParse::Known(task.clone())).await?;
                                        }
                                        tx.commit().await?;
                                    },
                                    Ok(resched) => {
                                        task.meta.consecutive_failure_count = 0;
                                        match task.meta.recurrence.clone().or(
                                            resched.map(|r| TaskRecurrence::FixedDuration { seconds: r.as_secs() as u32 })) {
                                            None => {
                                                tx.delete_task(&task.details.name(),
                                                               &task.meta.task_code).await?;
                                            }
                                            Some(TaskRecurrence::FixedDuration { seconds }) => {
                                                task.meta.next_scheduled = Utc::now() +
                                                    chrono::Duration::seconds(seconds as i64);
                                                tx.update_task(&task.details.name(), &task.meta.task_code,
                                                               &TaskParse::Known(task.clone())).await?;
                                            }
                                        }
                                        tx.commit().await?;
                                    }
                                }
                            }
                        }
                    }
                    TaskParse::Unknown(mut task) => {
                        warn!("Found unknown task type: {}, code: {}",
                              &task.task_type, &task.meta.task_code);
                        if task.meta.is_static {
                            // Probably a new (or newly removed) static type.
                            // We just skip this tick of it.
                            match task.meta.recurrence {
                                None => {
                                    tx.delete_task(&task.task_type, &task.meta.task_code).await?;
                                    tx.commit().await?;
                                }
                                Some(TaskRecurrence::FixedDuration { seconds }) => {
                                    task.meta.next_scheduled.add_assign(
                                        chrono::Duration::seconds(seconds as i64)
                                    );
                                    tx.update_task(&task.task_type, &task.meta.task_code,
                                                   &TaskParse::Unknown(task.clone())).await?;
                                }
                            }
                        } else {
                            tx.delete_task(&task.task_type, &task.meta.task_code).await?;
                            tx.commit().await?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn start_task_runner(pool: db::DBPool) {
    task::spawn(async move {
        loop {
            time::sleep(time::Duration::from_millis(500)).await;
            match process_tasks_once(pool.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("Error processing tasks: {}", e);
                }
            }
        }
    });
}

pub fn start_regular_tasks(pool: &db::DBPool, listener_map: ListenerMap) -> DResult<()> {
    start_session_cleanup_task(pool.clone());
    start_send_queue_task(pool.clone(), listener_map);
    start_task_runner(pool.clone());
    Ok(())
}
