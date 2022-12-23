use std::vec::Vec;
use std::collections::BTreeMap;
use std::error::Error;
use std::net::SocketAddr;
use std::fs;
use serde::*;
use tokio::task;
use tokio::time::{self, Duration};
use tokio::net::{TcpStream, TcpListener, lookup_host};
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::{mpsc, Mutex};
use tokio::io::{BufReader, AsyncWriteExt};
use log::{warn, info, LevelFilter};
use simple_logger::SimpleLogger;
use std::sync::Arc;
use blastmud_interfaces::*;
use tokio_util::codec;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;
use tokio_serde::formats::Cbor;
use futures::prelude::*;
use uuid::Uuid;
use tokio_stream::wrappers::ReceiverStream;
use warp;
use warp::filters::ws;
use warp::Filter;

#[derive(Deserialize, Debug)]
struct Config {
    listeners: Vec<String>,
    ws_listener: String,
    gameserver: String,
}

type DResult<A> = Result<A, Box<dyn Error + Send + Sync>>;

fn read_latest_config() -> DResult<Config> {
    serde_yaml::from_str(&fs::read_to_string("listener.conf")?).
        map_err(|error| Box::new(error) as Box<dyn Error + Send + Sync>)
}

#[derive(Debug, Clone)]
enum ServerTaskCommand {
    SwitchTo { new_server: String },
    Send { message: MessageFromListener }
}

fn run_server_task<FHandler, HandlerFut>(
    unfinished_business: Option<MessageFromListener>,
    listener_id: Uuid,
    mut receiver: ReceiverStream<ServerTaskCommand>,
    sender: mpsc::Sender<ServerTaskCommand>,
    server: String,
    message_handler: FHandler
)
where
    FHandler: Fn(MessageToListener) -> HandlerFut + Send + 'static,
    HandlerFut: Future<Output = ()> + Send + 'static
{
    task::spawn(async move {
        let conn = loop {
            match TcpStream::connect(&server).await {
                Err(e) => warn!("Can't connect to {}: {}", server, e),
                Ok(c) => break c
            }
            time::sleep(Duration::from_secs(1)).await;
        };
        let mut conn_framed = tokio_serde::Framed::new(
            codec::Framed::new(conn, LengthDelimitedCodec::new()),
            Cbor::<MessageToListener, MessageFromListener>::default()
        );

        let mut commands = stream::iter(vec!(
            ServerTaskCommand::Send {
                message: MessageFromListener::ListenerPing { uuid: listener_id }
            })
        ).chain(
            stream::iter(unfinished_business.map(|message| ServerTaskCommand::Send { message }))
        ).chain(&mut receiver);
        
        'full_select: loop {
            tokio::select!(
                req = conn_framed.try_next() => {
                    match req {
                        Err(e) => {
                            warn!("Got read error from {}: {}", server, e);
                            run_server_task(
                                None,
                                listener_id,
                                receiver,
                                sender,
                                server,
                                message_handler
                            );
                            break 'full_select;
                        }
                        Ok(None) => {
                            warn!("Got connection closed from {}", server);
                            run_server_task(
                                None,
                                listener_id,
                                receiver,
                                sender,
                                server,
                                message_handler
                            );
                            break 'full_select;
                        }
                        Ok(Some(MessageToListener::AcknowledgeMessage)) => {
                            // We do this here to ensure we never ack an ack.
                            warn!("Unexpected AcknowledgeMessage from gameserver. This suggests a bug in the gameserver");
                        }
                        Ok(Some(msg)) => {
                            let mhfut = message_handler(msg);
                            mhfut.await;
                    
                            match conn_framed.send(MessageFromListener::AcknowledgeMessage).await {
                                Ok(_) => {}
                                Err(e) => {
                                    warn!("Can't send acknowledgement to {}: {}", server, e);
                                    run_server_task(None, listener_id, receiver, sender, server,
                                                    message_handler);
                                    break 'full_select;
                                }
                            }
                        }
                    }
                },
                Some(req) = commands.next() => {
                    match req {
                        ServerTaskCommand::Send { message } =>
                            match conn_framed.send(message.clone()).await {
                                Ok(_) => {
                                    // Now we enter a cut-back loop where we don't
                                    // take on any new work until we see an
                                    // acknowledgement.
                                    'wait_for_ack: loop {
                                        match conn_framed.try_next().await {
                                            Err(e) => {
                                                warn!("Can't read acknowledgement from {}: {}", server, e);
                                                run_server_task(
                                                    Some(message),
                                                    listener_id,
                                                    receiver,
                                                    sender,
                                                    server,
                                                    message_handler
                                                );
                                                break 'full_select;
                                            }
                                            Ok(None) => { 
                                                warn!("Got connection closed from {}", server);
                                                run_server_task(
                                                    Some(message),
                                                    listener_id,
                                                    receiver,
                                                    sender,
                                                    server,
                                                    message_handler
                                                );
                                                break 'full_select;
                                           }
                                            Ok(Some(MessageToListener::AcknowledgeMessage)) => {
                                                break 'wait_for_ack;
                                            }
                                            Ok(Some(msg)) => {
                                                let mhfut = message_handler(msg);
                                                mhfut.await;
                                                
                                                match conn_framed.send(MessageFromListener::AcknowledgeMessage).await {
                                                    Ok(_) => {}
                                                    Err(e) => {
                                                        warn!("Can't send acknowledgement to {}: {}", server, e);
                                                        run_server_task(None, listener_id, receiver, sender, server,
                                                                        message_handler);
                                                        break 'full_select;
                                                    }
                                                }
                                            }
                                            
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Can't send message to {}: {}", server, e);
                                    run_server_task(
                                        Some(message),
                                        listener_id,
                                        receiver,
                                        sender,
                                        server,
                                        message_handler
                                    );
                                    break 'full_select;
                                }
                            }
                        ServerTaskCommand::SwitchTo { new_server } => {
                            // It is safe to just hard cutover at this point, because we haven't
                            // processed any messages we haven't acknowledged. The new gameserver
                            // will resend anything queued we didn't acknowledge.
                            info!("Ending connection to server {} due to reload", server);
                            run_server_task(
                                None,
                                listener_id,
                                receiver,
                                sender,
                                new_server,
                                message_handler
                            );
                            break 'full_select;
                        }
                    }
                }
            );
        }
        
    });
    
}

