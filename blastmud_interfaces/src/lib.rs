use uuid::Uuid;
use serde::*;
    
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageFromListener {
    ListenerPing { uuid: Uuid },
    SessionConnected { session: Uuid, source: String },
    SessionDisconnected { session: Uuid },
    SessionSentLine { session: Uuid, msg: String },
    AcknowledgeMessage
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageToListener {
    DisconnectSession { session: Uuid },
    SendToSession { session: Uuid, msg: String },
    AcknowledgeMessage
}
