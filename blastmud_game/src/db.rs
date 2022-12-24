use tokio_postgres::{config::Config as PgConfig, row::Row};
use deadpool_postgres::{Manager, Object, ManagerConfig, Pool,
                        RecyclingMethod};
use std::error::Error;
use std::str::FromStr;
use uuid::Uuid;
use tokio_postgres::NoTls;
use crate::message_handler::ListenerSession;
use crate::DResult;
use crate::models::session::Session;
use serde_json;

#[derive(Clone, Debug)]
pub struct DBPool {
    pool: Pool
}

#[derive(Clone, Debug)]
pub struct SendqueueItem {
    pub item: i64,
    pub session: ListenerSession,
    pub message: String
}
impl From<Row> for SendqueueItem {
    fn from(row: Row) -> Self {
        SendqueueItem {
            item: row.get("item"),
            session: ListenerSession {
                session: row.get("session"),
                listener: row.get("listener")
            },
            message: row.get("message")
        }
    }
}

impl DBPool {
    pub async fn record_listener_ping(self: &DBPool, listener: Uuid) -> DResult<()> {
        self.get_conn().await?.execute(
            "INSERT INTO listeners (listener, last_seen) \
             VALUES ($1, NOW()) \
             ON CONFLICT (listener) \
             DO UPDATE SET last_seen = EXCLUDED.last_seen", &[&listener]).await?;
        Ok(())
    }

    pub async fn get_dead_listeners(self: &Self) -> DResult<Vec<Uuid>> {
        Ok(self.get_conn().await?
           .query("SELECT listener FROM listeners WHERE last_seen < NOW() - \
                   INTERVAL '2 minutes'", &[])
           .await?.into_iter().map(|r| r.get(0)).collect())
    }

    pub async fn cleanup_listener(self: &Self, listener: Uuid) -> DResult<()> {
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

    pub async fn start_session(self: &Self, session: &ListenerSession, details: &Session) -> DResult<()> {
        self.get_conn().await?.execute(
            "INSERT INTO sessions (session, listener, details) \
               VALUES ($1, $2, $3) ON CONFLICT (session) DO NOTHING",
            &[&session.session, &session.listener, &serde_json::to_value(details)?]
        ).await?;
        Ok(())
    }

    pub async fn end_session(self: &Self, session: ListenerSession) -> DResult<()> {
        let mut conn = self.get_conn().await?;
        let tx = conn.transaction().await?;
        tx.execute("UPDATE users SET current_session = NULL, \
                    current_listener = NULL WHERE current_session = $1",
                   &[&session.session]).await?;
        tx.execute("DELETE FROM sendqueue WHERE session = $1",
                   &[&session.session]).await?;
        tx.execute("DELETE FROM sessions WHERE session = $1",
                   &[&session.session]).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn queue_for_session(self: &Self,
                                       session: &ListenerSession,
                                       message: &str) -> DResult<()> {
        let conn = self.get_conn().await?;
        conn.execute("INSERT INTO sendqueue (session, listener, message) VALUES ($1, $2, $3)",
                     &[&session.session, &session.listener, &message]).await?;
        Ok(())
    }

    pub async fn get_from_sendqueue(self: &Self) -> DResult<Vec<SendqueueItem>> {
        let conn = self.get_conn().await?;
        Ok(conn.query("SELECT item, session, listener, message FROM sendqueue ORDER BY item ASC LIMIT 10",
                      &[])
           .await?.into_iter().map(SendqueueItem::from).collect())
        
    }

    pub async fn delete_from_sendqueue(self: &DBPool, item: &SendqueueItem) -> DResult<()> {
        let conn = self.get_conn().await?;
        conn.execute("DELETE FROM sendqueue WHERE item=$1", &[&item.item]).await?;
        Ok(())
    }
    
    pub async fn get_conn(self: &DBPool) ->
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
            .map(|pool| Self { pool })
    }
}
