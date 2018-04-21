use core::fmt::Write;
use libtww::game::Console;
use libtww::warping::entrance::Entrance;
use libtww::warping::fadeout::FadeOut;
use libtww::warping::warp::Warp;
use visible;

use super::*;
use controller;
use utils::*;

use super::consts::*;

static mut cursor: usize = 0;

static mut category_index: usize = 0;
static mut stage_index: usize = 0;
static mut room_index: u8 = 0;
static mut entrance_index: u16 = 0;
static mut layer_override: i8 = -1;

// TODO Synchronize this with the changes again

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
    const MENU_ITEM_BROWSE: usize = 0;
    const MENU_ITEM_CATEGORY: usize = 2;
    const MENU_ITEM_STAGE: usize = 3;
    const MENU_ITEM_ENTRANCE: usize = 4;
    const MENU_ITEM_ROOM: usize = 5;
    const MENU_ITEM_LAYER_OVERRIDE: usize = 6;
    const MENU_ITEM_WARP: usize = 8;

    const LAST_STAGE: usize = 11;
    const LAST_ENTRANCE: usize = 12;
    const LAST_ROOM: usize = 13;

    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Warp Menu");
    let _ = write!(lines[1].begin(), "=========");

    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();

    if pressed_b {
        unsafe {
            warp_menu_state = WarpMenu::Main;
        }
        transition(MenuState::MainMenu);
        return;
    }

    let contents = [
        "Browse",
        "",
        "Category:       ",
        "Stage:          ",
        "Entrance:       ",
        "Room:           ",
        "Layer Override: ",
        "",
        "Warp",
        "",
        "Last Entrance:  ",
        "Stage:          ",
        "Entrance:       ",
        "Room:           ",
    ];

    move_cursor(contents.len(), unsafe { &mut cursor });

    let last_exit = Warp::last_exit();
    let last_entrance = Entrance::last_entrance();
    let last_exit_stage = last_exit.entrance.stage_name();
    let last_exit_category = &[(last_exit_stage, last_exit_stage)];

    let stages: &[Stage] = if unsafe { category_index } == CATEGORY_LAST_WARP {
        last_exit_category
    } else {
        get_stages_for_category(unsafe { category_index })
    };
    let (category_name, _) = CATEGORIES[unsafe { category_index }];
    let (visible_stage_name, actual_stage_name) = stages[unsafe { stage_index }];

    match unsafe { cursor } {
        MENU_ITEM_BROWSE => unsafe {
            if pressed_a {
                warp_menu_state = WarpMenu::BrowseTop;
                transition(MenuState::WarpMenu);
                return;
            }
        },
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
                visible = false;
                console.visible = false;
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
            LAST_STAGE => {
                let _ = write!(line.append(), "{}", last_entrance.stage_name());
            }
            LAST_ENTRANCE => unsafe {
                let _ = write!(line.append(), "{}", last_entrance.entrance);
            },
            LAST_ROOM => {
                let _ = write!(line.append(), "{}", last_entrance.room);
            }
            _ => {}
        }
    }
}
