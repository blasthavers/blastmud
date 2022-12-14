use serde::Deserialize;
use std::fs;
use std::error::Error;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

mod db;
mod listener;

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

    let config = read_latest_config()?;
    let pool = db::start_pool(&config.database_conn_string)?;

    // Test the database connection string works so we quit early if not...
    let _ = pool.get().await?.query("SELECT 1", &[]).await?;

    info!("Database pool initialised: {:?}", pool.status());

    let listener = listener::start_listener(config.listener);
    
    Ok(())
}
