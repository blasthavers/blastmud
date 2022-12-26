use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use super::user::{SkillType, StatType};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BuffCause {
    WaitingTask { task_code: String, task_type: String },
    ByItem { item_code: String, item_type: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BuffImpact {
    ChangeStat { stat: StatType, magnitude: i16 },
    ChangeSkill { stat: StatType, magnitude: i16 }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Buff {
    description: String,
    cause: BuffCause,
    impacts: Vec<BuffImpact>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Subattack {
    Normal,
    Powerattacking,
    Feinting,
    Grabbing,
    Wrestling
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LocationActionType {
    Normal,
    Sitting,
    Reclining,
    Worn, // Clothing etc...
    Wielded,
    Attacking(Subattack),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub item_code: String,
    pub item_type: String,
    pub display: String,
    pub display_less_explicit: Option<String>,
    pub location: String, // Item reference as item_type/item_code.
    pub action_type: LocationActionType,
    pub presence_target: Option<String>, // e.g. what are they sitting on.
    pub is_static: bool,

    pub total_xp: u64,
    pub total_stats: BTreeMap<StatType, u64>,
    pub total_skills: BTreeMap<SkillType, u64>,
    pub temporary_buffs: Vec<Buff>,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            item_code: "unset".to_owned(),
            item_type: "unset".to_owned(),
            display: "Item".to_owned(),
            display_less_explicit: None,
            location: "room/storage".to_owned(),
            action_type: LocationActionType::Normal,
            presence_target: None,
            is_static: false,
            total_xp: 0,
            total_stats: BTreeMap::new(),
            total_skills: BTreeMap::new(),
            temporary_buffs: Vec::new()
        }
    }
}
