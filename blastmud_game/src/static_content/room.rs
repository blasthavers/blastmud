use super::StaticItem;
use once_cell::sync::OnceCell;
use std::collections::BTreeMap;
use ansi::ansi;
use crate::models::item::Item;

pub struct Zone {
    pub code: &'static str,
    pub display: &'static str,
    pub outdoors: bool,
}

static STATIC_ZONE_DETAILS: OnceCell<BTreeMap<&'static str, Zone>> = OnceCell::new();
pub fn zone_details() -> &'static BTreeMap<&'static str, Zone> {
    STATIC_ZONE_DETAILS.get_or_init(
        || vec!(
            Zone { code: "melbs",
                   display: "Melbs",
                   outdoors: true },
            Zone { code: "repro_xv",
                   display: "Reprolabs XV",
                   outdoors: true },
        ).into_iter().map(|x|(x.code, x)).collect())
}

pub struct GridCoords {
    x: i64,
    y: i64,
    z: i64
}

pub enum ExitType {
    Free, // Anyone can just walk it.
    // Future ideas: Doors with locks, etc...
}

pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NORTHEAST,
    SOUTEAST,
    NORTHWEST,
    SOUTHWEST,
    UP,
    DOWN,
    IN(&'static str)
}

pub enum ExitTarget {
    UseGPS,
    Custom(&'static str)
}

pub struct Exit {
    direction: Direction,
    target: ExitTarget,
    exit_type: ExitType
}

pub struct Room {
    pub zone: &'static str,
    pub code: &'static str,
    pub name: &'static str,
    pub short: &'static str,
    pub grid_coords: GridCoords,
    pub description: &'static str,
    pub exits: Vec<Exit>
}

static STATIC_ROOM_LIST: OnceCell<Vec<Room>> = OnceCell::new();
pub fn room_list() -> &'static Vec<Room> {
    STATIC_ROOM_LIST.get_or_init(
        || vec!(
            Room {
                zone: "repro_xv",
                code: "repro_xv_chargen",
                name: "Choice Room",
                short: ansi!("<green>CR<reset>"),
                description: "A room brightly lit in unnaturally white light, covered in sparkling \
                              white tiles from floor to \
                              ceiling. A loudspeaker plays a message on loop:\r\n\
                              \t\"Citizen, you are here because your memory has been wiped and you \
                              are ready to start a fresh life. As a being enhanced by Gazos-Murlison \
                              Co technology, the emperor has granted you the power to choose 14 points \
                              of upgrades to yourself. Choose wisely, as it will impact who you end up \
                              being, and you would need to completely wipe your brain again to change \
                              them. Talk to Statbot to spend your 14 points and create your body.\"\r\n\
                              [Try <bold>\"statbot hi<reset>, to send hi to statbot - the \" means to \
                              whisper to a particular person in the room]",
                grid_coords: GridCoords { x: 0, y: 0, z: 2 },
                exits: vec!()
            },
        ).into_iter().collect())
}

static STATIC_ROOM_MAP_BY_CODE: OnceCell<BTreeMap<&'static str, &'static Room>> = OnceCell::new();
pub fn room_map_by_code() -> &'static BTreeMap<&'static str, &'static Room> {
    STATIC_ROOM_MAP_BY_CODE.get_or_init(
        || room_list().iter().map(|r| (r.code, r)).collect())
}

pub fn room_static_items() -> Box<dyn Iterator<Item = StaticItem>> {
    Box::new(room_list().iter().map(|r| StaticItem {
        item_code: r.code,
        initial_item: Box::new(|| Item {
            item_code: r.code.to_owned(),
            item_type: "room".to_owned(),
            display: r.description.to_owned(),
            location: r.code.to_owned(),
            is_static: true,
            ..Item::default()
        })
    }))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn room_zones_should_exist() {
        for room in room_list() {
            zone_details().get(room.zone).expect(
                &format!("zone {} for room {} should exist", room.zone, room.code));
        }
    }

    #[test]
    fn room_map_by_code_should_have_repro_xv_chargen() {
        assert_eq!(room_map_by_code().get("repro_xv_chargen").expect("repro_xv_chargen to exist").code,
                   "repro_xv_chargen");
    }
}
