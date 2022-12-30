use super::StaticItem;
use crate::models::item::Item;
use once_cell::sync::OnceCell;
use std::collections::BTreeMap;
use crate::message_handler::user_commands::{VerbContext, UResult};
use async_trait::async_trait;

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


pub struct NPC {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub spawn_location: &'static str,
    pub message_handler: Option<&'static (dyn NPCMessageHandler + Sync + Send)>
}

pub fn npc_list() -> &'static Vec<NPC> {
    static NPC_LIST: OnceCell<Vec<NPC>> = OnceCell::new();
    NPC_LIST.get_or_init(|| vec!(
        NPC {
            code: "repro_xv_chargen_statbot",
            name: "Statbot",
            description: "A silvery shiny metal mechanical being. It lets out a whirring sound as it moves.",
            spawn_location: "room/repro_xv_chargen",
            message_handler: Some(&statbot::StatbotMessageHandler)
        }
    ))
}

pub fn npc_by_code() -> &'static BTreeMap<&'static str, &'static NPC> {
    static NPC_CODE_MAP: OnceCell<BTreeMap<&'static str, &'static NPC>> = OnceCell::new();
    NPC_CODE_MAP.get_or_init(
        || npc_list().iter()
            .map(|npc| (npc.code, npc))
            .collect())
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
