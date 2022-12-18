use blastmud_interfaces::*;
use crate::listener::ListenerMap;
use crate::db;
use MessageFromListener::*;
use uuid::Uuid;
use tokio::{sync::oneshot, task};
use crate::listener::ListenerSend;
use crate::DResult;

pub async fn handle(listener: Uuid, msg: MessageFromListener, pool: db::DBPool, listener_map: ListenerMap)
                    -> DResult<()> {
    match msg {
        ListenerPing { .. } => { db::record_listener_ping(listener, pool).await?; }
        SessionConnected { session: _, source: _ } => {}
        SessionDisconnected { session: _ } => {}
        SessionSentLine { session, msg } => {
            let lmlock = listener_map.lock().await;
            let opt_sender = lmlock.get(&listener).map(|v| v.clone());
            drop(lmlock);
            match opt_sender {
                None => {}
                Some(sender) => {
                    task::spawn(async move {
                        let (tx, rx) = oneshot::channel();
                        sender.send(ListenerSend { message: MessageToListener::SendToSession {
                            session,
                            msg: format!("You hear an echo saying: \x1b[31m{}\x1b[0m\r\n", msg) },
                                                   ack_notify: tx }).await.unwrap_or(());
                        rx.await.unwrap_or(());
                    });
                }
            }
        }
        AcknowledgeMessage => {}
    }
    Ok(())
}
