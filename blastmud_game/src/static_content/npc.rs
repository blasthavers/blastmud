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
    pub description: &'static str,
    pub spawn_location: &'static str,
    pub message_handler: Option<&'static (dyn NPCMessageHandler + Sync + Send)>,
    pub says: Vec<NPCSayInfo>
}

pub fn npc_list() -> &'static Vec<NPC> {
    use NPCSayType::FromFixedList;
    static NPC_LIST: OnceCell<Vec<NPC>> = OnceCell::new();
    NPC_LIST.get_or_init(
        ||  {
            let melbs_citizen_stdsay = NPCSayInfo {
                say_code: "babble",
                frequency_secs: 60,
                talk_type: FromFixedList(vec!(
                    (false, "I'm so sick of being cloned."),
                    (false, "I hope I don't die again today."),
                    (false, "I wish the so-called king would do something about the damned zombies everywhere."),
                    (true, "I earn so many credits making babies for the body factory - it literally pays my bills."),
                    (false, "I know people hated the empire, but I kind of wish it was still intact - it was a lot better than what we have now."),
                    (false, "I wish there wasn't so much radiation outside of Melbs CBD."),
                    (false, "I heard about a guy who went to a special place somewhere around here, and there was a machine that enhanced his wristpad and gave him basically superpowers."),
                    (false, "The damn vampire movement... they are all so sneaky, and I never know when they are going to come for my blood."),
                ))
            };
            vec!(
              NPC {
                  code: "repro_xv_chargen_statbot",
                  name: "Statbot",
                  description: "A silvery shiny metal mechanical being. It lets out a whirring sound as it moves.",
                  spawn_location: "room/repro_xv_chargen",
                  message_handler: Some(&statbot::StatbotMessageHandler),
                  says: vec!()
              },
              NPC {
                  code: "melbs_citizen_1",
                  name: "Matthew Thomas",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_latrobest",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_2",
                  name: "Matthew Perez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_20",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_3",
                  name: "Kimberly Jackson",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_40",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_4",
                  name: "Michael Sanchez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_50",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_5",
                  name: "Jessica Davis",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_bourkest",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_6",
                  name: "Robert Davis",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_70",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_7",
                  name: "Paul Lewis",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_90",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_8",
                  name: "Andrew Moore",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_collinsst",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_9",
                  name: "Betty Thomas",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_100",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_10",
                  name: "Mary Robinson",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_110",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_11",
                  name: "Lisa Lopez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_flinderst",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_12",
                  name: "Kimberly Martinez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_flindersst_200",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_13",
                  name: "Anthony Nguyen",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_flindersst_190",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_14",
                  name: "Joshua Green",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_flindersst_180",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_15",
                  name: "Emily Wright",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_flindersst_170",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_16",
                  name: "Ashley Thomas",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_lonsdalest_130",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_17",
                  name: "Jessica Miller",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_80",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_18",
                  name: "Anthony Lopez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_lonsdalest_140",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_19",
                  name: "John Lopez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_elizabethst_lonsdalest",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_20",
                  name: "Thomas Garcia",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_williamsst_120",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_21",
                  name: "Donna Thompson",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_elizabethst_60",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_22",
                  name: "Matthew Davis",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_williamsst_100",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_23",
                  name: "Steven Jones",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_swanstonst_120",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_24",
                  name: "Linda Smith",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_swanstonst_lonsdalest",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_25",
                  name: "Karen Rodriguez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_bourkest_180",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_26",
                  name: "Paul Scott",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_swanstonst_70",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_27",
                  name: "Ashley Thomas",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_lonsdalest_130",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_28",
                  name: "Sandra Scott",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_elizabethst_30",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_29",
                  name: "Michael Rodriguez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_swanstonst_70",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_30",
                  name: "Donald Miller",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_elizabethst_30",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_31",
                  name: "Charles Moore",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_lonsdalest_160",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_32",
                  name: "Ashley Sanchez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_100",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_33",
                  name: "Margaret Lewis",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_flindersst_180",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_34",
                  name: "Sandra Thompson",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_swanstonst_80",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_35",
                  name: "Sandra King",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_lonsdalest_150",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_36",
                  name: "Lisa Anderson",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_lonsdalest_210",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_37",
                  name: "Kimberly Martin",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_80",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_38",
                  name: "Susan Smith",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_latrobest_190",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_39",
                  name: "Susan Martin",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_collinsst_150",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_40",
                  name: "Linda Scott",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_williamsst_30",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_41",
                  name: "Donald Miller",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_elizabethst_80",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_42",
                  name: "Mark Hill",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_collinsst_120",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_43",
                  name: "William Perez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_queenst_90",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_44",
                  name: "Donald Perez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_queenst_lonsdalest",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_45",
                  name: "Lisa Rodriguez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_collinsst_100",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_46",
                  name: "James Adams",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_latrobest_150",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_47",
                  name: "James Moore",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_latrobest_130",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_48",
                  name: "Joseph Martin",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_bourkest_150",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_49",
                  name: "Matthew Jones",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_60",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_50",
                  name: "Michael Sanchez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_queenst_100",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_51",
                  name: "Donna Torres",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_flindersst_150",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_52",
                  name: "Barbara Garcia",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_swanstonst_50",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_53",
                  name: "Daniel Miller",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_bourkest_110",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_54",
                  name: "Robert Young",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_kingst_collinsst",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_55",
                  name: "Donald Flores",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_swanstonst_40",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_56",
                  name: "Charles Thomas",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_flindersst_110",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_57",
                  name: "William Torres",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_swanstonst_60",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_58",
                  name: "Barbara Gonzalez",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_collinsst_190",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_59",
                  name: "Mary Smith",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_bourkest_180",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              },

              NPC {
                  code: "melbs_citizen_60",
                  name: "Michael Jackson",
                  description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
                  spawn_location: "room/melbs_williamsst_110",
                  message_handler: None,
                  says: vec!(melbs_citizen_stdsay.clone())
              }
            )
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
