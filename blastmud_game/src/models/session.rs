use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub source: String,
    pub less_explicit_mode: bool,
    // Reminder: Consider backwards compatibility when updating this. New fields should generally
    //           be an Option, or things will crash out for existing sessions.
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
        Session { source: "unknown".to_owned(), less_explicit_mode: false }
    }
}
