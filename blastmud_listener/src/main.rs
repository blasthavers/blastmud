use std::vec::Vec;
use std::collections::BTreeMap;
use std::error::Error;
use std::net::SocketAddr;
use std::fs;
use serde::*;
use tokio::task;
use tokio::time::{self, Duration};
use tokio::net::{TcpStream, TcpListener};
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::{mpsc, Mutex};
use tokio::io::{BufReader};
use log::{warn, info};
use simple_logger::SimpleLogger;
use std::sync::Arc;
use blastmud_interfaces::*;
use tokio_util::codec;
use tokio_util::codec::length_delimited::LengthDelimitedCodec;
use tokio_serde::formats::Cbor;
use futures::prelude::*;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Config {
    listeners: Vec<String>,
    gameserver: String,
}

fn read_latest_config() -> Result<Config, Box<dyn std::error::Error>> {
    serde_yaml::from_str(&fs::read_to_string("listener.conf")?).
        map_err(|error| Box::new(error) as Box<dyn Error>)
}

#[derive(Debug, Clone)]
enum ServerTaskCommand {
    SwitchTo { new_server: String },
    Send { message: MessageFromListener }
}

fn run_server_task<FHandler : Fn(MessageToListener) -> () + Send + 'static>(
    unfinished_business: Option<MessageFromListener>,
    mut receiver: mpsc::Receiver<ServerTaskCommand>,
    sender: mpsc::Sender<ServerTaskCommand>,
    server: String,
    message_handler: FHandler
) {
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

        for req in unfinished_business {
            match conn_framed.send(req).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Can't re-send acknowledgement to {}: {}. Dropping message", server, e);
                    // After a re-failure, we don't retry a further time.
                    run_server_task(None, receiver, sender, server, message_handler);
                    return;
                }
            }
        }
        
        'full_select: loop {
            tokio::select!(
                req = conn_framed.try_next() => {
                    match req {
                        Err(e) => {
                            warn!("Got read error from {}: {}", server, e);
                            run_server_task(
                                None,
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
                                receiver,
                                sender,
                                server,
                                message_handler
                            );
                            break 'full_select;
                        }
                        Ok(Some(msg)) => {
                            message_handler(msg);
                        }   
                    }
                    
                    match conn_framed.send(MessageFromListener::AcknowledgeMessage).await {
                        Ok(_) => {}
                        Err(e) => {
                            warn!("Can't send acknowledgement to {}: {}", server, e);
                            run_server_task(None, receiver, sender, server, message_handler);
                            break 'full_select;
                        }
                    }
                },
                Some(req) = receiver.recv() => {
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
                                                message_handler(msg);
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Can't send message to {}: {}", server, e);
                                    run_server_task(
                                        Some(message),
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
    Disconnect
}

struct SessionRecord {
    channel: mpsc::Sender<SessionCommand>
}

type SessionMap = Arc<Mutex<BTreeMap<Uuid, SessionRecord>>>;

fn handle_server_message(session_map: SessionMap, message: MessageToListener) {
}

fn start_server_task(server: String, session_map: SessionMap) -> mpsc::Sender<ServerTaskCommand> {
    let (sender, receiver) = mpsc::channel(20);
    run_server_task(None, receiver, sender.clone(), server,
                    move |msg| { handle_server_message(session_map.clone(), msg); });
    sender
}

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

    let (sender, receiver) = mpsc::channel(20);
    active_sessions.lock().await.insert(session, SessionRecord { channel: sender });
    server.send(ServerTaskCommand::Send { message: MessageFromListener::SessionConnected {
        session, source: addr.to_string()
    }}).await.unwrap();
    
    loop {
        match rbuf.try_next().await {
            Err(e) => {
                info!("Client connection {} got error {}", session, e);
                break;
            }
            Ok(None) => {
                info!("Client connection {} closed", session);
                break;
            }
            Ok(Some(msg)) => {
                server.send(ServerTaskCommand::Send {
                    message: MessageFromListener::SessionSentLine { session, msg }
                }).await.unwrap();
                /* match wstream.write_all((msg + "\r\n").as_bytes()).await {
                    Err(e) => {
                        info!("Client connection {} got error {}", session, e);
                    }
                    Ok(()) => {}
                } */
            }
        }
    }

    server.send(ServerTaskCommand::Send { message: MessageFromListener::SessionDisconnected {
        session
    }}).await.unwrap();
    active_sessions.lock().await.remove(&session);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();

    let mut config = read_latest_config()?;
    let active_sessions: SessionMap =
        Arc::new(Mutex::new(BTreeMap::new()));
    let server_sender = start_server_task(config.gameserver, active_sessions.clone());
    
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
