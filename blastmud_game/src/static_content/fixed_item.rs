// For things like signs that don't do much except stay where they are and carry a description.
use super::StaticItem;
use once_cell::sync::OnceCell;
use crate::models::item::{Item, Pronouns};
use ansi::ansi;

pub struct FixedItem {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub description_less_explicit: Option<&'static str>,
    pub location: &'static str,
    pub proper_noun: bool
}

fn fixed_item_list() -> &'static Vec<FixedItem> {
    static FIXED_ITEM_LIST: OnceCell<Vec<FixedItem>> = OnceCell::new();
    FIXED_ITEM_LIST.get_or_init(|| vec!(
        FixedItem {
            code: "repro_xv_updates_red_poster",
            name: ansi!("red poster"),
            description:
              "A larger faded poster with a thick red border. It says:\n\
               \t\"Dear newly memory wiped citizen! Welcome to Melbs! A lot \
               has changed since the memories your implant is based on were \
               created. The global Gazos-Murlison Co empire fell in a nuclear \
               attack, and most cities of the world were destroyed. \
               A few cities around Australia, like this one, took some fallout \
               but survived. The few remaining cities are now all independently \
               run. I was a young governor under the empire, and I now rule inner \
               Melbs as the King. I have gotten all the fallout out from the inner city, \
               and I have a robot police force to keep you safe from the worst baddies, \
               but be warned - there still are some dangers near by, and the world \
               further out, outside my realm, is a dangerous and radioactive place.\"",
            description_less_explicit: None,
            location: "room/repro_xv_updates",
            proper_noun: false
        }
    ))
}

pub fn static_items() -> Box<dyn Iterator<Item = StaticItem>> {
    Box::new(fixed_item_list().iter().map(|r| StaticItem {
        item_code: r.code,
        initial_item: Box::new(|| Item {
            item_code: r.code.to_owned(),
            item_type: "fixed_item".to_owned(),
            display: r.name.to_owned(),
            details: Some(r.description.to_owned()),
            details_less_explicit: r.description_less_explicit.map(|d|d.to_owned()),
            location: r.location.to_owned(),
            is_static: true,
            pronouns: Pronouns {
                is_proper: r.proper_noun,
                ..Pronouns::default_inanimate()
            },
            ..Item::default()
        })
    }))
}
