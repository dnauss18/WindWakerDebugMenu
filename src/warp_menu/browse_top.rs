use core::fmt::Write;
use libtww::game::Console;

use super::stage_selection::stage_category;
use super::*;
use crate::controller;
use crate::utils::*;

use super::consts::*;

static mut cursor: usize = 0;

pub fn transition_into() {}

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;
    let _ = write!(lines[0].begin(), "Warp Category Menu");
    let _ = write!(lines[1].begin(), "==================");

    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            warp_menu_state = WarpMenu::Main;
        }
        transition(MenuState::WarpMenu);
        return;
    }

    move_cursor(CATEGORIES.len(), unsafe { &mut cursor });

    let (_, category) = CATEGORIES[unsafe { cursor }];

    if pressed_a {
        unsafe {
            stage_category = category;
            warp_menu_state = WarpMenu::StageSelection;
        }
        transition(MenuState::WarpMenu);
        return;
    }

    for (index, (line, &(content, _))) in lines.into_iter().skip(3).zip(&CATEGORIES).enumerate() {
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let _ = write!(line.append(), "{}", content);
    }
}
