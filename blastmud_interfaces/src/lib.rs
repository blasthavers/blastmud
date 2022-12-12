use uuid::Uuid;
use serde::*;
    
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageFromListener {
    ListenerPing { uuid: Uuid },
    UserConnected { user: Uuid, source: String },
    UserDisconnected { user: Uuid },
    UserSentLine { user: Uuid, msg: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageToListener {
    DisconnectUser { user: Uuid },
    SendToUser { user: Uuid, msg: String },
}