enum SessionCommand {
    Disconnect,
    SendString { message : String }
}

struct SessionRecord {
    channel: mpsc::Sender<SessionCommand>,
    disconnect_channel: mpsc::UnboundedSender<()>
}

type SessionMap = Arc<Mutex<BTreeMap<Uuid, SessionRecord>>>;

async fn handle_server_message(session_map: SessionMap, message: MessageToListener) {
    match message {
        MessageToListener::AcknowledgeMessage => {}
        MessageToListener::DisconnectSession { session } => {
            match session_map.lock().await.get(&session) {
                // Just silently ignore it if they are disconnected.
                None => {}
                Some(SessionRecord { channel, disconnect_channel, .. }) => {
                    match channel.try_send(SessionCommand::Disconnect) {
                        Err(mpsc::error::TrySendError::Full(_)) => {
                            disconnect_channel.send(()).unwrap_or(());
                        }
                        _ => {}
                    }
                }
            }
        }
        MessageToListener::SendToSession { session, msg } => {
            match session_map.lock().await.get(&session) {
                // Just silently ignore it if they are disconnected.
                None => {}
                Some(SessionRecord { channel, .. }) => {
                    channel.try_send(SessionCommand::SendString { message: msg })
                        .unwrap_or(());
                }
            }
        }
    }
}

fn start_server_task(listener_id: Uuid,
                     server: String,
                     session_map: SessionMap) -> mpsc::Sender<ServerTaskCommand> {
    let (sender, receiver) = mpsc::channel(20);
    let receiver_stream = ReceiverStream::new(receiver);
    run_server_task(None, listener_id, receiver_stream, sender.clone(), server,
                    move |msg| handle_server_message(session_map.clone(),
                                              msg) );
    sender
}

const MAX_CAPACITY: usize = 20;
const STOP_READING_CAPACITY: usize = 10;

