use super::{
    Room, GridCoords, Exit, Direction, ExitTarget, ExitType,
    SecondaryZoneRecord
};
use ansi::ansi;
pub fn room_list() -> Vec<Room> {
    vec!(
        // Residential
        Room {
            zone: "cok_murl",
            secondary_zones: vec!(
                SecondaryZoneRecord {
                    zone: "melbs",
                    short: ansi!("<bgyellow><black>CK<reset>"),
                    grid_coords: GridCoords { x: 2, y: 5, z: 0 },
                    caption: Some("Condos on King")
                }
            ),
            code: "cok_lobby",
            name: "Residential Lobby",
            short: ansi!("<bgyellow><black>RE<reset>"),
            description: "A sizeable lobby that looks like it is serves the dual purpose as the entrance to the residential condos and as a grand entrance to the linked Murlison Suites commercial building. It is tiled with sparkling clean bluestone tiles. Light green tinted tempered glass panels line the walls. You notice a set of sleek lifts to the south, stairs to the north, and a passage to the attached Murlison commercial building to the east",
            description_less_explicit: None,
            grid_coords: GridCoords { x: 0, y: 0, z: 0 },
            exits: vec!(
                Exit {
                    direction: Direction::WEST,
                    target: ExitTarget::Custom("room/melbs_kingst_80"),
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
                Exit {
                    direction: Direction::EAST,
                    target: ExitTarget::UseGPS,
                    exit_type: ExitType::Free
                },
            ),
            should_caption: true,
            ..Default::default()
        },
        Room {
            zone: "cok_murl",
            code: "cok_gf_lift",
            name: "Residential Lifts",
            short: ansi!("<bgyellow><black>LI<reset>"),
            description: "A set of lifts leading up to various floors",
            description_less_explicit: None,
            grid_coords: GridCoords { x: 0, y: -1, z: 0 },
            exits: vec!(
                Exit {
                    direction: Direction::SOUTH,
                    target: ExitTarget::UseGPS,
                    exit_type: ExitType::Free
                },
            ),
            should_caption: true,
            ..Default::default()
        },
        Room {
            zone: "cok_murl",
            code: "cok_gf_stairs",
            name: "Residential Stairs",
            short: ansi!("<bgyellow><black>LI<reset>"),
            description: ansi!("A set of lifts leading up to various floors. It looks like it is also possible to go down to the basement by stepping down through a trapdoor covered with tape that says <bgwhite><red>EXTREME DANGER - DO NOT ENTER<reset>"),
            description_less_explicit: None,
            grid_coords: GridCoords { x: 0, y: 1, z: 0 },
            exits: vec!(
                Exit {
                    direction: Direction::NORTH,
                    target: ExitTarget::UseGPS,
                    exit_type: ExitType::Free
                },
            ),
            should_caption: true,
            ..Default::default()
        },
        
        // Commercial
        Room {
            zone: "cok_murl",
            code: "murl_lobby",
            name: "Murlison Suites Commercial Lobby",
            short: ansi!("<bgyellow><black>ML<reset>"),
            description: "A sleek reception that could have been the bridge of a 2000s era sci-fi spaceship. Linished metal plates are lit up by ambient blue LEDs, while stone tiles cover the floor",
            description_less_explicit: None,
            grid_coords: GridCoords { x: 1, y: 0, z: 0 },
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
                Exit {
                    direction: Direction::SOUTH,
                    target: ExitTarget::UseGPS,
                    exit_type: ExitType::Free
                }
            ),
            should_caption: true,
            ..Default::default()
        },
        Room {
            zone: "cok_murl",
            code: "murl_gf_lift",
            name: "Commercial Lifts",
            short: ansi!("<bgyellow><black>LI<reset>"),
            description: "A set of lifts leading up to various floors",
            description_less_explicit: None,
            grid_coords: GridCoords { x: 1, y: -1, z: 0 },
            exits: vec!(
                Exit {
                    direction: Direction::SOUTH,
                    target: ExitTarget::UseGPS,
                    exit_type: ExitType::Free
                },
            ),
            should_caption: true,
            ..Default::default()
        },
        Room {
            zone: "cok_murl",
            code: "murl_gf_stair",
            name: "Commercial Stairs",
            short: ansi!("<bgyellow><black>>><reset>"),
            description: "A set of stairs leading up to various floors",
            description_less_explicit: None,
            grid_coords: GridCoords { x: 1, y: 1, z: 0 },
            exits: vec!(
                Exit {
                    direction: Direction::NORTH,
                    target: ExitTarget::UseGPS,
                    exit_type: ExitType::Free
                },
            ),
            should_caption: true,
            ..Default::default()
        },        
    )
}
