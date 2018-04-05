use core::fmt::Write;
use libtww::game::Console;
use libtww::warping::warp::Warp;
use libtww::warping::fadeout::FadeOut;

use utils::*;
use controller;

static mut cursor: usize = 0;

static mut category_index: usize = 0;
static mut stage_index: usize = 0;
static mut room_index: u8 = 0;
static mut entrance_index: u16 = 0;
static mut layer_override: i8 = -1;

type Stage = (&'static str, &'static str);

// TODO Synchronize this with the changes again

static CATEGORIES: [&'static str; 20] = [
    "Last Warp",
    "Cavern",
    "Developer",
    "Dragon Roost Cavern",
    "Dragon Roost Island",
    "Earth Temple",
    "Forbidden Woods",
    "Forest Haven",
    "Forsaken Fortress",
    "Ganon's Tower",
    "Great Fairy",
    "Hyrule",
    "Nintendo Gallery",
    "Other",
    "Outset",
    "Savage Labyrinth",
    "Sea",
    "Tower of the Gods",
    "Wind Temple",
    "Windfall",
];

static STAGES_CAVERN: &'static [Stage; 19] = &[
    ("Bomb Island", "Cave01"),
    ("Star Island", "Cave02"),
    ("Cliff Plateau Isles", "Cave03"),
    ("Rock Spire Isle", "Cave04"),
    ("Horseshoe Island", "Cave05"),
    ("Pawprint Isle Wizzrobe", "Cave07"),
    ("Shark Island", "ITest63"),
    ("Ice Ring Isle", "MiniHyo"),
    ("Fire Mountain", "MiniKaz"),
    ("Needle Rock Isle", "SubD42"),
    ("Angular Isles", "SubD43"),
    ("Boating Course", "SubD71"),
    ("Stone Watcher Island", "TF_01"),
    ("Overlook Island", "TF_02"),
    ("Birds Peak Rock", "TF_03"),
    ("Cabana", "TF_04"),
    ("Dragon Roost Island", "TF_06"),
    ("Pawprint Isle Chuchu", "TyuTyu"),
    ("Diamond Steppe Island", "WarpD"),
];

static STAGES_DEV: &'static [Stage; 45] = &[
    ("Invisible Island", "A_nami"),
    ("E3 Forest", "A_R00"),
    ("Amos T", "Amos_T"),
    ("Wind Temple", "Cave08"),
    ("Outset Island", "DmSpot0"),
    ("E3 Boating Course", "E3ROOP"),
    ("Island With House", "Ebesso"),
    ("Pig Chamber", "H_test"),
    ("Fire Cavern With Switches", "ITest61"),
    ("Ice Ring Isle Cavern", "ITest62"),
    ("Fire Mountain Cavern", "I_SubAN"),
    ("Basic Actions", "I_TestM"),
    ("Rope Room", "I_TestR"),
    ("Bridge Room", "KATA_HB"),
    ("Large Empty Room", "KATA_RM"),
    ("Fire Mountain", "kazan"),
    ("K Test2", "K_Test2"),
    ("K Test3", "K_Test3"),
    ("K Test4", "K_Test4"),
    ("K Test5", "K_Test5"),
    ("K Test6", "K_Test6"),
    ("K Test8", "K_Test8"),
    ("K Test9", "K_Test9"),
    ("K Testa", "K_Testa"),
    ("K Testb", "K_Testb"),
    ("K Testc", "K_Testc"),
    ("K Testd", "K_Testd"),
    ("K Teste", "K_Teste"),
    ("Camera Test", "morocam"),
    ("Smoke Test Room", "Msmoke"),
    ("Headstone Island", "Mukao"),
    ("Early Pirate Ship", "Obshop"),
    ("Dev Ending", "ENDumi"),
    ("Orcas Room", "Ojhous2"),
    ("Ghost Ship 1", "PShip2"),
    ("Ghost Ship 2", "PShip3"),
    ("Ghost Ship 3", "SubD45"),
    ("Ship Control Test", "sea_E"),
    ("Stone Watcher Island Cavern", "SubD44"),
    ("Bomb Island Cavern", "SubD51"),
    ("Decorative Pedestals", "TEST"),
    ("Dark Cavern With Switches", "TF_05"),
    ("Grotto With Darknuts", "TF_07"),
    ("Tingles Room", "tincle"),
    ("Basic Island", "VrTest"),
];

