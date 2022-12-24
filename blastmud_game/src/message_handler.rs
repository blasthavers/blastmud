use blastmud_interfaces::*;
use crate::db;
use MessageFromListener::*;
use uuid::Uuid;
use crate::DResult;

mod new_session;
mod user_commands;

#[derive(Clone,Debug)]
pub struct ListenerSession {
    pub listener: Uuid,
    pub session: Uuid
}

pub async fn handle(listener: Uuid, msg: MessageFromListener, pool: db::DBPool)
                    -> DResult<()> {
    match msg {
        ListenerPing { .. } => { pool.record_listener_ping(listener).await?; }
        SessionConnected { session, source } => {
            new_session::handle(&ListenerSession { listener, session }, &source, pool).await?;
        }
        SessionDisconnected { session } => {
            pool.end_session(ListenerSession { listener, session }).await?;
        }
        SessionSentLine { session, msg } => {
            user_commands::handle(&ListenerSession { listener, session }, &msg, pool).await?;
        }
        AcknowledgeMessage => {}
    }
    Ok(())
}
