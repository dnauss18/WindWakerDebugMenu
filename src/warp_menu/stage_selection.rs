use super::*;
use crate::controller;
use crate::utils::*;
use crate::visible;
use core::fmt::Write;
use libtww::game::Console;
use libtww::warping::fadeout::FadeOut;
use libtww::warping::Warp;

pub static mut stage_category: Category = Category::Cavern;

use super::consts::*;

static mut cursor: usize = 0;

pub fn transition_into() {}

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Warp Stage Menu");
    let _ = write!(lines[1].begin(), "==================");

    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            warp_menu_state = WarpMenu::BrowseTop;
        }
        transition(MenuState::WarpMenu);
        return;
    }

    let stage: &[Stage] = match unsafe { stage_category } {
        Category::LastWarp => unreachable!(),
        Category::Cavern => STAGES_CAVERN,
        Category::Developer => STAGES_DEV,
        Category::DragonRoostCavern => STAGES_DRAGON_ROOST_CAVERN,
        Category::DragonRoostIsland => STAGES_DRAGON_ROOST_ISLAND,
        Category::EarthTemple => STAGES_EARTH_TEMPLE,
        Category::ForbiddenWoods => STAGES_FORBIDDEN_WOODS,
        Category::ForestHaven => STAGES_FOREST_HAVEN,
        Category::ForsakenFortress => STAGES_FORSAKEN_FORTRESS,
        Category::GanonsTower => STAGES_GANONS_TOWER,
        Category::GreatFairy => STAGES_GREAT_FAIRY,
        Category::Hyrule => STAGES_HYRULE,
        Category::NintendoGallery => STAGES_NINTENDO_GALLERY,
        Category::Other => STAGES_OTHER,
        Category::Outset => STAGES_OUTSET,
        Category::SavageLabyrinth => STAGES_SAVAGE_LABYRINTH,
        Category::Sea => STAGES_SEA,
        Category::ToweroftheGods => STAGES_TOWER_OF_THE_GODS,
        Category::WindTemple => STAGES_WIND_TEMPLE,
        Category::Windfall => STAGES_WINDFALL,
    };
    unsafe {
        if cursor > stage.len() - 1 {
            cursor = stage.len() - 1;
        }
    }

    move_cursor(stage.len(), unsafe { &mut cursor });

    if pressed_a {
        unsafe {
            let (_, actual_stage_name) = stage[cursor];
            match (stage_category, actual_stage_name) {
                (Category::Sea, "sea") => {
                    if pressed_a {
                        warp_menu_state = WarpMenu::RoomSelection;
                        transition(MenuState::WarpMenu);
                        return;
                    }
                }
                _ => {
                    visible = false;
                    console.visible = false;
                    let warp = Warp::new(actual_stage_name, 0, 0, -1, FadeOut::NormalBlack, true);
                    warp_menu_state = WarpMenu::Main;
                    transition(MenuState::WarpMenu);
                    warp.execute();
                    return;
                }
            }
        }
    }

    for (index, (line, &(content, _))) in lines.into_iter().skip(3).zip(stage).enumerate() {
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let _ = write!(line.append(), "{}", content);
    }
}
