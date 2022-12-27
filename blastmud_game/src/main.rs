use serde::Deserialize;
use std::fs;
use std::error::Error;
use log::{info, error, LevelFilter};
use simple_logger::SimpleLogger;
use tokio::signal::unix::{signal, SignalKind};
use db::DBPool;

mod db;
mod listener;
mod message_handler;
mod version_cutover;
mod av;
mod regular_tasks;
mod models;
mod static_content;

pub type DResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Deserialize, Debug)]
struct Config {
    listener: String,
    pidfile: String,
    database_conn_string: String
}

fn read_latest_config() -> DResult<Config> {
    serde_yaml::from_str(&fs::read_to_string("gameserver.conf")?).
        map_err(|error| Box::new(error) as Box<dyn Error + Send + Sync>)
}

#[tokio::main]
async fn main() -> DResult<()> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    av::check().or_else(|e| -> Result<(), Box<dyn Error + Send + Sync>> {
        error!("Couldn't verify age-verification.yml - this is not a complete game. Check README.md: {}", e);
        Err(e)
    })?;
    let config = read_latest_config()?;
    let pool = DBPool::start(&config.database_conn_string)?;

    // Test the database connection string works so we quit early if not...
    let _ = pool.get_conn().await?.query("SELECT 1", &[]).await?;

    info!("Database pool initialised");

    let listener_map = listener::make_listener_map();

    let mh_pool = pool.clone();
    listener::start_listener(config.listener, listener_map.clone(),
                             move |listener_id, msg| {
                                 message_handler::handle(listener_id, msg, mh_pool.clone())
                             }
    ).await?;

    static_content::refresh_static_content(&pool).await?;
    
    version_cutover::replace_old_gameserver(&config.pidfile)?;
    regular_tasks::start_regular_tasks(&pool, listener_map)?;
    
    let mut sigusr1 = signal(SignalKind::user_defined1())?;
    sigusr1.recv().await;
    
    Ok(())
}
