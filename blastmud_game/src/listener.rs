use std::error::Error;
use tokio::task;
use tokio::net::{TcpStream, TcpListener};
use log::{info, warn};
use tokio_util::codec;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;
use tokio_serde::formats::Cbor;
use blastmud_interfaces::*;
use futures::prelude::*;
use tokio::sync::{Mutex, mpsc, oneshot};
use std::sync::Arc;
use uuid::Uuid;
use std::collections::BTreeMap;

pub struct ListenerSend {
    message: MessageToListener,
    ack_notify: oneshot::Sender<()>
}
pub type ListenerMap = Arc<Mutex<BTreeMap<Uuid, mpsc::Sender<ListenerSend>>>>;
    
async fn handle_from_listener<FHandler, HandlerFut>(
    conn: TcpStream,
    message_handler: FHandler,
    listener_map: ListenerMap)
where
    FHandler: Fn(MessageFromListener) -> HandlerFut + Send + 'static,
    HandlerFut: Future<Output = ()> + Send + 'static {
    let mut conn_framed = tokio_serde::Framed::new(
        codec::Framed::new(conn, LengthDelimitedCodec::new()),
        Cbor::<MessageFromListener, MessageToListener>::default()
    );

    let session = match conn_framed.try_next().await {
        Ok(Some(MessageFromListener::ListenerPing { uuid })) => uuid,
        Ok(Some(msg)) => {
            warn!("Got non-ping first message from listener: {:?}", msg);
            return;
        }
        Ok(None) => {
            warn!("Lost listener connection before first message");
            return;
        }
        Err(e) => {
            warn!("Lost listener connection to error {} before first message", e);
            return;
        }
    };

    match conn_framed.send(MessageToListener::AcknowledgeMessage).await {
        Ok(_) => {}
        Err(e) => {
            warn!("Got error sending listener acknowledge for initial ping: {}", e);
            return;
        }
    }
    
    let (sender, mut receiver) = mpsc::channel(1);
    listener_map.lock().await.insert(session, sender);
    
    'listener_loop: loop {
        tokio::select!(
            req = conn_framed.try_next() => {
                match req {
                    Ok(Some(MessageFromListener::AcknowledgeMessage)) => {
                        warn!("Unexpected acknowledge from listener - bug in listener?");
                    }
                    Ok(Some(msg)) => {
                        let handle_fut = message_handler(msg);
                        handle_fut.await;
                        match conn_framed.send(
                            MessageToListener::AcknowledgeMessage
                        ).await {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Got error sending listener acknowledge: {}", e);
                                break 'listener_loop;
                            }
                        }
                    }
                    Ok(None) => {
                        warn!("Lost connection to listener {} due to end-of-stream",
                              session);
                        break 'listener_loop;
                    }
                    Err(e) => {
                        warn!("Lost connection to listener {} due to error {}",
                              session, e);
                        break 'listener_loop;
                    }
                }
            }
            Some(ListenerSend { message, ack_notify }) = receiver.recv() => {
                match conn_framed.send(message).await {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("Got error sending listener command: {}", e);
                        break 'listener_loop;
                    }
                }
                // Cut-back loop to wait for acknowledge.
                'ack_wait_loop: loop {
                    match conn_framed.try_next().await {
                        Ok(Some(MessageFromListener::AcknowledgeMessage)) => {
                            ack_notify.send(()).unwrap_or(());
                            break 'ack_wait_loop;
                        }
                        Ok(Some(msg)) => {
                            let handle_fut = message_handler(msg);
                            handle_fut.await;
                            match conn_framed.send(
                                MessageToListener::AcknowledgeMessage
                            ).await {
                                Ok(_) => {}
                                Err(e) => {
                                    warn!("Got error sending listener acknowledge: {}", e);
                                    break 'listener_loop;
                                }
                            }
                        }
                        Ok(None) => {
                            warn!("Lost connection to listener {} due to end-of-stream",
                                  session);
                            break 'listener_loop;
                        }
                        Err(e) => {
                            warn!("Lost connection to listener {} due to error {}",
                                  session, e);
                            break 'listener_loop;
                        }
                    }
                }
            }
        );
    }

    listener_map.lock().await.remove(&session);
}

pub fn make_listener_map() -> ListenerMap {
    Arc::new(Mutex::new(BTreeMap::new()))
}

pub async fn start_listener<FHandler, HandlerFut>(
    bind_to: String,
    listener_map: ListenerMap,
    handle_message: FHandler
) -> Result<(), Box<dyn Error>>
where
    FHandler: Fn(MessageFromListener) -> HandlerFut + Send + Clone + 'static,
    HandlerFut: Future<Output = ()> + Send + 'static
{
    info!("Starting listener on {}", bind_to);
    let listener = TcpListener::bind(bind_to).await?;

    let listener_map_for_task = listener_map.clone();
    task::spawn(async move {
        loop {
            match listener.accept().await {
                Err(e) => {
                    warn!("Error accepting from listener process: {}", e);
                }
                Ok((socket, _)) => {
                    info!("Accepted new inbound connection from listener");
                    task::spawn(handle_from_listener(socket, handle_message.clone(), listener_map_for_task.clone()));
                }
            }
        }
    });
    
    Ok(())
}
