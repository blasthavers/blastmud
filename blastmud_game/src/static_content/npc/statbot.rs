use super::NPCMessageHandler;
use crate::message_handler::user_commands::{
    VerbContext, UResult,
    get_user_or_fail,
    get_user_or_fail_mut,
    parsing::parse_to_space
};
use async_trait::async_trait;
use crate::models::{
    item::{Item, Sex, Pronouns},
    user::{User, StatType},
    session::Session
};
use ansi::ansi;
use serde::{Serialize, Deserialize};
use nom::character::complete::u8;

pub struct StatbotMessageHandler;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StatbotState {
    Brains,
    Senses,
    Brawn,
    Reflexes,
    Endurance,
    Cool,
    FixTotals,
    AssignSex,
    SetDescription,
    Done
}

async fn reply(ctx: &VerbContext<'_>, msg: &str) -> UResult<()> {
    ctx.trans.queue_for_session(
        ctx.session,
        Some(&format!(ansi!("Statbot replies in a mechanical voice: <blue>\"{}\"<reset>\n"),
                     msg))
    ).await?;
    Ok(())
}

fn work_out_state(user: &User, item: &Item) -> StatbotState {
    if !user.raw_stats.contains_key(&StatType::Brains) {
        return StatbotState::Brains;
    }
    if !user.raw_stats.contains_key(&StatType::Senses) {
        return StatbotState::Senses;
    }
    if !user.raw_stats.contains_key(&StatType::Brawn) {
        return StatbotState::Brawn;
    }
    if !user.raw_stats.contains_key(&StatType::Reflexes) {
        return StatbotState::Reflexes;
    }
    if !user.raw_stats.contains_key(&StatType::Endurance) {
        return StatbotState::Endurance;
    }
    if !user.raw_stats.contains_key(&StatType::Cool) {
        return StatbotState::Cool;
    }
    if points_left(user) != 0 {
        return StatbotState::FixTotals;
    }
    if item.sex.is_none() {
        return StatbotState::AssignSex;
    }
    if item.details == None || item.details == Some("A non-descript individual".to_owned()) {
        return StatbotState::SetDescription;
    }
    StatbotState::Done
}

fn points_left(user: &User) -> u16 {
    let brn = user.raw_stats.get(&StatType::Brains).cloned().unwrap_or(8);
    let sen = user.raw_stats.get(&StatType::Senses).cloned().unwrap_or(8);
    let brw = user.raw_stats.get(&StatType::Brawn).cloned().unwrap_or(8);
    let refl = user.raw_stats.get(&StatType::Reflexes).cloned().unwrap_or(8);
    let end = user.raw_stats.get(&StatType::Endurance).cloned().unwrap_or(8);
    let col = user.raw_stats.get(&StatType::Cool).cloned().unwrap_or(8);
    (62 - (brn + sen + brw + refl + end + col) as i16).max(0) as u16
}

