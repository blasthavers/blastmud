use crate::message_handler::ListenerSession;
use crate::DResult;
use crate::db::DBPool;
use ansi::ansi;
use std::default::Default;
use crate::models::session::Session;

pub async fn handle(session: &ListenerSession, source: String, pool: &DBPool) -> DResult<()> {
    pool.start_session(session, &Session { source, ..Default::default() }).await?;
    pool.queue_for_session(&session, Some(&ansi!("\
      Welcome to <red>BlastMud<reset> - a text-based post-apocalyptic \
      game <bold>restricted to adults (18+)<reset>\r\n\
      Some commands to get you started:\r\n\
      \t<bold>register <lt>username> <lt>password> <lt>email><reset> to register as a new user.\r\n\
      \t<bold>login <lt>username> <lt>password><reset> to log in as an existing user.\r\n\
      \t<bold>help<reset> to learn more.\r\n\
      [Please note BlastMud is still under development. You are welcome to play as we \
      develop it, but note it might still have bugs, unimplemented features, and \
      unbalanced gameplay aspects].\r\n"))).await?;
    Ok(())
}
