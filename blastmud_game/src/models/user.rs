use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTermData {
    pub accepted_terms: BTreeMap<String, DateTime<Utc>>,
    pub terms_complete: bool, // Recalculated on accept and login.
    pub last_presented_term: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserExperienceData {
    pub spent_xp: u64, // Since last chargen complete.
    pub completed_journals: BTreeMap<String, DateTime<Utc>>,
    pub xp_change_for_this_reroll: i64,
    pub crafted_items: BTreeMap<String, u64>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkillType {
    Apraise,
    Blades,
    Bombs,
    Chemistry,
    Climb,
    Clubs,
    Craft,
    Fish,
    Fists,
    Flails,
    Fuck,
    Hack,
    Locksmith,
    Medic,
    Persuade,
    Pilot,
    Pistols,
    Quickdraw,
    Repair,
    Ride,
    Rifles,
    Scavenge,
    Science,
    Sneak,
    Spears,
    Swim,
    Teach,
    Throw,
    Track,
    Wrestle,
    Whips
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatType {
    Brains,
    Senses,
    Brawn,
    Reflexes,
    Endurance,
    Cool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub password_hash: String, // bcrypted.
    pub email: String,
    pub player_item_id: i64,
    pub registered_at: Option<DateTime<Utc>>,
    pub banned_until: Option<DateTime<Utc>>,
    pub abandoned_at: Option<DateTime<Utc>>,
    pub chargen_last_completed_at: Option<DateTime<Utc>>,
    
    pub terms: UserTermData,
    pub experience: UserExperienceData,
    pub raw_skills: BTreeMap<SkillType, u16>,
    pub raw_stats: BTreeMap<StatType, u16>,
    // Reminder: Consider backwards compatibility when updating this. New fields should generally
    //           be an Option, or things will crash out for existing sessions.
}

impl Default for UserTermData {
    fn default() -> Self {
        UserTermData {
            accepted_terms: BTreeMap::new(),
            terms_complete: false,
            last_presented_term: None
        }
    }
}

impl Default for UserExperienceData {
    fn default() -> Self {
        UserExperienceData {
            spent_xp: 0,
            completed_journals: BTreeMap::new(),
            xp_change_for_this_reroll: 0,
            crafted_items: BTreeMap::new(),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            username: "unknown".to_owned(),
            password_hash: "unknown".to_owned(),
            email: "unknown".to_owned(),
            player_item_id: 0,
            registered_at: None,
            banned_until: None,
            abandoned_at: None,
            chargen_last_completed_at: None,

            terms: UserTermData::default(),
            experience: UserExperienceData::default(),
            raw_skills: BTreeMap::new(),
            raw_stats: BTreeMap::new(),
        }
    }
}
