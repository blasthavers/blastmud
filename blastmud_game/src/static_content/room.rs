use super::StaticItem;
use once_cell::sync::OnceCell;
use std::collections::BTreeMap;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::message_handler::user_commands::{
    UResult, VerbContext
};
use crate::models::item::{Item, ItemFlag};

mod repro_xv;
mod melbs;
mod cok_murl;

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
            Zone { code: "cok_murl",
                   display: "CoK-Murlison Complex",
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
            Direction::IN { .. } => self.clone()
        }
    }
}

#[async_trait]
pub trait ExitBlocker {
    // True if they will be allowed to pass the exit, false otherwise.
    async fn attempt_exit(
        self: &Self,
        ctx: &mut VerbContext,
        player: &Item,
        exit: &Exit
    ) -> UResult<bool>;
}

pub enum ExitType {
    Free, // Anyone can just walk it.
    Blocked(&'static (dyn ExitBlocker + Sync + Send)), // Custom code about who can pass.
    // Future ideas: Doors with locks, etc...
}

#[allow(dead_code)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize)]
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
    IN { item: String }
}

impl Direction {
    pub fn describe(self: &Self) -> String {
        match self {
            Direction::NORTH => "north".to_owned(),
            Direction::SOUTH => "south".to_owned(),
            Direction::EAST => "east".to_owned(),
            Direction::WEST => "west".to_owned(),
            Direction::NORTHEAST => "northeast".to_owned(),
            Direction::SOUTHEAST => "southeast".to_owned(),
            Direction::NORTHWEST => "northwest".to_owned(),
            Direction::SOUTHWEST => "southwest".to_owned(),
            Direction::UP => "up".to_owned(),
            Direction::DOWN => "down".to_owned(),
            Direction::IN { item } => item.to_owned()
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
            "up" => Some(&Direction::UP),
            "down" => Some(&Direction::DOWN),
            _ => None
        }
    }
}

#[derive(Eq,Ord,Debug,PartialEq,PartialOrd,Clone)]
pub enum ExitTarget {
    UseGPS,
    Custom(&'static str)
}

pub struct Exit {
    pub direction: Direction,
    pub target: ExitTarget,
    pub exit_type: ExitType,
}

pub struct SecondaryZoneRecord {
    pub zone: &'static str,
    pub short: &'static str,
    pub grid_coords: GridCoords,
    pub caption: Option<&'static str>
}

pub struct Room {
    pub zone: &'static str,
    // Other zones where it can be seen on the map and accessed.
    pub secondary_zones: Vec<SecondaryZoneRecord>,
    pub code: &'static str,
    pub name: &'static str,
    pub short: &'static str,
    pub grid_coords: GridCoords,
    pub description: &'static str,
    pub description_less_explicit: Option<&'static str>,
    pub exits: Vec<Exit>,
    pub should_caption: bool,
    pub item_flags: Vec<ItemFlag>
}

impl Default for Room {
    fn default() -> Self {
        Self {
            zone: "default",
            secondary_zones: vec!(),
            code: "default",
            name: "default",
            short: "DF",
            grid_coords: GridCoords { x: 0, y: 0, z: 0 },
            description: "default",
            description_less_explicit: None,
            exits: vec!(),
            should_caption: true,
            item_flags: vec!(),
        }
    }
    
}

static STATIC_ROOM_LIST: OnceCell<Vec<Room>> = OnceCell::new();
pub fn room_list() -> &'static Vec<Room> {
    STATIC_ROOM_LIST.get_or_init(
        || {
            let mut rooms = repro_xv::room_list();
            rooms.append(&mut melbs::room_list());
            rooms.append(&mut cok_murl::room_list());
            rooms.into_iter().collect()
    })
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
        || room_list().iter()
            .map(|r| ((r.zone, &r.grid_coords), r))
            .chain(room_list().iter()
                   .flat_map(|r| r.secondary_zones.iter()
                                  .map(|sz| ((sz.zone, &sz.grid_coords), r))
                                  .collect::<Vec<((&'static str, &'static GridCoords), &'static Room)>>()))
            .collect())
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
            flags: r.item_flags.clone(),
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
    use itertools::Itertools;
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

    #[test]
    fn grid_coords_should_be_unique_in_zone() {
        let mut roomlist: Vec<&'static Room> = room_list().iter().collect();
        roomlist.sort_unstable_by(
            |a,b|
            a.grid_coords.cmp(&b.grid_coords)
                .then(a.zone.cmp(&b.zone)));
        let dups : Vec<Vec<(&'static str, &GridCoords, &'static str)>> =
            roomlist.iter()
            .group_by(|x| (&x.grid_coords, x.zone))
            .into_iter()
            .map(|((coord, zone), rg)|
                 rg.map(|r| (r.name, coord, zone))
                 .collect::<Vec<(&str, &GridCoords, &str)>>())
            .filter(|x| x.len() > 1)
            .collect();
        assert_eq!(dups,
                   Vec::<Vec<(&str, &GridCoords, &str)>>::new());
    }
    
}