async fn handle_client_socket(
    server: mpsc::Sender<ServerTaskCommand>,
    active_sessions: SessionMap,    
    mut stream: TcpStream,
    addr: SocketAddr
) {
    let (rstream, mut wstream) = stream.split();
    let mut rbuf = codec::FramedRead::new(
        BufReader::new(rstream),
        codec::LinesCodec::new_with_max_length(512)
    );
    let session = Uuid::new_v4();
    info!("Accepted session {} from {}", session, addr);

    
    let (lsender, mut lreceiver) = mpsc::channel(MAX_CAPACITY);
    let (discon_sender, mut discon_receiver) = mpsc::unbounded_channel();
    
    active_sessions.lock().await.insert(
        session, SessionRecord {
            channel: lsender.clone(),
            disconnect_channel: discon_sender.clone()
        });
    server.send(ServerTaskCommand::Send { message: MessageFromListener::SessionConnected {
        session, source: addr.to_string()
    }}).await.unwrap();
    
    'client_loop: loop {
        tokio::select!(
            Some(()) = discon_receiver.recv() => {
                info!("Client connection {} instructed for immediate disconnect", session);
                break 'client_loop;
            }
            Some(message) = lreceiver.recv() => {
                match message {
                    SessionCommand::Disconnect => {
                        info!("Client connection {} instructed for disconnect", session);
                        break 'client_loop;
                    }
                    SessionCommand::SendString { message } =>
                        match wstream.write_all(message.as_bytes()).await {
                            Err(e) => {
                                info!("Client connection {} got error {}", session, e);
                            }
                            Ok(()) => {}
                        }
                }
            },
            line_read = rbuf.try_next(), if lsender.capacity() > STOP_READING_CAPACITY  => {
                match line_read {
                    Err(e) => {
                        info!("Client connection {} got error {}", session, e);
                        break 'client_loop;
                    }
                    Ok(None) => {
                        info!("Client connection {} closed", session);
                        break 'client_loop;
                    }
                    Ok(Some(msg)) => {
                        server.send(ServerTaskCommand::Send {
                            message: MessageFromListener::SessionSentLine {session, msg }
                        }).await.unwrap();
                    }
                }
            }
        );
    }

    server.send(ServerTaskCommand::Send { message: MessageFromListener::SessionDisconnected {
        session
    }}).await.unwrap();
    active_sessions.lock().await.remove(&session);
}

fn start_pinger(listener: Uuid, server: mpsc::Sender<ServerTaskCommand>) {
    task::spawn(async move {
        loop {
            time::sleep(Duration::from_secs(60)).await;
            server.send(ServerTaskCommand::Send {
                message: MessageFromListener::ListenerPing { uuid: listener }
            }).await.unwrap();
        }
    });
}

async fn handle_websocket(
    mut ws: ws::WebSocket,
    src: String,
    active_sessions: SessionMap,
    server: mpsc::Sender<ServerTaskCommand>
) {
    let session = Uuid::new_v4();
    info!("Accepted websocket session {} with forwarded-for {}", session, src);

    let (lsender, mut lreceiver) = mpsc::channel(MAX_CAPACITY);
    let (discon_sender, mut discon_receiver) = mpsc::unbounded_channel();
    
    active_sessions.lock().await.insert(
        session, SessionRecord {
            channel: lsender.clone(),
            disconnect_channel: discon_sender.clone()
        });
    server.send(ServerTaskCommand::Send { message: MessageFromListener::SessionConnected {
        session, source: src
    }}).await.unwrap();
    
    'client_loop: loop {
        tokio::select!(
            Some(()) = discon_receiver.recv() => {
                info!("Client connection {} instructed for immediate disconnect", session);
                break 'client_loop;
            }
            Some(message) = lreceiver.recv() => {
                match message {
                    SessionCommand::Disconnect => {
                        info!("Client connection {} instructed for disconnect", session);
                        break 'client_loop;
                    }
                    SessionCommand::SendString { message } =>
                        match ws.send(ws::Message::text(message)).await {
                            Err(e) => {
                                info!("Client connection {} got error {}", session, e);
                            }
                            Ok(()) => {}
                        }
                }
            },
            msg_read = ws.try_next(), if lsender.capacity() > STOP_READING_CAPACITY  => {
                match msg_read {
                    Err(e) => {
                        info!("Client connection {} got error {}", session, e);
                        break 'client_loop;
                    }
                    Ok(None) => {
                        info!("Client connection {} closed", session);
                        break 'client_loop;
                    }
                    Ok(Some(msg)) if msg.is_close() => {
                        info!("Client connection {} got WS close message", session);
                        break 'client_loop;
                    }
                    Ok(Some(wsmsg)) if wsmsg.is_text() => {
                        match wsmsg.to_str() {
                            Err(_) => {}
                            Ok(msg) => {
                                server.send(ServerTaskCommand::Send {
                                    message: MessageFromListener::SessionSentLine {
                                        session,
                                        msg: msg.to_owned()
                                    }
                                }).await.unwrap_or(());
                            }
                        }
                    }
                    _ => {}
                }
            },
            _ = time::sleep(time::Duration::from_secs(60)) => {
                match ws.send(ws::Message::ping([])).await {
                    Err(e) => {
                        info!("Client connection {} got error {}", session, e);
                    }
                    Ok(()) => {}
                }
            }
        );
    }

    server.send(ServerTaskCommand::Send { message: MessageFromListener::SessionDisconnected {
        session
    }}).await.unwrap();
    active_sessions.lock().await.remove(&session);
}

