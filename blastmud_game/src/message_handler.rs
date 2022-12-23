use blastmud_interfaces::*;
use crate::db;
use MessageFromListener::*;
use uuid::Uuid;
use crate::DResult;

#[derive(Clone,Debug)]
pub struct ListenerSession {
    pub listener: Uuid,
    pub session: Uuid
}

pub async fn handle(listener: Uuid, msg: MessageFromListener, pool: db::DBPool)
                    -> DResult<()> {
    match msg {
        ListenerPing { .. } => { pool.record_listener_ping(listener).await?; }
        SessionConnected { session, source: _ } => {
            pool.start_session(ListenerSession { listener, session }).await?;
        }
        SessionDisconnected { session } => {
            pool.end_session(ListenerSession { listener, session }).await?;
        }
        SessionSentLine { session, msg } => {
            pool.queue_for_session(&ListenerSession { listener, session },
                                   &format!("You hear an echo saying: \x1b[31m{}\x1b[0m\r\n", msg)).await?;
        }
        AcknowledgeMessage => {}
    }
    Ok(())
}
