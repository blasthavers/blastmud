use blastmud_interfaces::*;
use deadpool_postgres::Pool;
use crate::listener::ListenerMap;
use MessageFromListener::*;
use uuid::Uuid;
use tokio::{sync::oneshot, task};
use crate::listener::ListenerSend;
use std::error::Error;

pub async fn handle(listener: Uuid, msg: MessageFromListener, _pool: Pool, listener_map: ListenerMap)
  -> Result<(), Box<dyn Error>> {
    match msg {
        ListenerPing { uuid: _ } => {}
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
