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
use crate::message_handler::user_commands::parsing::parse_offset;
use crate::models::{
    session::Session,
    user::User,
    item::Item,
    task::{Task, TaskParse}
};
use tokio_postgres::types::ToSql;
use std::collections::BTreeSet;
use std::sync::Arc;
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
    
    pub async fn find_static_task_types(self: &Self) -> DResult<Box<BTreeSet<String>>> {
        Ok(Box::new(
            self
                .get_conn().await?
                .query("SELECT DISTINCT details->>'task_type' AS task_type \
                        FROM tasks WHERE details->>'is_static' = 'true'", &[]).await?
               .iter()
                .map(|r| r.get("task_type"))
                .collect()))
    }
    
    pub async fn delete_static_items_by_type(self: &Self, item_type: &str) -> DResult<()> {
        self.get_conn().await?.query(
            "DELETE FROM items WHERE details->>'is_static' = 'true' AND details->>'item_type' = {}",
            &[&item_type]).await?;
        Ok(())
    }
    
    pub async fn delete_static_tasks_by_type(self: &Self, task_type: &str) -> DResult<()> {
        self.get_conn().await?.query(
            "DELETE FROM tasks WHERE details->>'is_static' = 'true' AND details->>'task_type' = {}",
            &[&task_type]).await?;
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

#[derive(Clone, Debug)]
pub struct ItemSearchParams<'l> {
    pub from_item: &'l Item,
    pub query: &'l str,
    pub include_contents: bool,
    pub include_loc_contents: bool,
    pub include_active_players: bool,
    pub include_all_players: bool
}