fn next_action_text(session: &Session, user: &User) -> String {
    let brn = user.raw_stats.get(&StatType::Brains).cloned().unwrap_or(8);
    let sen = user.raw_stats.get(&StatType::Senses).cloned().unwrap_or(8);
    let brw = user.raw_stats.get(&StatType::Brawn).cloned().unwrap_or(8);
    let refl = user.raw_stats.get(&StatType::Reflexes).cloned().unwrap_or(8);
    let end = user.raw_stats.get(&StatType::Endurance).cloned().unwrap_or(8);
    let col = user.raw_stats.get(&StatType::Cool).cloned().unwrap_or(8);
    let summary = format!("Brains: {}, Senses: {}, Brawn: {}, Reflexes: {}, Endurance: {}, Cool: {}. To spend: {}", brn, sen, brw, refl, end, col, points_left(user));

    let st = user.statbot.as_ref().unwrap_or(&StatbotState::Brains);
    
    match st {
        StatbotState::Brains => ansi!(
            "I am Statbot, a robot servant of the empire, put here to help you choose \
             how your body will function. The base body has 8 each of brains, senses, \
             brawn, reflexes, endurance and cool - but you get 14 points of improvement. \
             Each point spent lifts that stat by one. Your first job is to choose how much \
             brainpower you will have. If you choose 8, you don't spend any points. There \
             is a maximum of 15 - if you choose 15, you will spend 7 points and have 7 \
             left for other stats. Brains help your appraise, bombs, chemistry, craft, \
             hack, locksmith, medic, pursuade, pilot, repair, science and teach \
             skills.\n\
             \tType <green><bold>-statbot brains 8<reset><blue> (or any other \
             number) to set your brains to that number. You will be able to adjust your \
             stats by sending me the new value, up until you leave here. Your stats now \
             are: ").to_owned() + &summary,
        StatbotState::Senses => format!(ansi!(
            "Your next job is to choose how good your senses will be. Senses help your \
             appraise, dodge, focus,{} scavenge, sneak, throw, track and whips skills.\n\
             \tType <green><bold>-statbot senses 8<reset><blue> (or any other number) to \
             set your senses to that number. You will be able to adjust your stats by \
             sending me the new value, up until you leave here. Your stats now are: {}"),
                                        if session.less_explicit_mode {
                                            ""
                                        } else { " fuck,"}, &summary),
        StatbotState::Brawn => ansi!(
            "Your next job is to choose how strong you will be. Brawn helps your \
             clubs, fists, and throw skills.\n\
             \tType <green><bold>-statbot brawn 8<reset><blue> (or any other number) to \
             set your brawn to that number. You will be able to adjust your stats by \
             sending me the new value, up until you leave here. Your stats now are: "
        ).to_owned() + &summary,
        StatbotState::Reflexes => ansi!(
            "Your next job is to choose how quick your reflexes will be. Reflexes help \
             your blades, climb, clubs, dodge, locksmith, pilot, pistols, quickdraw, \
             rifles, spears, and whips skills.\n\
             \tType <green><bold>-statbot reflexes 8<reset><blue> (or any other number) to \
             set your reflexes to that number. You will be able to adjust your stats by \
             sending me the new value, up until you leave here. Your stats now are: "
        ).to_owned() + &summary,
        StatbotState::Endurance => format!(ansi!(
            "Your next job is to choose how much stamina you will have. Endurance helps \
             your climb, fish, fists, focus,{} scavenge, spears and swim skills.\n\
             \tType <green><bold>-statbot endurance 8<reset><blue> (or any other number) to \
             set your endurance to that number. You will be able to adjust your stats by \
             sending me the new value, up until you leave here. Your stats now are: {}"
        ), if session.less_explicit_mode { "" } else { " fuck,"}, &summary),
        StatbotState::Cool => ansi!(
            "Your next job is to choose how much you keep your cool under pressure. \
             Cool helps your blades, bombs, fish, pistols, quickdraw, rifles and sneak \
             skills.\n\
             \tType <green><bold>-statbot cool 8<reset><blue> (or any other number) to \
             set your cool to that number. You will be able to adjust your stats by \
             sending me the new value, up until you leave here. Your stats now are: "
        ).to_owned() + &summary,
        StatbotState::FixTotals => ansi!(
            "You haven't allocated all 14 points above 8 to stats, but you've already \
             set up all your raw stats. You'll need to go back and allocate more points \
             to one or more of them. Type:\n\
             \t<green><bold>-statbot <lt>skill> <lt>number><reset><blue>, where <lt>skill> \
             is one of brains, senses, brawn, reflexes, endurance or cool, and number is \
             between 8 and 15.").to_owned() + &summary,
        StatbotState::AssignSex => ansi!(
            "Now it is time to pick the biological sex of your new body. We have male and female bodies \
             available. Type: \n\
             \t<green><bold>-statbot sex male<reset><blue> or\n\
             \t<green><bold>-statbot sex female<reset><blue> to pick what you want \
             to be!").to_owned(),
        StatbotState::SetDescription => ansi!(
            "One last thing... write a short description (40-255 characters) to tell \
             people what your new body looks like. You can change this later even after \
             leaving here. Type: \n\
             \t<green><bold>describe me as Some text here<reset><blue>\n\
             Once you have done that, you may leave to the east to attach to your new \
             body by typing: <green><bold>east<reset><blue> or <green><bold>e<reset><blue>"
        ).to_owned(),
        StatbotState::Done => ansi!(
            "Don't let me detain you... you are free to leave. Type: \n\
            \t<green><bold>east<reset><blue> or <green><bold>e<reset><blue>\n\
            to head off to the east."
        ).to_owned()
    }
}

