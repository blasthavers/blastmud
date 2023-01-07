use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TaskRecurrence {
    FixedDuration { seconds: u32 }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[serde(tag="task_type", content="task_details")]
pub enum TaskDetails {
    RunQueuedCommand,
    NPCSay {
        npc_code: String,
        say_code: String
    }
}
impl TaskDetails {
    pub fn name(self: &Self) -> &'static str {
        use TaskDetails::*;
        match self {
            RunQueuedCommand => "RunQueuedCommand",
            NPCSay { .. } => "NPCSay",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TaskMeta {
    pub task_code: String,
    pub is_static: bool,
    pub recurrence: Option<TaskRecurrence>,
    pub consecutive_failure_count: u32,
    pub next_scheduled: DateTime<Utc>,
}

impl Default for TaskMeta {
    fn default() -> Self {
        Self {
            task_code: "unspecified".to_string(),
            is_static: false,
            recurrence: None,
            consecutive_failure_count: 0,
            next_scheduled: Utc::now() + chrono::Duration::seconds(3600)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Task {
    #[serde(flatten)]
    pub meta: TaskMeta,
    #[serde(flatten)]
    pub details: TaskDetails,
    // Be careful of backwards compatibility if you add anything new
    // (consider Option).
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TaskOther {
    #[serde(flatten)]
    pub meta: TaskMeta,
    pub task_type: String,
    pub task_details: Value
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum TaskParse {
    Known(Task),
    Unknown(TaskOther)
}