async fn upgrade_websocket(src: String, wsreq: ws::Ws,
                           active_sessions: SessionMap,
                           server_sender: mpsc::Sender<ServerTaskCommand>) ->
    Result<impl warp::Reply, warp::Rejection> {
        Ok(
            wsreq.on_upgrade(|wss| handle_websocket(
                wss, src, active_sessions,
                server_sender))
        )
}

async fn start_websocket(bind: String, active_sessions: SessionMap, server_sender: mpsc::Sender<ServerTaskCommand>) -> DResult<()> {
    let sockaddr = lookup_host(bind).await?.next().expect("Can't resolve websocket bind name");
    let routes =
        warp::get()
        .and(warp::path("wsgame"))
        .and(warp::header("X-Forwarded-For"))
        .and(ws::ws())
        .and_then(move |src, wsreq| upgrade_websocket(src, wsreq, active_sessions.clone(), server_sender.clone()));

    task::spawn(
        warp::serve(
            routes
        ).run(sockaddr)
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let listener_id = Uuid::new_v4();
    let mut config = read_latest_config()?;
    let active_sessions: SessionMap =
        Arc::new(Mutex::new(BTreeMap::new()));
    let server_sender = start_server_task(listener_id, config.gameserver, active_sessions.clone());

    start_pinger(listener_id, server_sender.clone());
    // Note: for now, this cannot be reconfigured without a complete restart.
    start_websocket(config.ws_listener, active_sessions.clone(), server_sender.clone()).await?;
    
    let mut sighups = signal(SignalKind::hangup())?;
      
    loop {
        let mut listen_handles = Vec::new();
        for listener in config.listeners.clone() {
            let server_sender_for_listener = server_sender.clone();
            let active_sessions_for_listener = active_sessions.clone();
            listen_handles.push(task::spawn(async move { 
                match TcpListener::bind(&listener).await {
                    Err(e) => { warn!("Error listening to {}: {}", &listener, e); }
                    Ok(listensock) => {
                        loop {
                            match listensock.accept().await {
                                Err(e) => { warn!("Error accepting connection from {}: {}",
                                                  &listener, e); }
                                Ok((stream, addr)) => {
                                    let server_sender_for_client = server_sender_for_listener.clone();
                                    let active_sessions_for_client = active_sessions_for_listener.clone();
                                    task::spawn(async move {
                                        handle_client_socket(server_sender_for_client,
                                                             active_sessions_for_client,
                                                             stream,
                                                             addr
                                        ).await;
                                    }); }
                            }
                        }
                    }
                }
            }));
        }
        
        sighups.recv().await;
        
        info!("Reloading configurations");
        config = read_latest_config()?;
        
        // Note: It is deliberate behaviour to send this even if gameserver
        // hasn't changed - SIGHUP is to be used after a server hot cutover to tell
        // it to connect to the new server process even if on the same port.
        server_sender.send(ServerTaskCommand::SwitchTo { new_server: config.gameserver })
            .await?;
 
        for handle in &listen_handles {
            handle.abort();
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn doesnt_stop_reading_at_max_capacity() {
        use crate::*;
        assert!(MAX_CAPACITY > STOP_READING_CAPACITY);    
    }
}
