use tokio::{task, time};
use tokio::net::{TcpSocket, TcpStream, lookup_host};
use log::{info, warn};
use tokio_util::codec;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;
use tokio_serde::formats::Cbor;
use blastmud_interfaces::*;
use futures::prelude::*;
use tokio::sync::{Mutex, mpsc, oneshot};
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;
use std::collections::BTreeMap;
use crate::DResult;
use std::time::Instant;

#[derive(Debug)]
pub struct ListenerSend {
    pub message: MessageToListener,
    pub ack_notify: oneshot::Sender<()>
}
pub type ListenerMap = Arc<Mutex<BTreeMap<Uuid, mpsc::Sender<ListenerSend>>>>;

async fn handle_from_listener<FHandler, HandlerFut>(
    conn: TcpStream,
    message_handler: FHandler,
    listener_map: ListenerMap)
where
    FHandler: Fn(Uuid, MessageFromListener) -> HandlerFut + Send + 'static,
    HandlerFut: Future<Output = DResult<()>> + Send + 'static {
    let mut conn_framed = tokio_serde::Framed::new(
        codec::Framed::new(conn, LengthDelimitedCodec::new()),
        Cbor::<MessageFromListener, MessageToListener>::default()
    );

    let listener_id = match conn_framed.try_next().await {
        Ok(Some(ref msg@MessageFromListener::ListenerPing { uuid })) => {
            let handle_fut = message_handler(uuid.clone(), msg.clone());
            match handle_fut.await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Error processing initial ListenerPing: {}", e);
                }
            };
            uuid
        },
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

    let connected_at = Instant::now();
    let (sender, mut receiver) = mpsc::channel(1);
    listener_map.lock().await.insert(listener_id, sender);
    
    'listener_loop: loop {
        tokio::select!(
            req = conn_framed.try_next() => {
                match req {
                    Ok(Some(MessageFromListener::AcknowledgeMessage)) => {
                        warn!("Unexpected acknowledge from listener - bug in listener?");
                    }
                    Ok(Some(msg)) => {
                        let handle_fut = message_handler(listener_id, msg);
                        match handle_fut.await {
                            Ok(_) => {}
                            Err(e) => {
                                // On the assumption errors that get here are bad enough that they are a
                                // problem with the system rather than the message, so we want to log and
                                // retry later.
                                warn!("Error from message handler - closing listener connection: {}", e);
                                break 'listener_loop;
                            }
                        }
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
                              listener_id);
                        break 'listener_loop;
                    }
                    Err(e) => {
                        warn!("Lost connection to listener {} due to error {}",
                              listener_id, e);
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
                            let handle_fut = message_handler(listener_id, msg);
                            match handle_fut.await {
                                Ok(_) => {}
                                Err(e) => {
                                    if connected_at.elapsed() > std::time::Duration::from_secs(60) {
                                        // On the assumption errors that get here are bad enough that they are a
                                        // problem with the system rather than the message, so we want to log and
                                        // retry later.
                                        warn!("Error from message handler - closing listener connection: {}", e);
                                        break 'listener_loop;
                                    } else {
                                        warn!("Error from message handler, but we only just connected, so \
                                               acknowledging it anyway as a safety measure against reconnect \
                                               loops: {}", e);
                                    }
                                }
                            }

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
                                  listener_id);
                            break 'listener_loop;
                        }
                        Err(e) => {
                            warn!("Lost connection to listener {} due to error {}",
                                  listener_id, e);
                            break 'listener_loop;
                        }
                    }
                }
            }
        );
    }

    // We delay to avoid wasting resources if we do end up in a loop.
    time::sleep(time::Duration::from_secs(1)).await;    
    listener_map.lock().await.remove(&listener_id);
}

pub fn make_listener_map() -> ListenerMap {
    Arc::new(Mutex::new(BTreeMap::new()))
}

pub async fn start_listener<FHandler, HandlerFut>(
    bind_to: String,
    listener_map: ListenerMap,
    handle_message: FHandler
) -> DResult<()>
where
    FHandler: Fn(Uuid, MessageFromListener) -> HandlerFut + Send + Clone + 'static,
    HandlerFut: Future<Output = DResult<()>> + Send + 'static
{
    info!("Starting listener on {}", bind_to);
    let addr = lookup_host(bind_to).await?.next().expect("listener address didn't resolve");
    let socket = match addr {
        SocketAddr::V4 {..} => TcpSocket::new_v4()?,
        SocketAddr::V6 {..} => TcpSocket::new_v6()?
    };
    socket.set_reuseaddr(true)?;
    socket.set_reuseport(true)?;
    socket.bind(addr)?;
    let listener = socket.listen(5)?;
    
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
