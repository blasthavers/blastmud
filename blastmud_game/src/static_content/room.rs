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

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct GridCoords {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl GridCoords {
    pub fn apply(self: &GridCoords, dir: &Direction) -> GridCoords {
        match dir {
            Direction::NORTH => GridCoords {y: self.y - 1, ..*self},
            Direction::SOUTH => GridCoords {y: self.y + 1, ..*self},
            Direction::EAST => GridCoords {x: self.x + 1, ..*self},
            Direction::WEST => GridCoords {x: self.x - 1, ..*self},
            Direction::NORTHEAST => GridCoords {x: self.x + 1, y: self.y - 1, ..*self},
            Direction::SOUTHEAST => GridCoords {x: self.x + 1, y: self.y + 1, ..*self},
            Direction::NORTHWEST => GridCoords {x: self.x - 1, y: self.y - 1, ..*self},
            Direction::SOUTHWEST => GridCoords {x: self.x - 1, y: self.y + 1, ..*self},
            Direction::UP => GridCoords {z: self.z + 1, ..*self},
            Direction::DOWN => GridCoords {z: self.z - 1, ..*self},
            Direction::IN(_) => self.clone()
        }
    }
}

pub enum ExitType {
    Free, // Anyone can just walk it.
    // Future ideas: Doors with locks, etc...
}

#[allow(dead_code)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NORTHEAST,
    SOUTHEAST,
    NORTHWEST,
    SOUTHWEST,
    UP,
    DOWN,
    IN(&'static str)
}

impl Direction {
    pub fn describe(self: &Self) -> &'static str {
        match self {
            Direction::NORTH => "north",
            Direction::SOUTH => "south",
            Direction::EAST => "east",
            Direction::WEST => "west",
            Direction::NORTHEAST => "northeast",
            Direction::SOUTHEAST => "southeast",
            Direction::NORTHWEST => "northwest",
            Direction::SOUTHWEST => "southwest",
            Direction::UP => "up",
            Direction::DOWN => "down",
            Direction::IN(s) => s
        }
    }

    pub fn parse(input: &str) -> Option<&'static Direction> {
        match input {
            "north" | "n" => Some(&Direction::NORTH),
            "south" | "s" => Some(&Direction::SOUTH),
            "east" | "e" => Some(&Direction::EAST),
            "west" | "w" => Some(&Direction::WEST),
            "northeast" | "ne" => Some(&Direction::NORTHEAST),
            "southeast" | "se" => Some(&Direction::SOUTHEAST),
            "northwest" | "nw" => Some(&Direction::NORTHEAST),
            "southwest" | "sw" => Some(&Direction::SOUTHWEST),
            _ => None
        }
    }
}

pub enum ExitTarget {
    UseGPS,
    #[allow(dead_code)]
    Custom(&'static str)
}

pub struct Exit {
    pub direction: Direction,
    pub target: ExitTarget,
    pub exit_type: ExitType
}

pub struct Room {
    pub zone: &'static str,
    pub code: &'static str,
    pub name: &'static str,
    pub short: &'static str,
    pub grid_coords: GridCoords,
    pub description: &'static str,
    pub description_less_explicit: Option<&'static str>,
    pub exits: Vec<Exit>
}

static STATIC_ROOM_LIST: OnceCell<Vec<Room>> = OnceCell::new();
pub fn room_list() -> &'static Vec<Room> {
    STATIC_ROOM_LIST.get_or_init(
        || vec!(
            Room {
                zone: "repro_xv",
                code: "repro_xv_chargen",
                name: ansi!("<yellow>Choice Room<reset>"),
                short: ansi!("<bgwhite><green>CR<reset>"),
                description: ansi!(
                    "A room brightly lit in unnaturally white light, covered in sparkling \
                    white tiles from floor to ceiling. A loudspeaker plays a message on \
                    loop:\n\
                    \t<blue>\"Citizen, you are here because your memory has been wiped and \
                    you are ready to start a fresh life. As a being enhanced by \
                    Gazos-Murlison Co technology, the emperor has granted you the power \
                    to choose 14 points of upgrades to yourself. Choose wisely, as it \
                    will impact who you end up being, and you would need to completely \
                    wipe your brain again to change them. Talk to Statbot to spend your \
                    14 points and create your body.\"<reset>\n\
                    [Try <bold>-statbot hi<reset>, to send hi to statbot - the - means \
                    to whisper to a particular person in the room]"),
                description_less_explicit: None,
                grid_coords: GridCoords { x: 0, y: 0, z: 1 },
                exits: vec!(Exit {
                    direction: Direction::EAST,
                    target: ExitTarget::UseGPS,
                    exit_type: ExitType::Free,
                })
            },
            Room {
                zone: "repro_xv",
                code: "repro_xv_respawn",
                name: ansi!("<yellow>Body Factory<reset>"),
                short: ansi!("<bgmagenta><white>BF<reset>"),
                description: ansi!(
                    "A room filled with glass vats full of clear fluids, with bodies of \
                    various stages of development floating in them. It smells like bleach. \
                    Being here makes you realise you aren't exactly alive right now... you \
                    have no body. But you sense you could go <bold>up<reset> and attach \
                    your memories to a body matching your current stats"),
                description_less_explicit: None,
                grid_coords: GridCoords { x: 1, y: 0, z: 1 },
                exits: vec!()
            },
        ).into_iter().collect())
}

static STATIC_ROOM_MAP_BY_CODE: OnceCell<BTreeMap<&'static str, &'static Room>> = OnceCell::new();
pub fn room_map_by_code() -> &'static BTreeMap<&'static str, &'static Room> {
    STATIC_ROOM_MAP_BY_CODE.get_or_init(
        || room_list().iter().map(|r| (r.code, r)).collect())
}

static STATIC_ROOM_MAP_BY_ZLOC: OnceCell<BTreeMap<(&'static str, &'static GridCoords),
                                                  &'static Room>> = OnceCell::new();
pub fn room_map_by_zloc() -> &'static BTreeMap<(&'static str, &'static GridCoords), &'static Room> {
    STATIC_ROOM_MAP_BY_ZLOC.get_or_init(
        || room_list().iter().map(|r| ((r.zone, &r.grid_coords), r)).collect())
}

pub fn room_static_items() -> Box<dyn Iterator<Item = StaticItem>> {
    Box::new(room_list().iter().map(|r| StaticItem {
        item_code: r.code,
        initial_item: Box::new(|| Item {
            item_code: r.code.to_owned(),
            item_type: "room".to_owned(),
            display: r.name.to_owned(),
            details: Some(r.description.to_owned()),
            details_less_explicit: r.description_less_explicit.map(|d|d.to_owned()),
            location: format!("zone/{}", r.zone),
            is_static: true,
            ..Item::default()
        })
    }))
}

pub fn resolve_exit(room: &Room, exit: &Exit) -> Option<&'static Room> {
    match exit.target {
        ExitTarget::Custom(t) => t.split_once("/").and_then(
            |(t,c)|
            if t != "room" {
                None
            } else {
                room_map_by_code().get(c).map(|r|*r)
            }),
        ExitTarget::UseGPS =>
            room_map_by_zloc().get(&(room.zone, &room.grid_coords.apply(&exit.direction))).map(|r|*r)
    }
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
