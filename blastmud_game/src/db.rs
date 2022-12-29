use tokio_postgres::{config::Config as PgConfig, row::Row};
use deadpool_postgres::{Manager, Object, ManagerConfig, Pool, Transaction,
                        RecyclingMethod};
use std::error::Error;
use std::str::FromStr;
use ouroboros::self_referencing;
use uuid::Uuid;
use tokio_postgres::NoTls;
use crate::message_handler::ListenerSession;
use crate::DResult;
use crate::models::{session::Session, user::User, item::Item};
use tokio_postgres::types::ToSql;
use std::collections::BTreeSet;

use serde_json::{self, Value};
use futures::FutureExt;

#[derive(Clone, Debug)]
pub struct DBPool {
    pool: Pool
}

#[self_referencing]
pub struct DBTrans {
    conn: Object,
    #[borrows(mut conn)]
    #[covariant]
    pub trans: Option<Transaction<'this>>
}

#[derive(Clone, Debug)]
pub struct SendqueueItem {
    pub item: i64,
    pub session: ListenerSession,
    pub message: Option<String>
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

    pub async fn start_transaction(self: &Self) -> DResult<DBTrans> {
        let conn = self.get_conn().await?;
        Ok(DBTransAsyncSendTryBuilder {
            conn,
            trans_builder: |conn| Box::pin(conn.transaction().map(|r| r.map(Some)))
        }.try_build().await?)
    }
    
