use tokio::{task, time, sync::oneshot};
use crate::DResult;
use crate::db;
use crate::listener::{ListenerMap, ListenerSend};
use blastmud_interfaces::MessageToListener;
use log::warn;

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
            time::sleep(time::Duration::from_secs(1)).await;
            match process_sendqueue_once(pool.clone(), listener_map.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("Error processing sendqueue: {}", e);
                }
            }
        }
    });
}

pub fn start_regular_tasks(pool: &db::DBPool, listener_map: ListenerMap) -> DResult<()> {
    start_session_cleanup_task(pool.clone());
    start_send_queue_task(pool.clone(), listener_map);
    Ok(())
}
