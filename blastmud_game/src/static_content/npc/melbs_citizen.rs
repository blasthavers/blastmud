use super::{NPC, NPCSayInfo, NPCSayType};

pub fn npc_list() -> Vec<NPC> {
    use NPCSayType::FromFixedList;
    let melbs_citizen_stdsay = NPCSayInfo {
        say_code: "babble",
        frequency_secs: 60,
        talk_type: FromFixedList(vec!(
            (false, "I'm so sick of being cloned."),
            (false, "I hope I don't die again today."),
            (false, "I wish the so-called king would do something about the damned zombies everywhere."),
            (true, "I earn so many credits making babies for the body factory - it literally pays my bills."),
            (false, "I know people hated the empire, but I kind of wish it was still intact - it was a lot better than what we have now."),
            (false, "I wish there wasn't so much radiation outside of Melbs CBD."),
            (false, "I heard about a guy who went to a special place somewhere around here, and there was a machine that enhanced his wristpad and gave him basically superpowers."),
            (false, "The damn vampire movement... they are all so sneaky, and I never know when they are going to come for my blood."),
        ))
    };
       
    vec!(
      NPC {
          code: "melbs_citizen_1",
          name: "Matthew Thomas",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_latrobest",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_2",
          name: "Matthew Perez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_20",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },

      NPC {
          code: "melbs_citizen_3",
          name: "Kimberly Jackson",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_40",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_4",
          name: "Michael Sanchez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_50",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_5",
          name: "Jessica Davis",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_bourkest",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_6",
          name: "Robert Davis",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_70",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_7",
          name: "Paul Lewis",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_90",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_8",
          name: "Andrew Moore",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_collinsst",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_9",
          name: "Betty Thomas",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_100",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_10",
          name: "Mary Robinson",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_110",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_11",
          name: "Lisa Lopez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_flinderst",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_12",
          name: "Kimberly Martinez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_flindersst_200",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_13",
          name: "Anthony Nguyen",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_flindersst_190",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_14",
          name: "Joshua Green",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_flindersst_180",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_15",
          name: "Emily Wright",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_flindersst_170",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_16",
          name: "Ashley Thomas",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_lonsdalest_130",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_17",
          name: "Jessica Miller",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_80",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_18",
          name: "Anthony Lopez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_lonsdalest_140",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_19",
          name: "John Lopez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_elizabethst_lonsdalest",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_20",
          name: "Thomas Garcia",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_williamsst_120",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_21",
          name: "Donna Thompson",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_elizabethst_60",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_22",
          name: "Matthew Davis",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_williamsst_100",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_23",
          name: "Steven Jones",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_swanstonst_120",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_24",
          name: "Linda Smith",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_swanstonst_lonsdalest",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_25",
          name: "Karen Rodriguez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_bourkest_180",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_26",
          name: "Paul Scott",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_swanstonst_70",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_27",
          name: "Ashley Thomas",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_lonsdalest_130",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_28",
          name: "Sandra Scott",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_elizabethst_30",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_29",
          name: "Michael Rodriguez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_swanstonst_70",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_30",
          name: "Donald Miller",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_elizabethst_30",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_31",
          name: "Charles Moore",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_lonsdalest_160",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_32",
          name: "Ashley Sanchez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_100",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_33",
          name: "Margaret Lewis",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_flindersst_180",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_34",
          name: "Sandra Thompson",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_swanstonst_80",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_35",
          name: "Sandra King",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_lonsdalest_150",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_36",
          name: "Lisa Anderson",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_lonsdalest_210",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_37",
          name: "Kimberly Martin",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_80",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_38",
          name: "Susan Smith",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_latrobest_190",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_39",
          name: "Susan Martin",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_collinsst_150",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_40",
          name: "Linda Scott",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_williamsst_30",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_41",
          name: "Donald Miller",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_elizabethst_80",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_42",
          name: "Mark Hill",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_collinsst_120",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_43",
          name: "William Perez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_queenst_90",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_44",
          name: "Donald Perez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_queenst_lonsdalest",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_45",
          name: "Lisa Rodriguez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_collinsst_100",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_46",
          name: "James Adams",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_latrobest_150",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_47",
          name: "James Moore",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_latrobest_130",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_48",
          name: "Joseph Martin",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_bourkest_150",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_49",
          name: "Matthew Jones",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_60",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_50",
          name: "Michael Sanchez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_queenst_100",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_51",
          name: "Donna Torres",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_flindersst_150",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_52",
          name: "Barbara Garcia",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_swanstonst_50",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_53",
          name: "Daniel Miller",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_bourkest_110",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_54",
          name: "Robert Young",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_kingst_collinsst",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_55",
          name: "Donald Flores",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_swanstonst_40",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_56",
          name: "Charles Thomas",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_flindersst_110",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_57",
          name: "William Torres",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_swanstonst_60",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_58",
          name: "Barbara Gonzalez",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_collinsst_190",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_59",
          name: "Mary Smith",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_bourkest_180",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      },
      NPC {
          code: "melbs_citizen_60",
          name: "Michael Jackson",
          description: "A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life",
          spawn_location: "room/melbs_williamsst_110",
          message_handler: None,
          says: vec!(melbs_citizen_stdsay.clone()),
          ..Default::default()
      }
    )
}