    pub async fn queue_for_session(self: &Self,
                                       session: &ListenerSession,
                                       message: Option<&str>) -> DResult<()> {
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

    pub async fn find_static_item_types(self: &Self) -> DResult<Box<BTreeSet<String>>> {
        Ok(Box::new(
            self
                .get_conn().await?
                .query("SELECT DISTINCT details->>'item_type' AS item_type \
                        FROM items WHERE details->>'is_static' = 'true'", &[]).await?
               .iter()
                .map(|r| r.get("item_type"))
                .collect()))
    }
    
    pub async fn delete_static_items_by_type(self: &Self, item_type: &str) -> DResult<()> {
        self.get_conn().await?.query(
            "DELETE FROM items WHERE details->>'is_static' = 'true' AND details->>'item_type' = {}",
            &[&item_type]).await?;
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

impl DBTrans {
    pub async fn queue_for_session(self: &Self,
                                   session: &ListenerSession,
                                   message: Option<&str>) -> DResult<()> {
        self.pg_trans()?
            .execute("INSERT INTO sendqueue (session, listener, message) VALUES ($1, $2, $3)",
                     &[&session.session, &session.listener, &message]).await?;
        Ok(())
    }

    pub async fn get_session_user_model(self: &Self, session: &ListenerSession) -> DResult<Option<(Session, Option<User>)>> {
        match self.pg_trans()?
               .query_opt("SELECT s.details AS sess_details, \
                          u.details AS user_details FROM sessions s \
                          LEFT JOIN users u ON u.current_session = s.session \
                          WHERE s.session = $1", &[&session.session])
               .await? {
            None => Ok(None),
            Some(row) =>
                       Ok(Some(
                           (serde_json::from_value(
                               row.get("sess_details"))?,
                            match row.get::<&str, Option<serde_json::Value>>("user_details") {
                                None => None,
                                Some(v) => serde_json::from_value(v)?
                            })
                       ))
        }
    }

    pub async fn save_session_model(self: &Self, session: &ListenerSession, details: &Session)
                                    -> DResult<()> {
        self.pg_trans()?
            .execute("UPDATE sessions SET details = $1 WHERE session = $2",
                     &[&serde_json::to_value(details)?, &session.session]).await?;
        Ok(())
    }

    pub async fn find_by_username(self: &Self, username: &str) -> DResult<Option<User>> {
        if let Some(details_json) = self.pg_trans()?
            .query_opt("SELECT details FROM users WHERE username=$1",
                       &[&username.to_lowercase()]).await? {
                return Ok(Some(serde_json::from_value(details_json.get("details"))?))
            }
        Ok(None)
    }

    pub async fn create_item(self: &Self, item: &Item) -> DResult<i64> {
        Ok(self.pg_trans()?.query_one("INSERT INTO items (details) VALUES ($1) RETURNING item_id",
                                   &[&serde_json::to_value(item)?]).await?
           .get("item_id"))
    }

    pub async fn limited_update_static_item(self: &Self, item: &Item) -> DResult<()> {
        let value = serde_json::to_value(item)?;
        let obj_map = value.as_object()
            .expect("Static item to be object in JSON");
        let mut params: Vec<&(dyn ToSql + Sync)> = vec!(&item.item_type, &item.item_code);
        let mut det_ex: String = "details".to_owned();
        let mut var_id = 3;
        // Only copy more permanent fields, others are supposed to change over time and shouldn't
        // be reset on restart.
        for to_copy in ["display", "display_less_explicit", "details", "details_less_explicit",
                        "total_xp", "total_stats", "total_skills"] {
            det_ex = format!("jsonb_set({}, '{{{}}}', ${})", det_ex, to_copy, var_id);
            params.push(obj_map.get(to_copy).unwrap_or(&Value::Null));
            var_id += 1;
        }
        self.pg_trans()?.execute(
            &("UPDATE items SET details = ".to_owned() + &det_ex +
              " WHERE details->>'item_type' = $1 AND details->>'item_code' = $2"),
            &params).await?;
        Ok(())
    }

    pub async fn create_user(self: &Self, session: &ListenerSession, user_dat: &User) -> DResult<()> {
        self.pg_trans()?.execute("INSERT INTO users (\
              username, current_session, current_listener, details\
              ) VALUES ($1, $2, $3, $4)", &[&user_dat.username.to_lowercase(),
                                            &session.session,
                                            &session.listener,
                                            &serde_json::to_value(user_dat)?]).await?;
        Ok(())
    }

    pub async fn save_user_model(self: &Self, details: &User)
                                 -> DResult<()> {
        self.pg_trans()?
            .execute("UPDATE users SET details = $1 WHERE username = $2",
                     &[&serde_json::to_value(details)?,
                       &details.username.to_lowercase()]).await?;
        Ok(())
    }

    pub async fn attach_user_to_session(self: &Self, username: &str,
                                        session: &ListenerSession) -> DResult<()> {
        let username_l = username.to_lowercase();
        self.pg_trans()?
            .execute("INSERT INTO sendqueue (session, listener, message) \
                      SELECT current_session, current_listener, $1 FROM users \
                      WHERE username = $2 AND current_session IS NOT NULL \
                      AND current_listener IS NOT NULL",
                     &[&"Logged in from another session\r\n", &username_l]).await?;
        self.pg_trans()?
            .execute("INSERT INTO sendqueue (session, listener, message) \
                      SELECT current_session, current_listener, null FROM users \
                      WHERE username = $1 AND current_session IS NOT NULL \
                      AND current_listener IS NOT NULL",
                     &[&username_l]).await?;
        self.pg_trans()?
            .execute("UPDATE users SET current_session = $1, current_listener = $2 WHERE username = $3",
                     &[&session.session as &(dyn ToSql + Sync), &session.listener, &username_l]).await?;
        Ok(())
    }
    
    pub async fn find_static_items_by_type(self: &Self, item_type: &str) ->
        DResult<Box<BTreeSet<String>>> {
        Ok(Box::new(
            self.pg_trans()?
                .query("SELECT DISTINCT details->>'item_code' AS item_code FROM items WHERE \
                        details->>'is_static' = 'true' AND \
                        details->>'item_type' = $1", &[&item_type])
                .await?
                .into_iter()
                .map(|v| v.get("item_code"))
                .collect()))
    }

    pub async fn delete_static_items_by_code(self: &Self, item_type: &str,
                                             item_code: &str) -> DResult<()> {
        self.pg_trans()?.query(
            "DELETE FROM items WHERE details->>'is_static' = 'true' AND \
               details->>'item_type' = {} AND \
               details->>'item_code' = {}",
            &[&item_type, &item_code]).await?;
        Ok(())
    }

    pub async fn find_item_by_type_code(self: &Self, item_type: &str, item_code: &str) ->
        DResult<Option<Item>> {
        if let Some(item) = self.pg_trans()?.query_opt(
            "SELECT details FROM items WHERE \
             details->>'item_type' = $1 AND \
             details->>'item_code' = $2", &[&item_type, &item_code]).await? {
          return Ok(serde_json::from_value(item.get("details"))?);
        }
        Ok(None)
    }

    pub async fn find_items_by_location(self: &Self, location: &str) -> DResult<Vec<Item>> {
        Ok(self.pg_trans()?.query(
            "SELECT details FROM items WHERE details->>'location' = $1 \
             LIMIT 20", &[&location]
        ).await?.into_iter()
           .filter_map(|i| serde_json::from_value(i.get("details")).ok())
           .collect())
        
    }

    pub async fn resolve_items_by_display_name_for_player(
        self: &Self,
        from_item: &Item,
        query: &str,
        include_contents: bool,
        include_loc_contents: bool,
        include_active_players: bool,
        include_all_players: bool
    ) -> DResult<Vec<Item>> {
        let mut ctes: Vec<String> = Vec::new();
        let mut include_tables: Vec<&'static str> = Vec::new();

        let player_loc = &from_item.location;
        let player_desig = format!("{}/{}", from_item.item_type,
                                   from_item.item_code);
        if include_contents {
            ctes.push("contents AS (\
                         SELECT details FROM items WHERE details->>'location' = $1
                      )".to_owned());
            include_tables.push("SELECT details FROM contents");
        }
        if include_loc_contents {
            ctes.push("loc_contents AS (\
                         SELECT details FROM items WHERE details->>'location' = $2
                      )".to_owned());
            include_tables.push("SELECT details FROM loc_contents");
        }
        if include_active_players {
            ctes.push("active_players AS (\
                         SELECT details FROM items WHERE details->>'item_type' = 'player' \
                                        AND current_session IS NOT NULL \
                      )".to_owned());
            include_tables.push("SELECT details FROM active_players");
        }
        if include_all_players {
            ctes.push("all_players AS (\
                         SELECT details FROM items WHERE details->>'item_type' = 'player'
                      )".to_owned());
            include_tables.push("SELECT details FROM all_players");
        }
        ctes.push(format!("relevant_items AS ({})", include_tables.join(" UNION ")));

        let cte_str: String = ctes.join(", ");
        
        Ok(self.pg_trans()?.query(
            &format!(
                "WITH {} SELECT details FROM relevant_items WHERE (lower(details->>'display') LIKE $3) \
                 OR (lower(details ->>'display_less_explicit') LIKE $3) \
                 ORDER BY length(details->>'display') DESC \
                 LIMIT 2", &cte_str),
            &[&player_desig, &player_loc,
              &(query.replace("\\", "\\\\")
                .replace("_", "\\_")
                .replace("%", "")
                .to_lowercase() + "%")]
        ).await?.into_iter()
           .filter_map(|i| serde_json::from_value(i.get("details")).ok())
           .collect())
    }
    
    pub async fn commit(mut self: Self) -> DResult<()> {
        let trans_opt = self.with_trans_mut(|t| std::mem::replace(t, None));
        if let Some(trans) = trans_opt {
            trans.commit().await?;
        }
        Ok(())
    }

    pub fn pg_trans(self: &Self) -> DResult<&Transaction> {
        self.borrow_trans().as_ref().ok_or("Transaction already closed".into())
    }
}
