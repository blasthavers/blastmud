use super::{StaticItem, StaticTask};
use crate::models::{
    item::Item,
    task::{Task, TaskMeta, TaskRecurrence, TaskDetails}
};
use once_cell::sync::OnceCell;
use std::collections::BTreeMap;
use crate::message_handler::user_commands::{
    VerbContext, UResult, CommandHandlingError,
    say::say_to_room
};
use crate::DResult;
use async_trait::async_trait;
use chrono::Utc;
use rand::{thread_rng, Rng, prelude::*};
use crate::regular_tasks::{TaskHandler, TaskRunContext};
use log::info;
use std::time;

pub mod statbot;
mod melbs_citizen;
mod melbs_dog;

#[async_trait]
pub trait NPCMessageHandler {
    async fn handle(
        self: &Self,
        ctx: &mut VerbContext,
        source: &Item,
        target: &Item,
        message: &str
    ) -> UResult<()>;
}

#[derive(Clone, Debug)]
pub enum NPCSayType {
    // Bool is true if it should be filtered for less-explicit.
    FromFixedList(Vec<(bool, &'static str)>)
}

#[derive(Clone, Debug)]
pub struct NPCSayInfo {
    pub say_code: &'static str,
    pub frequency_secs: u64,
    pub talk_type: NPCSayType
}

pub struct NPC {
    pub code: &'static str,
    pub name: &'static str,
    pub proper_noun: bool,
    pub description: &'static str,
    pub spawn_location: &'static str,
    pub message_handler: Option<&'static (dyn NPCMessageHandler + Sync + Send)>,
    pub says: Vec<NPCSayInfo>
}

impl Default for NPC {
    fn default() -> Self {
        Self {
            code: "DEFAULT",
            name: "default",
            proper_noun: true,
            description: "default",
            spawn_location: "default",
            message_handler: None,
            says: vec!()
        }
    }
}

pub fn npc_list() -> &'static Vec<NPC> {
    static NPC_LIST: OnceCell<Vec<NPC>> = OnceCell::new();
    NPC_LIST.get_or_init(
        || {
            let mut npcs = vec!(
              NPC {
                  code: "repro_xv_chargen_statbot",
                  name: "Statbot",
                  description: "A silvery shiny metal mechanical being. It lets out a whirring sound as it moves.",
                  spawn_location: "room/repro_xv_chargen",
                  message_handler: Some(&statbot::StatbotMessageHandler),
                  says: vec!(),
                  ..Default::default()
              },
            );
            npcs.append(&mut melbs_citizen::npc_list());
            npcs.append(&mut melbs_dog::npc_list());
            npcs
        })
}

pub fn npc_by_code() -> &'static BTreeMap<&'static str, &'static NPC> {
    static NPC_CODE_MAP: OnceCell<BTreeMap<&'static str, &'static NPC>> = OnceCell::new();
    NPC_CODE_MAP.get_or_init(
        || npc_list().iter()
            .map(|npc| (npc.code, npc))
            .collect())
}

pub fn npc_say_info_by_npc_code_say_code() -> &'static BTreeMap<(&'static str, &'static str),
                                                                &'static NPCSayInfo> {
    static NPC_SAYINFO_MAP: OnceCell<BTreeMap<(&'static str, &'static str),
                                              &'static NPCSayInfo>> = OnceCell::new();
    NPC_SAYINFO_MAP.get_or_init(
        || npc_list().iter().flat_map(
            |npc| npc.says.iter().map(
                |says| ((npc.code, says.say_code), says)
            )
        ).collect())
}

pub fn npc_static_items() -> Box<dyn Iterator<Item = StaticItem>> {
    Box::new(npc_list().iter().map(|c| StaticItem {
        item_code: c.code,
        initial_item: Box::new(|| Item {
            item_code: c.code.to_owned(),
            item_type: "npc".to_owned(),
            display: c.name.to_owned(),
            details: Some(c.description.to_owned()),
            location: c.spawn_location.to_owned(),
            is_static: true,
            ..Item::default()
        })
    }))
}

pub fn npc_say_tasks() -> Box<dyn Iterator<Item = StaticTask>> {
    Box::new(npc_list().iter().flat_map(|c| c.says.iter().map(|say| StaticTask {
        task_code: c.code.to_owned() + "_" + say.say_code,
        initial_task: Box::new(
            || {
                let mut rng = thread_rng();
                Task {
                    meta: TaskMeta {
                        task_code: c.code.to_owned() + "_" + say.say_code,
                        is_static: true,
                        recurrence: Some(TaskRecurrence::FixedDuration { seconds: say.frequency_secs as u32 }),
                        next_scheduled: Utc::now() + chrono::Duration::seconds(rng.gen_range(0..say.frequency_secs) as i64),
                        ..TaskMeta::default()
                    },
                    details: TaskDetails::NPCSay {
                        npc_code: c.code.to_owned(),
                        say_code: say.say_code.to_owned()
                    },
                }
            })
    })))
}

pub struct NPCSayTaskHandler;
#[async_trait]
impl TaskHandler for NPCSayTaskHandler {
    async fn do_task(&self, ctx: &mut TaskRunContext) -> DResult<Option<time::Duration>> {
        let (npc_code, say_code) = match &ctx.task.details {
            TaskDetails::NPCSay { npc_code, say_code } => (npc_code.clone(), say_code.clone()),
            _ => Err("Expected NPC say task to be NPCSay type")?
        };

        let say_info = match npc_say_info_by_npc_code_say_code().get(&(&npc_code, &say_code)) {
            None => {
                info!("NPCSayTaskHandler can't find NPCSayInfo for npc {} say_code {}",
                      npc_code, say_code);
                return Ok(None);
            }
            Some(r) => r
        };
        let npc_item = match ctx.trans.find_item_by_type_code("npc", &npc_code).await? {
            None => {
                info!("NPCSayTaskHandler can't find NPC {}", npc_code);
                return Ok(None);
            }
            Some(r) => r
        };

        let (is_explicit, say_what) = match &say_info.talk_type {
            NPCSayType::FromFixedList(l) => {
                let mut rng = thread_rng();
                match l[..].choose(&mut rng) {
                    None => {
                        info!("NPCSayTaskHandler NPCSayInfo for npc {} say_code {} has no choices",
                              npc_code, say_code);
                        return Ok(None);
                    }
                    Some(r) => r.clone()
                }
            }
        };
        
        match say_to_room(ctx.trans, &npc_item, &npc_item.location, say_what, is_explicit).await {
            Ok(()) => {}
            Err(CommandHandlingError::UserError(e)) => {
                info!("NPCSayHandler couldn't send for npc {} say_code {}: {}",
                      npc_code, say_code, e);
            }
            Err(CommandHandlingError::SystemError(e)) => Err(e)?
        }
        Ok(None)
    }
}
pub static SAY_HANDLER: &'static (dyn TaskHandler + Sync + Send) = &NPCSayTaskHandler;
