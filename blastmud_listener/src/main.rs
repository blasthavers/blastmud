use std::vec::Vec;
use std::error::Error;
use std::fs;
use serde::*;
use tokio::task;
use tokio::net::{TcpStream, TcpListener};
use tokio::signal::unix::{signal, SignalKind};
use log::{warn, info};
use simple_logger::SimpleLogger;
use rand::thread_rng;
use rand::seq::SliceRandom;
use tokio::time::{self, Duration};
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Debug)]
struct Config {
    listeners: Vec<String>,
    gameservers: Vec<String>,
}

fn read_latest_config() -> Result<Config, Box<dyn std::error::Error>> {
    serde_yaml::from_str(&fs::read_to_string("listener.conf")?).
        map_err(|error| Box::new(error) as Box<dyn Error>)
}

#[derive(Clone)]
enum ServerInfo {
    Connected { stream: Arc<TcpStream>, host: String },
    Disconnected { host: String },
}

async fn connect_upstream(upstream: &str) -> ServerInfo {
    info!("About to connect to {}", upstream);
    let stream = TcpStream::connect(&upstream).await;
    match stream {
        Ok(stream) => {
            info!("Connected to {}", upstream);
            ServerInfo::Connected { stream: Arc::new(stream), host: upstream.to_string() }
        }
        Err(e) => {
            warn!("Couldn't connect to game: {}", e);
            ServerInfo::Disconnected { host: upstream.to_string() }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();
    
    let mut sighups = signal(SignalKind::hangup())?;
    loop {
        let config = read_latest_config()?;
        let mut new_servers = Vec::new();
        for gameserver in config.gameservers {
            new_servers.push(connect_upstream(&gameserver).await);
        }

        let mut servers = Arc::new(Mutex::new(new_servers));

        let send_server = |msg: &str| async move {
            loop {
                let mut servers_lock = servers.lock().unwrap();
                let connected: Vec<&ServerInfo> = (*servers_lock).iter().filter(|serv| match serv {
                    ServerInfo::Connected { .. } => true,
                    _ => false
                }).collect();
                
                match connected.choose(&mut thread_rng()) {
                    None => {
                        time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        };
        
        let mut listen_handles = Vec::new();
        for listener in config.listeners {
            listen_handles.push(task::spawn(async move { 
                match TcpListener::bind(&listener).await {
                    Err(e) => { warn!("Error listening to {}: {}", &listener, e); }
                    Ok(listensock) => {
                        loop {
                            match listensock.accept().await {
                                Err(e) => { warn!("Error accepting connection from {}: {}",
                                                  &listener, e); }
                                _ => {}
                            }
                        }
                    }
                }
            }));
        }

        let reconnect_handle = task::spawn(async move {
            let mut new_servers = Vec::new();
            let mut old_server_lock = servers.lock().unwrap();
            let mut old_servers = (*old_server_lock).clone();
            drop(old_server_lock);
            
            for server in old_servers {
                match server {
                    ServerInfo::Disconnected { host } => {
                        new_servers.push(connect_upstream(&host).await)
                    }
                    x => { new_servers.push(x) }
                }
            }
            *(servers.lock().unwrap()) = new_servers;
        });
        
        let mut should_reload = false;

        while !should_reload {
            tokio::select!(_ = sighups.recv() => {
                should_reload = true
            })
        }

        for handle in &listen_handles {
            handle.abort();
        }
        reconnect_handle.abort();
        
        info!("Reloading configurations")
    }
}
