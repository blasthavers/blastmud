use super::{
    Room, GridCoords, Exit, Direction, ExitTarget, ExitType,
    SecondaryZoneRecord
};
use crate::static_content::npc;
use ansi::ansi;

pub fn room_list() -> Vec<Room> {
    vec!(
        Room {
            zone: "repro_xv",
            code: "repro_xv_chargen",
            name: ansi!("Choice Room"),
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
            grid_coords: GridCoords { x: 0, y: 0, z: -1 },
            exits: vec!(Exit {
                direction: Direction::EAST,
                target: ExitTarget::UseGPS,
                exit_type: ExitType::Blocked(&npc::statbot::ChoiceRoomBlocker),
            }),
            should_caption: true,
            ..Default::default()
        },
        Room {
            zone: "repro_xv",
            code: "repro_xv_updates",
            name: ansi!("Update Centre"),
            short: ansi!("<bgwhite><green>UC<reset>"),
            description: ansi!(
                "A room covered in posters, evidently meant to help newly wiped individuals \
                get up to speed on what has happened in the world since their memory implant was \
                created. A one-way opens to the east - you have a feeling that once you go through, \
                there will be no coming back in here. <bold>[Hint: Try reading the posters here.]<reset>"),
            grid_coords: GridCoords { x: 1, y: 0, z: -1 },
            exits: vec!(Exit {
                direction: Direction::EAST,
                target: ExitTarget::UseGPS,
                exit_type: ExitType::Free,
            }),
            should_caption: true,
            ..Default::default()
        },
        Room {
            zone: "repro_xv",
            secondary_zones: vec!(),
            code: "repro_xv_respawn",
            name: ansi!("Body Factory"),
            short: ansi!("<bgmagenta><white>BF<reset>"),
            description: ansi!(
                "A room filled with glass vats full of clear fluids, with bodies of \
                various stages of development floating in them. It smells like bleach. \
                Being here makes you realise you aren't exactly alive right now... you \
                have no body. But you sense you could go <bold>up<reset> and attach \
                your memories to a body matching your current stats"),
            grid_coords: GridCoords { x: 2, y: 0, z: -1 },
            exits: vec!(Exit {
                direction: Direction::UP,
                target: ExitTarget::UseGPS,
                exit_type: ExitType::Free
            }),
            should_caption: true,
            ..Default::default()
        },
        Room {
            zone: "repro_xv",
            secondary_zones: vec!(SecondaryZoneRecord {
                zone: "melbs",
                short: ansi!("<bgmagenta><white>RL<reset>"),
                grid_coords: GridCoords { x: 2, y: 1, z: 0 },
                caption: Some("ReproLabs")
            }),
            code: "repro_xv_lobby",
            name: "Lobby",
            short: "<=",
            description: ansi!(
                "An entrance for an establishment called ReproLabs XV. \
                It looks like they make bodies and attach peoples memories to \
                them, and allow people to reclone when they die. It has an \
                unattended reception desk, with chrome-plated letters reading \
                ReproLabs XV stuck to the wall behind it. A pipe down to into the ground \
                opens up here, but the airflow is so strong, it looks like it is out \
                only - it seems to be how newly re-cloned bodies get back into the world"),
            grid_coords: GridCoords { x: 2, y: 0, z: 0 },
            exits: vec!(
                Exit {
                    direction: Direction::WEST,
                    target: ExitTarget::Custom("room/melbs_kingst_50"),
                    exit_type: ExitType::Free
                }),
            should_caption: true,
            ..Default::default()
        }
    )
}