impl ItemSearchParams<'_> {
    pub fn base<'l>(from_item: &'l Item, query: &'l str) -> ItemSearchParams<'l> {
        ItemSearchParams {
            from_item, query,
            include_contents: false,
            include_loc_contents: false,
            include_active_players: false,
            include_all_players: false
        }
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
                        "total_xp", "total_stats", "total_skills", "pronouns"] {
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

    pub async fn limited_update_static_task(self: &Self, task: &Task) -> DResult<()> {
        let value = serde_json::to_value(task)?;
        let obj_map = value.as_object()
            .expect("Static task to be object in JSON");
        let task_name: &(dyn ToSql + Sync) = &task.details.name();
        let mut params: Vec<&(dyn ToSql + Sync)> = vec!(task_name, &task.meta.task_code);
        let mut det_ex: String = "details".to_owned();
        let mut var_id = 3;
        // Only copy more permanent fields, others are supposed to change over time and shouldn't
        // be reset on restart. We do reset failure count since the problem may be fixed.
        for to_copy in ["recurrence", "consecutive_failure_count", "task_details"] {
            det_ex = format!("jsonb_set({}, '{{{}}}', ${})", det_ex, to_copy, var_id);
            params.push(obj_map.get(to_copy).unwrap_or(&Value::Null));
            var_id += 1;
        }
        self.pg_trans()?.execute(
            &("UPDATE tasks SET details = ".to_owned() + &det_ex +
              " WHERE details->>'task_type' = $1 AND details->>'task_code' = $2"),
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

    pub async fn find_static_tasks_by_type(self: &Self, task_type: &str) ->
        DResult<Box<BTreeSet<String>>> {
        Ok(Box::new(
            self.pg_trans()?
                .query("SELECT DISTINCT details->>'task_code' AS task_code FROM tasks WHERE \
                        details->>'is_static' = 'true' AND \
                        details->>'task_type' = $1", &[&task_type])
                .await?
                .into_iter()
                .map(|v| v.get("task_code"))
                .collect()))
    }
    
    pub async fn delete_static_items_by_code(self: &Self, item_type: &str,
                                             item_code: &str) -> DResult<()> {
        self.pg_trans()?.query(
            "DELETE FROM items WHERE details->>'is_static' = 'true' AND \
               details->>'item_type' = $1 AND \
               details->>'item_code' = $2",
            &[&item_type, &item_code]).await?;
        Ok(())
    }

    pub async fn delete_static_tasks_by_code(self: &Self, task_type: &str,
                                             task_code: &str) -> DResult<()> {
        self.pg_trans()?.query(
            "DELETE FROM task WHERE details->>'is_static' = 'true' AND \
               details->>'task_type' = $1 AND \
               details->>'task_code' = $2",
            &[&task_type, &task_code]).await?;
        Ok(())
    }
    
    pub async fn find_item_by_type_code(self: &Self, item_type: &str, item_code: &str) ->
        DResult<Option<Arc<Item>>> {
        if let Some(item) = self.pg_trans()?.query_opt(
            "SELECT details FROM items WHERE \
             details->>'item_type' = $1 AND \
             details->>'item_code' = $2", &[&item_type, &item_code]).await? {
          return Ok(Some(Arc::new(serde_json::from_value::<Item>(item.get("details"))?)));
        }
        Ok(None)
    }

    pub async fn find_items_by_location(self: &Self, location: &str) -> DResult<Vec<Arc<Item>>> {
        Ok(self.pg_trans()?.query(
            "SELECT details FROM items WHERE details->>'location' = $1 \
             ORDER BY details->>'display'
             LIMIT 100", &[&location]
        ).await?.into_iter()
           .filter_map(|i| serde_json::from_value(i.get("details")).ok())
           .map(Arc::new)
           .collect())
    }

    pub async fn save_item_model(self: &Self, details: &Item)
                                 -> DResult<()> {
        self.pg_trans()?
            .execute("UPDATE items SET details = $1 WHERE \
                      details->>'item_type' = $2 AND \
                      details->>'item_code' = $3",
                     &[&serde_json::to_value(details)?,
                       &details.item_type, &details.item_code]).await?;
        Ok(())
    }

    pub async fn find_session_for_player(self: &Self, item_code: &str) -> DResult<Option<(ListenerSession, Session)>> {
        Ok(self.pg_trans()?
           .query_opt("SELECT u.current_listener, u.current_session, s.details \
                       FROM users u JOIN sessions s ON s.session = u.current_session \
                       WHERE u.username=$1", &[&item_code])
           .await?
           .and_then(|r| match (r.get("current_listener"), r.get("current_session"),
                                r.get("details")) {
               (Some(listener), Some(session), details) =>
                   Some((ListenerSession { listener, session },
                         serde_json::from_value(details).ok()?)),
               _ => None
           }))
    }
    
    pub async fn resolve_items_by_display_name_for_player<'l>(
        self: &Self,
        search: &'l ItemSearchParams<'l>
    ) -> DResult<Arc<Vec<Arc<Item>>>> {
        let mut ctes: Vec<String> = Vec::new();
        let mut include_tables: Vec<&'static str> = Vec::new();

        let player_loc = &search.from_item.location;
        let player_desig = format!("{}/{}", search.from_item.item_type,
                                   search.from_item.item_code);
        
        let (offset, query) = parse_offset(search.query);
        let mut param_no: usize = 4;
        let query_wildcard = query.replace("\\", "\\\\")
              .replace("_", "\\_")
              .replace("%", "")
            .to_lowercase() + "%";
        let offset_sql = offset.map(|x| (if x >= 1 { x - 1 } else { x}) as i64).unwrap_or(0);
        let query_len = query.len() as i32;
        let mut params: Vec<&(dyn ToSql + Sync)> = vec!(
            &query_wildcard,
            &offset_sql, &query_len);
        
        
        if search.include_contents {
            ctes.push(format!("contents AS (\
                               SELECT details FROM items WHERE details->>'location' = ${}\
                               )", param_no));
            param_no += 1;
            params.push(&player_desig);
            include_tables.push("SELECT details FROM contents");
        }
        if search.include_loc_contents {
            ctes.push(format!("loc_contents AS (\
                               SELECT details FROM items WHERE details->>'location' = ${}\
                               )", param_no));
            drop(param_no); // or increment if this is a problem.
            params.push(&player_loc);
            include_tables.push("SELECT details FROM loc_contents");
        }
        if search.include_active_players {
            ctes.push("active_players AS (\
                         SELECT details FROM items WHERE details->>'item_type' = 'player' \
                                        AND current_session IS NOT NULL \
                      )".to_owned());
            include_tables.push("SELECT details FROM active_players");
        }
        if search.include_all_players {
            ctes.push("all_players AS (\
                         SELECT details FROM items WHERE details->>'item_type' = 'player'
                      )".to_owned());
            include_tables.push("SELECT details FROM all_players");
        }
        ctes.push(format!("relevant_items AS ({})", include_tables.join(" UNION ")));

        let cte_str: String = ctes.join(", ");

        Ok(Arc::new(self.pg_trans()?.query(
            &format!(
                "WITH {} SELECT details FROM relevant_items WHERE (lower(details->>'display') LIKE $1) \
                 OR (lower(details ->>'display_less_explicit') LIKE $1) \
                 ORDER BY ABS(length(details->>'display')-$3) ASC \
                 LIMIT 1 OFFSET $2", &cte_str),
            &params
        ).await?.into_iter()
                    .filter_map(|i| serde_json::from_value(i.get("details")).ok())
                    .map(Arc::new)
                    .collect()))
    }

    pub async fn get_next_scheduled_task(&self) -> DResult<Option<TaskParse>> {
        match self.pg_trans()?.query_opt(
            "SELECT details FROM tasks WHERE \
             CAST(details->>'next_scheduled' AS TIMESTAMPTZ) <= now() \
             ORDER BY details->>'next_scheduled' ASC LIMIT 1", &[]
        ).await? {
            None => Ok(None),
            Some(row) => Ok(serde_json::from_value(row.get("details"))?)
        }
    }

    pub async fn delete_task(&self, task_type: &str, task_code: &str) -> DResult<()> {
        self.pg_trans()?.execute(
            "DELETE FROM tasks WHERE details->>'task_type' = $1 AND \
            details->>'task_code' = $2", &[&task_type, &task_code]
        ).await?;
        Ok(())
    }

    pub async fn upsert_task(&self, task: &Task) -> DResult<()> {
        self.pg_trans()?.execute(
            "INSERT INTO tasks (details) \
             VALUES ($1) \
             ON CONFLICT ((details->>'task_code'), (details->>'task_type')) \
             DO UPDATE SET details = $1", &[&serde_json::to_value(task)?]).await?;
        Ok(())
    }
    
    pub async fn update_task(&self, task_type: &str, task_code: &str, task: &TaskParse) -> DResult<()> {
        self.pg_trans()?.execute(
            "UPDATE tasks SET details = $3 WHERE details->>'task_type' = $1 AND \
             details->>'task_code' = $2",
            &[&task_type, &task_code, &serde_json::to_value(task)?]
        ).await?;
        Ok(())
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
