use super::NPCMessageHandler;
use crate::message_handler::user_commands::{VerbContext, UResult, get_user_or_fail};
use async_trait::async_trait;
use crate::models::{item::Item, user::User, user::StatType};
use ansi::ansi;
use serde::{Serialize, Deserialize};

pub struct StatbotMessageHandler;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StatbotState {
    Brains,
    Senses,
    Brawn,
    Reflexes,
    Endurance,
    Cool,
    AssignGender,
    SetDescription
}

async fn reply(ctx: &VerbContext<'_>, msg: &str) -> UResult<()> {
    ctx.trans.queue_for_session(
        ctx.session,
        Some(&format!(ansi!("Statbot replies in a mechanical voice: <blue>\"{}\"<reset>\n"),
                     msg))
    ).await?;
    Ok(())
}

fn next_action_text(user: &User) -> String {
    let st = user.statbot.as_ref().unwrap_or(&StatbotState::Brains);
    let brn = user.raw_stats.get(&StatType::Brains).cloned().unwrap_or(8);
    let sen = user.raw_stats.get(&StatType::Senses).cloned().unwrap_or(8);
    let brw = user.raw_stats.get(&StatType::Brawn).cloned().unwrap_or(8);
    let refl = user.raw_stats.get(&StatType::Reflexes).cloned().unwrap_or(8);
    let end = user.raw_stats.get(&StatType::Endurance).cloned().unwrap_or(8);
    let col = user.raw_stats.get(&StatType::Cool).cloned().unwrap_or(8);
    let tot = brn + sen + brw + refl + end + col;
    let summary = format!("Brains: {}, Senses: {}, Brawn: {}, Reflexes: {}, Endurance: {}, Cool: {}. To spend: {}", brn, sen, brw, refl, end, col, tot - 48);
    match st {
        StatbotState::Brains => ansi!("I am Statbot, a robot servant of the empire, put here to help you choose how your body will function. The base body has 8 each of brains, senses, brawn, reflexes, endurance and cool - but you get 14 points of improvement. Each point spent lifts that stat by one. Your first job is to choose how much brainpower you will have. If you choose 8, you don't spend any points. There is a maximum of 15 - if you choose 15, you will spend 7 points and have 7 left for other stats.\n\n\tType <green><bold>-statbot brains 8<reset><blue> (or any other number) to set your brains to that number. You will be able to adjust your stats by sending me the new value, up until you leave here. Your stats now are: ").to_owned() + &summary,
        StatbotState::Senses => "".to_owned(),
        StatbotState::Brawn => "".to_owned(),
        StatbotState::Reflexes => "".to_owned(),
        StatbotState::Endurance => "".to_owned(),
        StatbotState::Cool => "".to_owned(),
        StatbotState::AssignGender => "".to_owned(),
        StatbotState::SetDescription => "".to_owned()
    }
}

#[async_trait]
impl NPCMessageHandler for StatbotMessageHandler {
    async fn handle(
        self: &Self,
        ctx: &mut VerbContext,
        _source: &Item,
        _target: &Item,
        message: &str
    ) -> UResult<()> {
        let user = get_user_or_fail(ctx)?;
        match message {
            _ => {
                reply(ctx, &next_action_text(user)).await?;
            }
        }
        Ok(())
    }
}
