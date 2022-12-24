use crate::message_handler::ListenerSession;
use crate::DResult;
use crate::db::DBPool;
use ansi_macro::ansi;

pub async fn handle(session: &ListenerSession, _source: &str, pool: DBPool) -> DResult<()> {
    pool.clone().start_session(session).await?;
    pool.queue_for_session(&session, &ansi!("\
      Welcome to <red>BlastMud<reset> - a text-based post-apocalyptic \
      game <bold>restricted to adults (18+)<reset>\r\n\
      Some commands to get you started:\r\n\
      \t<bold>register <lt>username> <lt>password> <lt>email><reset> to register as a new user.\r\n\
      \t<bold>connect <lt>username> <lt>password><reset> to log in as an existing user.\r\n\
      \t<bold>help<reset> to learn more.\r\n")).await?;
    Ok(())
}
