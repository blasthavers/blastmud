use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use crate::regular_tasks::queued_command::QueueCommand;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Session {
    pub source: String,
    pub less_explicit_mode: bool,
    pub queue: VecDeque<QueueCommand>,
    // Reminder: Consider backwards compatibility when updating this. New fields should generally
    //           be an Option, or you should ensure the default value is sensible, or things will
    //           crash out for existing sessions.
}

impl Session {
    pub fn explicit_if_allowed<'l>(self: &Self, explicit: &'l str, non_explicit: Option<&'l str>) -> &'l str {
        if self.less_explicit_mode {
            non_explicit.unwrap_or(explicit)
        } else {
            explicit
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Session { source: "unknown".to_owned(), less_explicit_mode: false,
                  queue: VecDeque::new() }
    }
}