static STAGES_DRAGON_ROOST_CAVERN: &'static [Stage; 3] = &[
    ("Dungeon", "M_NewD2"),
    ("Boss", "M_DragB"),
    ("Mini Boss", "M_Dra09"),
];

static STAGES_DRAGON_ROOST_ISLAND: &'static [Stage; 3] = &[
    ("Pond", "Adanmae"),
    ("Komalis Room", "Comori"),
    ("Postal Service", "Atorizk"),
];

static STAGES_EARTH_TEMPLE: &'static [Stage; 4] = &[
    ("Entrance", "Edaichi"),
    ("Temple", "M_Dai"),
    ("Boss", "M_DaiB"),
    ("Mini Boss", "M_DaiMB"),
];

static STAGES_FORBIDDEN_WOODS: &'static [Stage; 3] = &[
    ("Dungeon", "kindan"),
    ("Boss", "kinBOSS"),
    ("Mini Boss", "kinMB"),
];

static STAGES_FOREST_HAVEN: &'static [Stage; 3] = &[
    ("Potion Room", "Ocrogh"),
    ("Forest Haven", "Omori"),
    ("Makars Hiding Place", "Otkura"),
];

static STAGES_FORSAKEN_FORTRESS: &'static [Stage; 7] = &[
    ("Ganondorfs Room", "M2ganon"),
    ("FF1 Tower", "Mjtower"),
    ("FF2 Tower", "M2tower"),
    ("FF1 Interior", "majroom"),
    ("FF2 Interior", "ma2room"),
    ("FF3 Interior", "ma3room"),
    ("FF1", "MajyuE"),
];

static STAGES_GANONS_TOWER: &'static [Stage; 15] = &[
    ("Entrance", "GanonA"),
    ("Room Towards Gohma", "GanonB"),
    ("Room Towards Molgera", "GanonC"),
    ("Room Towards Kalle Demos", "GanonD"),
    ("Room Towards Jalhalla", "GanonE"),
    ("Phantom Ganons Maze", "GanonJ"),
    ("Puppet Ganon", "GanonK"),
    ("Staircase Towards Puppet Ganon", "GanonL"),
    ("Main Room", "GanonM"),
    ("Staircase To Main Room", "GanonN"),
    ("Tower", "GTower"),
    ("Gohma", "Xboss0"),
    ("Kalle Demos", "Xboss1"),
    ("Jalhalla", "Xboss2"),
    ("Molgera", "Xboss3"),
];

static STAGES_GREAT_FAIRY: &'static [Stage; 6] = &[
    ("North", "Fairy01"),
    ("East", "Fairy02"),
    ("West", "Fairy03"),
    ("Forest Of Fairies", "Fairy04"),
    ("Thorned", "Fairy05"),
    ("South", "Fairy06"),
];

static STAGES_HYRULE: &'static [Stage; 3] = &[
    ("Castle", "Hyroom"),
    ("Field", "Hyrule"),
    ("Master Sword Chamber", "kenroom"),
];

static STAGES_NINTENDO_GALLERY: &'static [Stage; 8] = &[
    ("Great Sea", "figureA"),
    ("Windfall Island", "figureB"),
    ("Outset Island", "figureC"),
    ("Forsaken Fortress", "figureD"),
    ("Secret Cavern", "figureE"),
    ("Dragon Roost Island", "figureF"),
    ("Forest Haven", "figureG"),
    ("Main Room", "Pfigure"),
];

