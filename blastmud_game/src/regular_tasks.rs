use tokio::{task, time};
use crate::DResult;
use crate::db;
use log::warn;

async fn cleanup_session_once(pool: db::DBPool) -> DResult<()> {
    for listener in pool.clone().get_dead_listeners().await? {
        pool.clone().cleanup_listener(listener).await?;
    }
    Ok(())
}

fn start_session_cleanup_task(pool: db::DBPool) -> DResult<()> {
    task::spawn(async move {
        loop {
            match cleanup_session_once(pool.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("Error cleaning up sessions: {}", e);
                    time::sleep(time::Duration::from_secs(1)).await;
                }
            }
        }
    });
    Ok(())
}

pub fn start_regular_tasks(pool: db::DBPool) -> DResult<()> {
    start_session_cleanup_task(pool)
}
