use tokio_postgres::config::Config as PgConfig;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::error::Error;
use std::str::FromStr;
use uuid::Uuid;
use tokio_postgres::NoTls;

pub async fn record_listener_ping(_listener: Uuid, _pool: Pool) {
    // pool.get().await?.query("");
}

pub fn start_pool(connstr: &str) -> Result<Pool, Box<dyn Error>> {
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(
        PgConfig::from_str(connstr)?,
        NoTls, mgr_config
    );

    Pool::builder(mgr).max_size(4).build().map_err(|e| Box::new(e) as Box<dyn Error>)
}
