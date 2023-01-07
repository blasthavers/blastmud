use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use super::{user::{SkillType, StatType}, session::Session};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BuffCause {
    WaitingTask { task_code: String, task_type: String },
    ByItem { item_code: String, item_type: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum BuffImpact {
    ChangeStat { stat: StatType, magnitude: i16 },
    ChangeSkill { stat: StatType, magnitude: i16 }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Buff {
    description: String,
    cause: BuffCause,
    impacts: Vec<BuffImpact>
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pronouns {
    pub subject: String,
    pub object: String,
    pub intensive: String,
    pub possessive: String,
    // And some miscellaneous details to determine context
    pub is_plural: bool, // ... are instead of ... is
    pub is_proper: bool, // When naming, just ... instead of The ...
}

impl Pronouns {
    pub fn default_inanimate() -> Pronouns {
        Pronouns {
            subject: "it".to_owned(),
            object: "it".to_owned(),
            intensive: "itself".to_owned(),
            possessive: "its".to_owned(),
            is_plural: false,
            is_proper: true,
        }
    }

    pub fn default_animate() -> Pronouns {
        Pronouns {
            subject: "they".to_owned(),
            object: "them".to_owned(),
            intensive: "themselves".to_owned(),
            possessive: "their".to_owned(),
            is_plural: true,
            is_proper: true,
        }
    }
    
    #[allow(dead_code)]
    pub fn default_male() -> Pronouns {
        Pronouns {
            subject: "he".to_owned(),
            object: "him".to_owned(),
            intensive: "himself".to_owned(),
            possessive: "his".to_owned(),
            is_plural: false,
            is_proper: true,
        }
    }


    #[allow(dead_code)]
    pub fn default_female() -> Pronouns {
        Pronouns {
            subject: "she".to_owned(),
            object: "her".to_owned(),
            intensive: "herself".to_owned(),
            possessive: "her".to_owned(),
            is_plural: false,
            is_proper: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Subattack {
    Normal,
    Powerattacking,
    Feinting,
    Grabbing,
    Wrestling
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LocationActionType {
    Normal,
    Sitting,
    Reclining,
    Worn, // Clothing etc...
    Wielded,
    Attacking(Subattack),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ItemFlag {
    NoSay,
    NoSeeContents
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(default)]
pub struct Item {
    pub item_code: String,
    pub item_type: String,
    pub display: String,
    pub display_less_explicit: Option<String>,
    pub details: Option<String>,
    pub details_less_explicit: Option<String>,
    pub location: String, // Item reference as item_type/item_code.
    pub action_type: LocationActionType,
    pub presence_target: Option<String>, // e.g. what are they sitting on.
    pub is_static: bool,

    pub total_xp: u64,
    pub total_stats: BTreeMap<StatType, u16>,
    pub total_skills: BTreeMap<SkillType, u16>,
    pub temporary_buffs: Vec<Buff>,
    pub pronouns: Pronouns,
    pub flags: Vec<ItemFlag>,
    pub sex: Option<Sex>,
}

impl Item {
    pub fn display_for_session<'l>(self: &'l Self, session: &Session) -> &'l str {
        session.explicit_if_allowed(&self.display,
                                    self.display_less_explicit.as_ref().map(String::as_str))
    }

    pub fn details_for_session<'l>(self: &'l Self, session: &Session) -> Option<&'l str>{
        self.details.as_ref()
            .map(|dets|
                 session.explicit_if_allowed(
                     dets.as_str(),
                     self.details_less_explicit.as_ref().map(String::as_str)
                 )
            )
    }
}

impl Default for Item {
    fn default() -> Self {
        Item {
            item_code: "unset".to_owned(),
            item_type: "unset".to_owned(),
            display: "Item".to_owned(),
            display_less_explicit: None,
            details: None,
            details_less_explicit: None,
            location: "room/storage".to_owned(),
            action_type: LocationActionType::Normal,
            presence_target: None,
            is_static: false,
            total_xp: 0,
            total_stats: BTreeMap::new(),
            total_skills: BTreeMap::new(),
            temporary_buffs: Vec::new(),
            pronouns: Pronouns::default_inanimate(),
            flags: vec!(),
            sex: None
        }
    }
}