static STAGES_OTHER: &'static [Stage; 3] = &[
    ("Name Select", "Name"),
    ("Ending", "ENDING"),
    ("Title Screen", "sea_T"),
];

static STAGES_OUTSET: &'static [Stage; 7] = &[
    ("Links House", "LinkRM"),
    ("Under Links House", "LinkUG"),
    ("Forest Of Fairies", "A_mori"),
    ("Orcas Room", "Ojhous"),
    ("Mesas House", "Omasao"),
    ("Abe And Roses House", "Onobuta"),
    ("Jabuns Room", "Pjavdou"),
];

static STAGES_SAVAGE_LABYRINTH: &'static [Stage; 4] = &[
    ("Entrance", "Cave09"),
    ("Room11", "Cave10"),
    ("Room32", "Cave11"),
    ("End", "Cave06"),
];

static STAGES_SEA: &'static [Stage; 8] = &[
    ("Sea", "sea"),
    ("Tetras Ship Inside", "Asoko"),
    ("Tetras Ship Outside", "A umikz"),
    ("Submarine Five Star Isles", "Abship"),
    ("Cabana", "Abesso"),
    ("Boating Course", "Ocean"),
    ("Ghost Ship", "PShip"),
    ("Islet Of Steel", "ShipD"),
];

static STAGES_TOWER_OF_THE_GODS: &'static [Stage; 4] = &[
    ("Dungeon", "Siren"),
    ("Boss", "SirenB"),
    ("Mini Boss", "SirenMB"),
    ("Outside", "ADMumi"),
];

static STAGES_WIND_TEMPLE: &'static [Stage; 4] = &[
    ("Entrance", "Ekaze"),
    ("Temple", "kaze"),
    ("Boss", "kazeB"),
    ("Mini Boss", "kazeMB"),
];

static STAGES_WINDFALL: &'static [Stage; 8] = &[
    ("Game Room", "Kaisen"),
    ("School Of Joy", "Nitiyou"),
    ("Bomb Shop", "Obombh"),
    ("Lenzos House", "Ocmera"),
    ("Cafe Bar", "Opub"),
    ("House Of Wealth", "Orichh"),
    ("Chu Jelly Juice Shop", "Pdrgsh"),
    ("Jail", "Pnezumi"),
];

fn get_stages_for_category(index: usize) -> &'static [(&'static str, &'static str)] {
    match index {
        1 => STAGES_CAVERN,
        2 => STAGES_DEV,
        3 => STAGES_DRAGON_ROOST_CAVERN,
        4 => STAGES_DRAGON_ROOST_ISLAND,
        5 => STAGES_EARTH_TEMPLE,
        6 => STAGES_FORBIDDEN_WOODS,
        7 => STAGES_FOREST_HAVEN,
        8 => STAGES_FORSAKEN_FORTRESS,
        9 => STAGES_GANONS_TOWER,
        10 => STAGES_GREAT_FAIRY,
        11 => STAGES_HYRULE,
        12 => STAGES_NINTENDO_GALLERY,
        13 => STAGES_OTHER,
        14 => STAGES_OUTSET,
        15 => STAGES_SAVAGE_LABYRINTH,
        16 => STAGES_SEA,
        17 => STAGES_TOWER_OF_THE_GODS,
        18 => STAGES_WIND_TEMPLE,
        19 => STAGES_WINDFALL,
        _ => unreachable!(),
    }
}

const CATEGORY_LAST_WARP: usize = 0;

pub fn transition_into() {
    let last_exit = Warp::last_exit();
    unsafe {
        if category_index == CATEGORY_LAST_WARP {
            entrance_index = last_exit.entrance.entrance;
            room_index = last_exit.entrance.room;
            layer_override = last_exit.layer_override;
        }
    }
}

