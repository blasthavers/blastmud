use super::{TaskHandler, TaskRunContext};
use async_trait::async_trait;
use crate::DResult;

pub struct RunQueuedCommandTaskHandler;

#[async_trait]
impl TaskHandler for RunQueuedCommandTaskHandler {
    async fn do_task(&self, _ctx: &mut TaskRunContext) -> DResult<()> {
        Ok(())
        /*
        match ctx.task {
            
            _ => Err("Unexpected task type")?
        }? */
    }
}

pub static HANDLER: &'static (dyn TaskHandler + Sync + Send) = &RunQueuedCommandTaskHandler;
