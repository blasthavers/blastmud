use log::info;
use blastmud_interfaces::*;
use deadpool_postgres::Pool;
use crate::listener::ListenerMap;

pub async fn handle(msg: MessageFromListener, _pool: Pool, _listener_map: ListenerMap) {
    info!("Processing message: {:?}", msg)
}
