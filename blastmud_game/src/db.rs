use tokio_postgres::config::Config as PgConfig;
use deadpool_postgres::{Manager, Object, ManagerConfig, Pool,
                        RecyclingMethod};
use std::error::Error;
use std::str::FromStr;
use uuid::Uuid;
use tokio_postgres::NoTls;
use crate::DResult;

#[derive(Clone, Debug)]
pub struct DBPool {
    pool: Pool
}

impl DBPool {
    pub async fn record_listener_ping(self: DBPool, listener: Uuid) -> DResult<()> {
        self.get_conn().await?.execute(
            "INSERT INTO listeners (listener, last_seen) \
             VALUES ($1, NOW()) \
             ON CONFLICT (listener) \
             DO UPDATE SET last_seen = EXCLUDED.last_seen", &[&listener]).await?;
        Ok(())
    }

    pub async fn get_dead_listeners(self: DBPool) -> DResult<Vec<Uuid>> {
        Ok(self.get_conn().await?
           .query("SELECT listener FROM listeners WHERE last_seen < NOW() - \
                   INTERVAL '2 minutes'", &[])
           .await?.into_iter().map(|r| r.get(0)).collect())
    }

    pub async fn cleanup_listener(self: DBPool, listener: Uuid) -> DResult<()> {
        let mut conn = self.get_conn().await?;
        let tx = conn.transaction().await?;
        tx.execute("UPDATE users SET current_session = NULL, \
                    current_listener = NULL WHERE current_listener = $1",
                   &[&listener]).await?;
        tx.execute("DELETE FROM sendqueue WHERE listener = $1",
                   &[&listener]).await?;
        tx.execute("DELETE FROM sessions WHERE listener = $1",
                   &[&listener]).await?;
        tx.execute("DELETE FROM listeners WHERE listener = $1",
                   &[&listener]).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn start_session(self: DBPool, listener: Uuid, session: Uuid) -> DResult<()> {
        self.get_conn().await?.execute(
            "INSERT INTO sessions (session, listener, details) VALUES ($1, $2, '{}')",
            &[&session, &listener]
        ).await?;
        Ok(())
    }
    
    pub async fn get_conn(self: DBPool) ->
        DResult<Object> {
            let conn = self.pool.get().await?;
            conn.execute("SET synchronous_commit=off", &[]).await?;
            Ok(conn)
        }
    
    pub fn start(connstr: &str) -> DResult<DBPool> {
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast
        };
        let mgr = Manager::from_config(
            PgConfig::from_str(connstr)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?,
            NoTls, mgr_config
        );
        
        Pool::builder(mgr).max_size(4).build()
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
            .map(|pool| DBPool { pool })
    }
}