pub fn render() {
    const MENU_ITEM_CATEGORY: usize = 0;
    const MENU_ITEM_STAGE: usize = 1;
    const MENU_ITEM_ENTRANCE: usize = 2;
    const MENU_ITEM_ROOM: usize = 3;
    const MENU_ITEM_LAYER_OVERRIDE: usize = 4;
    const MENU_ITEM_WARP: usize = 6;

    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Warp Menu");
    let _ = write!(lines[1].begin(), "=========");

    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    let contents = [
        "Category:       ",
        "Stage:          ",
        "Entrance:       ",
        "Room:           ",
        "Layer Override: ",
        "",
        "Warp",
    ];

    move_cursor(contents.len(), unsafe { &mut cursor });

    let last_exit = Warp::last_exit();
    let last_exit_stage = last_exit.entrance.stage_name();
    let last_exit_category = &[(last_exit_stage, last_exit_stage)];

    let stages: &[Stage] = if unsafe { category_index } == CATEGORY_LAST_WARP {
        last_exit_category
    } else {
        get_stages_for_category(unsafe { category_index })
    };
    let category_name = CATEGORIES[unsafe { category_index }];
    let (visible_stage_name, actual_stage_name) = stages[unsafe { stage_index }];

    match unsafe { cursor } {
        MENU_ITEM_CATEGORY => unsafe {
            if dpad_left && category_index > 0 {
                category_index -= 1;
                stage_index = 0;
                if category_index == CATEGORY_LAST_WARP {
                    entrance_index = last_exit.entrance.entrance;
                    room_index = last_exit.entrance.room;
                    layer_override = last_exit.layer_override;
                }
            } else if dpad_right && category_index + 1 < CATEGORIES.len() {
                category_index += 1;
                stage_index = 0;
                if category_index == CATEGORY_LAST_WARP + 1 {
                    entrance_index = 0;
                    room_index = 0;
                    layer_override = -1;
                }
            }
        },
        MENU_ITEM_STAGE => unsafe {
            if dpad_left && stage_index > 0 {
                stage_index -= 1;
            } else if dpad_right && stage_index + 1 < stages.len() {
                stage_index += 1;
            }
        },
        MENU_ITEM_ENTRANCE => unsafe {
            if dpad_left {
                entrance_index -= 1;
            } else if dpad_right {
                entrance_index += 1;
            }
        },
        MENU_ITEM_ROOM => unsafe {
            if dpad_left {
                room_index -= 1;
            } else if dpad_right {
                room_index += 1;
            }
        },
        MENU_ITEM_LAYER_OVERRIDE => unsafe {
            if dpad_left && layer_override > -1 {
                layer_override -= 1;
            } else if dpad_right {
                layer_override += 1;
            }
        },
        MENU_ITEM_WARP => unsafe {
            if pressed_a {
                let warp = Warp::new(
                    actual_stage_name,
                    entrance_index,
                    room_index,
                    layer_override,
                    FadeOut::NormalBlack,
                    true,
                );
                warp.execute();
            }
        },
        _ => {}
    }

    for (index, (line, &content)) in lines.into_iter().skip(3).zip(&contents).enumerate() {
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }
        let _ = write!(line.append(), "{}", content);
        match index {
            MENU_ITEM_CATEGORY => {
                let _ = write!(line.append(), "{}", category_name);
            }
            MENU_ITEM_STAGE => {
                let _ = write!(line.append(), "{}", visible_stage_name);
            }
            MENU_ITEM_ENTRANCE => {
                let _ = write!(line.append(), "{}", unsafe { entrance_index });
            }
            MENU_ITEM_ROOM => {
                let _ = write!(line.append(), "{}", unsafe { room_index });
            }
            MENU_ITEM_LAYER_OVERRIDE => {
                if unsafe { layer_override } == -1 {
                    let _ = write!(line.append(), "None");
                } else {
                    let _ = write!(line.append(), "{}", unsafe { layer_override });
                }
            }
            _ => {}
        }
    }
}