async fn stat_command(ctx: &mut VerbContext<'_>, item: &Item,
                      stat: &StatType, arg: &str) -> UResult<()> {
    match u8::<&str, nom::error::Error<&str>>(arg) {
        Err(_) => { reply(ctx, "I'll need a number after the stat name.").await?; }
        Ok((rest, _)) if rest.trim() != "" => {
            reply(ctx, "SYNTAX ERROR - who dares to put extra text after the stat number!").await?;
        }
        Ok((_, statno)) if statno < 8 => {
            reply(ctx, "8 is the minimum, you can't go lower").await?;
        }
        Ok((_, statno)) if statno > 15 => {
            reply(ctx, "15 is the maximum, you can't go higher even if you have points").await?;
        }
        Ok((_, statno)) => {
            let points = {
                let user = get_user_or_fail(ctx)?;
                points_left(get_user_or_fail(ctx)?) + (user.raw_stats.get(stat).cloned().unwrap_or(8) - 8)
            };
            if (statno - 8) as u16 > points {
                reply(ctx, &if points == 0 { "You have no points left".to_owned() } else {
                    format!("You only have {} point{} left", points, if points == 1 { "" } else { "s" })
                }).await?;
                return Ok(());
            }
            {
                let user_mut = get_user_or_fail_mut(ctx)?;
                user_mut.raw_stats.insert(stat.clone(), statno as u16);
                user_mut.statbot = Some(work_out_state(user_mut, item));
            }
            let user: &User = get_user_or_fail(ctx)?;
            ctx.trans.save_user_model(user).await?;
            let mut item_updated = item.clone();
            item_updated.total_stats = user.raw_stats.clone();
            ctx.trans.save_item_model(&item_updated).await?;
            reply(ctx, &next_action_text(&ctx.session_dat, user)).await?;
        }
    }

    Ok(())
}

async fn sex_command(ctx: &mut VerbContext<'_>, item: &Item,
                     arg: &str) -> UResult<()> {
    let choice = match arg.trim().to_lowercase().as_str() {
        "male" | "man" => Sex::Male,
        "female" | "woman" => Sex::Female,
        _ => {
            reply(ctx, "You want to be a what? The empire values all its subjects, \
                        but the body factory makes only male and female. Pick one \
                        of those.").await?;
            return Ok(());
        }
    };
    let mut item_updated = item.clone();
    item_updated.pronouns = match choice {
        Sex::Male => Pronouns::default_male(),
        Sex::Female => Pronouns::default_female()
    };
    item_updated.sex = Some(choice);
    {
        let user_mut = get_user_or_fail_mut(ctx)?;
        user_mut.statbot = Some(work_out_state(user_mut, &item_updated));
    }
    let user: &User = get_user_or_fail(ctx)?;
    ctx.trans.save_user_model(user).await?;
    ctx.trans.save_item_model(&item_updated).await?;
    reply(ctx, &next_action_text(&ctx.session_dat, user)).await?;
    Ok(())
}

#[async_trait]
impl NPCMessageHandler for StatbotMessageHandler {
    async fn handle(
        self: &Self,
        ctx: &mut VerbContext,
        source: &Item,
        _target: &Item,
        message: &str
    ) -> UResult<()> {
        let (command, arg) = parse_to_space(message);
        match command.to_lowercase().as_str() {
            "brains" | "brn" | "brain" => stat_command(ctx, source, &StatType::Brains, &arg).await?,
            "senses" | "sen" | "sense" => stat_command(ctx, source, &StatType::Senses, &arg).await?,
            "brawn" | "brw" => stat_command(ctx, source, &StatType::Brawn, &arg).await?,
            "reflexes" | "ref" | "reflex" => stat_command(ctx, source, &StatType::Reflexes, &arg).await?,
            "endurance" | "end" => stat_command(ctx, source, &StatType::Endurance, &arg).await?,
            "cool" | "col" => stat_command(ctx, source, &StatType::Cool, &arg).await?,
            "sex" => sex_command(ctx, source, &arg).await?,
            _ => {
                reply(ctx, &next_action_text(&ctx.session_dat, get_user_or_fail(ctx)?)).await?;
            }
        }
        Ok(())
    }
}
