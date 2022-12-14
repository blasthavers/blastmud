use super::{
    Room, GridCoords, Exit, Direction, ExitTarget, ExitType,
    SecondaryZoneRecord
};
use crate::models::item::ItemFlag;
use ansi::ansi;

pub fn room_list() -> Vec<Room> {
    vec!(
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_latrobest",
          name: "King Street & Latrobe St",
          short: ansi!("<yellow>##<reset>"),
          description: "A wide road (5 lanes each way) intersects a narrower 3 lane road. Both have been rather poorly maintained. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false, 
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_10",
          name: "King Street - 10 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: -4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_20",
          name: "King Street - 20 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: -3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_30",
          name: "King Street - 30 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: -2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_lonsdalest",
          name: "King Street & Lonsdale St",
          short: ansi!("<yellow>##<reset>"),
          description: "A wide road (5 lanes each way) intersects a narrower 2 lane each way road. Both have been rather poorly maintained. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_40",
          name: ansi!("King Street - 40 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 0, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_homelessshelter",
          name: ansi!("Homeless Shelter"),
          short: ansi!("<bgwhite><red>HS<reset>"),
          description: ansi!(
              "A spartan room with row after row of plain beds. A thick mist from a fog machine means you can't see or hear much going on here, and no one can attack anyone. It looks like a safe place to log off if you have no better choice"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 2, y: 0, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: true,
          item_flags: vec!(ItemFlag::NoSay, ItemFlag::NoSeeContents),
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(
              SecondaryZoneRecord {
                  zone: "repro_xv",
                  short: ansi!("<bggreen><white>EX<reset>"),
                  grid_coords: GridCoords { x: 1, y: 0, z: 0 },
                  caption: Some("Melbs"),
              }
          ),
          code: "melbs_kingst_50",
          name: ansi!("King Street - 50 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::Custom("room/repro_xv_lobby"),
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_60",
          name: "King Street - 60 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_bourkest",
          name: "King Street & Bourke St",
          short: ansi!("<yellow>##<reset>"),
          description: "A wide road (5 lanes each way) intersects a slightly narrower 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_70",
          name: "King Street - 70 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(
              SecondaryZoneRecord {
                  zone: "cok_murl",
                  short: ansi!("<yellow>||<reset>"),
                  grid_coords: GridCoords { x: -1, y: 0, z: 0 },
                  caption: Some("King Street")
              }
          ),
          code: "melbs_kingst_80",
          name: "King Street - 80 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::Custom("room/cok_lobby"),
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_90",
          name: "King Street - 90 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 6, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_collinsst",
          name: "King Street & Collins St",
          short: ansi!("<yellow>##<reset>"),
          description: "A wide road (5 lanes each way) intersects another wide 4-lane road. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_100",
          name: "King Street - 100 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 8, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_110",
          name: "King Street - 110 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 9, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_120",
          name: "King Street - 120 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 10, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_kingst_flinderst",
          name: "King Street & Flinders St",
          short: ansi!("<yellow>##<reset>"),
          description: "A wide road (5 lanes each way) intersects a wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 1, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_210",
          name: "Flinders St - 210 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 2, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_200",
          name: "Flinders St - 200 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 3, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_190",
          name: "Flinders St - 190 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 4, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_flindersst",
          name: "Williams St & Flinders St",
          short: ansi!("<yellow>##<reset>"),
          description: "An intersection of a steep asphalt road with a wide road with rusted tram tracks in the middle. Potholes dot the road surfaces",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_180",
          name: "Flinders St - 180 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 6, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_170",
          name: "Flinders St - 170 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 7, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_160",
          name: "Flinders St - 160 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 8, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_flindersst",
          name: "Queen St & Flinders St",
          short: ansi!("<yellow>##<reset>"),
          description: "A wide road with rusted tram tracks in the middle intersects another wide road. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_150",
          name: "Flinders St - 150 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 10, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_140",
          name: "Flinders St - 140 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 11, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_130",
          name: "Flinders St - 130 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 12, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_flindersst",
          name: "Elizabeth St & Flinders St",
          short: ansi!("<yellow>##<reset>"),
          description: "A wide road with rusted tram tracks in the middle intersects a wide road stained from years of neglect. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_120",
          name: "Flinders St - 120 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 14, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_110",
          name: "Flinders St - 110 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 15, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_flindersst_100",
          name: "Flinders St - 100 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A wide road with rusted tram tracks in the middle. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 16, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_flindersst",
          name: "Swanston St & Flinders St",
          short: ansi!("<yellow>##<reset>"),
          description: "The intersection of two wide roads, with rusted tram tracks and infrastructure in the middle. Crumbling bollards line all corners of the intersection, and potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 11, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_latrobest",
          name: "Swanston Street & Latrobe St",
          short: ansi!("<yellow>##<reset>"),
          description: "A dilapidated major tram thoroughfare intersects a narrower 3 lane road. Both have been rather poorly maintained. Potholes dot the ashphalt road",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swansonst_10",
          name: "Swanston Street - 10 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: -4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_20",
          name: "Swanston Street - 20 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: -3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_30",
          name: "Swanston Street - 30 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: -2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_lonsdalest",
          name: "Swanston Street & Lonsdale St",
          short: ansi!("<yellow>##<reset>"),
          description: "A dilapidated major tram thoroughfare intersects a narrower 2 lane each way road. Both have been rather poorly maintained. Potholes dot the ashphalt and weeds poke out from cracks in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_40",
          name: ansi!("Swanston Street - 40 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A wide road (5 lanes each way) that has been rather poorly maintained. Potholes dot the ashphalt road, while cracks line the footpaths on either side"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 0, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_50",
          name: ansi!("Swanston Street - 50 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_60",
          name: "Swanston Street - 60 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_bourkest",
          name: "Swanston Street & Bourke St",
          short: ansi!("<yellow>##<reset>"),
          description: "A dilapidated major tram thoroughfare intersects a slightly narrower 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt and weeds poke out from cracks in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_70",
          name: "Swanston Street - 70 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_80",
          name: "Swanston Street - 80 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_90",
          name: "Swanston Street - 90 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 6, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_collinsst",
          name: "Swanston Street & Collins St",
          short: ansi!("<yellow>##<reset>"),
          description: "A dilapidated major tram thoroughfare intersects another wide 4-lane road. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_100",
          name: "Swanston Street - 100 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 8, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_110",
          name: "Swanston Street - 110 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 9, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_swanstonst_120",
          name: "Swanston Street - 120 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A road that looks to have been a major tram thoroughfare before the collapse. Cracks line the filthy concrete footpaths and rusted tram tracks, and weeds poke out from holes in the concrete",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 17, y: 10, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_210",
          name: "La Trobe St - 210 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 2, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobesst_200",
          name: "La Trobe St - 200 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 3, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_190",
          name: "La Trobe St - 190 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 4, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamstlatrobest",
          name: "Williams St & La Trobe St",
          short: ansi!("<yellow>##<reset>"),
          description: "An intersection of an overgrown weedy road with a wide road with rusted tram tracks in the middle. Potholes dot the visible road surfaces",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_180",
          name: "La Trobe St - 180 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 6, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_170",
          name: "La Trobe St - 170 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 7, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_160",
          name: "La Trobe St - 160 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 8, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_latrobest",
          name: "Queen St & La Trobe St",
          short: ansi!("<yellow>##<reset>"),
          description: "Two relatively wide roads intersects; the road running east to west is overgrown with weeds, while the road running to the south has been kept slightly clearer",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_150",
          name: "La Trobe St - 150 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 10, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_140",
          name: "La Trobe St - 140 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 11, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_130",
          name: "La Trobe St - 130 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 12, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_latrobest",
          name: "Elizabeth St & La Trobe St",
          short: ansi!("<yellow>##<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds intersects a wide road stained from years of neglect",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_120",
          name: "La Trobe St - 120 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 14, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_110",
          name: "La Trobe St - 110 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 15, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_latrobest_100",
          name: "La Trobe St - 100 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A moderately wide road that is now overgrown and completely covered in weeds",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 16, y: -5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_210",
          name: "Lonsdale St - 210 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 2, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_200",
          name: "Lonsdale St - 200 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 3, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_190",
          name: "Lonsdale St - 190 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 4, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamstlonsdalest",
          name: "Williams St & Lonsdale St",
          short: ansi!("<yellow>##<reset>"),
          description: "An intersection of two moderately wide roads, both poorly maintained",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_180",
          name: "Lonsdale St - 180 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 6, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_170",
          name: "Lonsdale St - 170 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 7, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_160",
          name: "Lonsdale St - 160 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 8, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_lonsdalest",
          name: "Queen St & Lonsdale St",
          short: ansi!("<yellow>##<reset>"),
          description: "A relatively wide roads intersects a narrower road; both roads are littered with potholes in which weeds have set root",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_150",
          name: "Lonsdale St - 150 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 10, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_140",
          name: "Lonsdale St - 140 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 11, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_130",
          name: "Lonsdale St - 130 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 12, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_lonsdalest",
          name: "Elizabeth St & Lonsdale St",
          short: ansi!("<yellow>##<reset>"),
          description: "A pot-holded two-lane each way road intersects a wide road stained from years of neglect",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_120",
          name: "Lonsdale St - 120 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 14, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_110",
          name: "Lonsdale St - 110 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 15, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_lonsdalest_100",
          name: "Lonsdale St - 100 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A two-lane each way road that has been rather poorly maintained. Potholes dot the ashphalt and cracks line the footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 16, y: -1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_10",
          name: "Williams St - 10 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: -4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_20",
          name: "Williams St - 20 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: -3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_30",
          name: "Williams St - 30 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: -2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_40",
          name: ansi!("Williams St - 40 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A moderately wide road with a long crack in the asphalt running along its length"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 0, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_50",
          name: ansi!("Williams St - 50 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A moderately wide road with a long crack in the asphalt running along its length"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_60",
          name: "Williams St - 60 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_bourkest",
          name: "Williams St & Bourke St",
          short: ansi!("<yellow>##<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length intersects a 4-lane road with wide but heavily cracked concrete footpaths",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_70",
          name: "Williams St - 70 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_80",
          name: "Williams St - 80 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_90",
          name: "Williams St - 90 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 6, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_collinsst",
          name: "Williams St & Collins St",
          short: ansi!("<yellow>##<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length intersects a wide 4-lane road. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_100",
          name: "Williams St - 100 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 8, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_110",
          name: "Williams St - 110 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 9, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_williamsst_120",
          name: "Williams St - 120 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A moderately wide road with a long crack in the asphalt running along its length",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 5, y: 10, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_210",
          name: "Bourke St - 210 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 2, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_200",
          name: "Bourke St - 200 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 3, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_190",
          name: "Bourke St - 190 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 4, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_180",
          name: "Bourke St - 180 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 6, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_170",
          name: "Bourke St - 170 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 7, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_160",
          name: "Bourke St - 160 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 8, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_bourkest",
          name: "Queen St & Bourke St",
          short: ansi!("<yellow>##<reset>"),
          description: "A relatively wide roads intersects a narrower road; both roads are littered with potholes in which weeds have set root",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_150",
          name: "Bourke St - 150 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 10, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_140",
          name: "Bourke St - 140 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 11, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_130",
          name: "Bourke St - 130 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 12, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_bourkest",
          name: "Elizabeth St & Bourke St",
          short: ansi!("<yellow>##<reset>"),
          description: "A pot-holded two-lane each way road intersects a wide road stained from years of neglect",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_120",
          name: "Bourke St - 120 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 14, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_110",
          name: "Bourke St - 110 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 15, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_bourkest_100",
          name: "Bourke St - 100 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with wide but heavily cracked concrete footpaths. Potholes dot the ashphalt",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 16, y: 3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_10",
          name: "Queen St - 10 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: -4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_20",
          name: "Queen St - 20 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: -3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_30",
          name: "Queen St - 30 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: -2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_40",
          name: ansi!("Queen St - 40 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 0, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_50",
          name: ansi!("Queen St - 50 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_60",
          name: "Queen St - 60 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_70",
          name: "Queen St - 70 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_80",
          name: "Queen St - 80 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_90",
          name: "Queen St - 90 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 6, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_collinsst",
          name: "Queen St & Collins St",
          short: ansi!("<yellow>##<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic intersects a wide 4-lane road. Potholes dot the ashphalt road, while cracks line the footpaths on either side",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_100",
          name: "Queen St - 100 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 8, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_110",
          name: "Queen St - 110 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 9, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_queenst_120",
          name: "Queen St - 120 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 9, y: 10, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_210",
          name: "Collins St - 210 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 2, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_200",
          name: "Collins St - 200 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 3, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_190",
          name: "Collins St - 190 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 4, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_180",
          name: "Collins St - 180 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 6, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_170",
          name: "Collins St - 170 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 7, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_160",
          name: "Collins St - 160 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 8, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_150",
          name: "Collins St - 150 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 10, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_140",
          name: "Collins St - 140 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 11, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_130",
          name: "Collins St - 130 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 12, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_collinsst",
          name: "Elizabeth St & Collins St",
          short: ansi!("<yellow>##<reset>"),
          description: "A 4-lane road with round muddy potholes intersects a wide road stained from years of neglect",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_120",
          name: "Collins St - 120 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 14, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_110",
          name: "Collins St - 110 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 15, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_collinsst_100",
          name: "Collins St - 100 block",
          short: ansi!("<yellow>==<reset>"),
          description: "A 4-lane road with round muddy potholes marring the poorly maintained asphalt surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 16, y: 7, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::WEST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::EAST,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },

      // New content marker
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_10",
          name: "Elizabeth St - 10 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: -4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_20",
          name: "Elizabeth St - 20 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: -3, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_30",
          name: "Elizabeth St - 30 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: -2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_40",
          name: ansi!("Elizabeth St - 40 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 0, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_50",
          name: ansi!("Elizabeth St - 50 block"),
          short: ansi!("<yellow>||<reset>"),
          description: ansi!(
              "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface"),
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 1, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }, 
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_60",
          name: "Elizabeth St - 60 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 2, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_70",
          name: "Elizabeth St - 70 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 4, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_80",
          name: "Elizabeth St - 80 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 5, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_90",
          name: "Elizabeth St - 90 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 6, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_100",
          name: "Elizabeth St - 100 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 8, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_110",
          name: "Elizabeth St - 110 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A wide road stained from years of neglect. The road smells foul, and you can make out brown, white, red, and even grey stains, as well as the occasional slick from ancient oil spilled on the surface",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 9, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      },
      Room {
          zone: "melbs",
          secondary_zones: vec!(),
          code: "melbs_elizabethst_120",
          name: "Elizabeth St - 120 block",
          short: ansi!("<yellow>||<reset>"),
          description: "A fairly wide road where the surface has broken down but has been kept clear by regular foot traffic",
          description_less_explicit: None,
          grid_coords: GridCoords { x: 13, y: 10, z: 0 },
          exits: vec!(
              Exit {
                  direction: Direction::NORTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
              Exit {
                  direction: Direction::SOUTH,
                  target: ExitTarget::UseGPS,
                  exit_type: ExitType::Free
              },
          ),
          should_caption: false,
          ..Default::default()
      }
    )
}
