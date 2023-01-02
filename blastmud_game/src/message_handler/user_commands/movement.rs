use super::{
    VerbContext, UserVerb, UserVerbRef, UResult, UserError, user_error,
    get_player_item_or_fail,
    look
};
use async_trait::async_trait;
use crate::{
    regular_tasks::queued_command::{
        QueueCommandHandler,
        QueueCommand,
        queue_command
    },
    static_content::room::{self, Direction, ExitType}
};
use std::time;

pub struct QueueHandler;
#[async_trait]
impl QueueCommandHandler for QueueHandler {
    async fn start_command(&self, _ctx: &mut VerbContext<'_>, _command: &QueueCommand)
                           -> UResult<time::Duration> {
        Ok(time::Duration::from_secs(1))
    }

    #[allow(unreachable_patterns)]
    async fn finish_command(&self, ctx: &mut VerbContext<'_>, command: &QueueCommand)
                            -> UResult<()> {
        let direction = match command {
            QueueCommand::Movement { direction } => direction,
            _ => user_error("Unexpected command".to_owned())?
        };
        let player_item = get_player_item_or_fail(ctx).await?;
        let (heretype, herecode) = player_item.location.split_once("/").unwrap_or(("room", "repro_xv_chargen"));
        if heretype != "room" {
            // Fix this when we have planes / boats / roomkits.
            user_error("Navigating outside rooms not yet supported.".to_owned())?
        }
        let room = room::room_map_by_code().get(herecode)
            .ok_or_else(|| UserError("Can't find your current location".to_owned()))?;
        let exit = room.exits.iter().find(|ex| ex.direction == *direction)
            .ok_or_else(|| UserError("There is nothing in that direction".to_owned()))?;

        // Ideally we would queue if we were already moving rather than insta-move.
        match exit.exit_type {
            ExitType::Free => {}
            ExitType::Blocked(blocker) => {
                if !blocker.attempt_exit(ctx, &player_item, exit).await? {
                    user_error("Stopping movement".to_owned())?;
                }
            }
        }

        let new_room =
            room::resolve_exit(room, exit).ok_or_else(|| UserError("Can't find that room".to_owned()))?;
        let mut new_player_item = (*player_item).clone();
        new_player_item.location = format!("{}/{}", "room", new_room.code);
        ctx.trans.save_item_model(&new_player_item).await?;
        look::VERB.handle(ctx, "look", "").await?;
        Ok(())
    }
}

pub struct Verb;

#[async_trait]
impl UserVerb for Verb {
    async fn handle(self: &Self, ctx: &mut VerbContext, verb: &str, remaining: &str) -> UResult<()> {
        let dir = Direction::parse(verb).ok_or_else(|| UserError("Unknown direction".to_owned()))?;
        if remaining.trim() != "" {
            user_error("Movement commands don't take extra data at the end.".to_owned())?;
        }
        queue_command(ctx, &QueueCommand::Movement { direction: dir.clone() }).await
    }
}
static VERB_INT: Verb = Verb;
pub static VERB: UserVerbRef = &VERB_INT as UserVerbRef;
