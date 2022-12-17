use serde::Deserialize;
use std::fs;
use std::error::Error;
use log::{info, error, LevelFilter};
use simple_logger::SimpleLogger;
use tokio::signal::unix::{signal, SignalKind};

mod db;
mod listener;
mod message_handler;
mod version_cutover;
mod av;

#[derive(Deserialize, Debug)]
struct Config {
    listener: String,
    pidfile: String,
    database_conn_string: String
}

fn read_latest_config() -> Result<Config, Box<dyn Error>> {
    serde_yaml::from_str(&fs::read_to_string("gameserver.conf")?).
        map_err(|error| Box::new(error) as Box<dyn Error>)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    av::check().or_else(|e| -> Result<(), Box<dyn Error>> {
        error!("Couldn't verify age-verification.yml - this is not a complete game. Check README.md: {}", e);
        Err(e)
    })?;
    let config = read_latest_config()?;
    let pool = db::start_pool(&config.database_conn_string)?;

    // Test the database connection string works so we quit early if not...
    let _ = pool.get().await?.query("SELECT 1", &[]).await?;

    info!("Database pool initialised: {:?}", pool.status());

    let listener_map = listener::make_listener_map();
    listener::start_listener(config.listener, listener_map.clone(),
                             move |listener_id, msg| {
                                 message_handler::handle(listener_id, msg, pool.clone(), listener_map.clone())
                             }
    ).await?;

    version_cutover::replace_old_gameserver(&config.pidfile)?;

    let mut sigusr1 = signal(SignalKind::user_defined1())?;
    sigusr1.recv().await;
    
    Ok(())
}
