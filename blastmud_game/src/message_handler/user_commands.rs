use crate::message_handler::ListenerSession;
use crate::DResult;
use crate::db::DBPool;
use ansi_macro::ansi;

pub async fn handle(session: &ListenerSession, msg: &str, pool: DBPool) -> DResult<()> {
    pool.queue_for_session(session,
                           &format!(ansi!(
                               "You hear an echo saying: <bggreen><red>{}<reset>\r\n"
                           ), msg)).await?;
    Ok(())
}
