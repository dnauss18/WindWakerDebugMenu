use super::*;
use controller;
use core::fmt::Write;
use libtww::game::Console;
use libtww::warping::fadeout::FadeOut;
use libtww::warping::Warp;
use utils::*;
use visible;

pub static STAGE: &'static str = "sea";

use super::consts::*;

static mut cursor: usize = 0;
static mut scroll_offset: usize = 0;

pub fn transition_into() {}

pub fn scroll_move_cursor(array: &'static [&'static str]) {
    if controller::DPAD_UP.is_pressed() && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            if cursor >= 4 && cursor - 4 < scroll_offset {
                scroll_offset = cursor - 4;
            }
        }
    } else if controller::DPAD_DOWN.is_pressed() && unsafe { cursor + 1 } < array.len() {
        unsafe {
            cursor += 1;
            if cursor + 4 < array.len() && cursor > scroll_offset + 20 {
                scroll_offset = cursor - 20;
            }
        }
    }
}

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Warp Room Menu");
    let _ = write!(lines[1].begin(), "==============");

    let pressed_a = controller::A.is_pressed();
    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        unsafe {
            warp_menu_state = WarpMenu::StageSelection;
        }
        transition(MenuState::WarpMenu);
        return;
    }

    let rooms: &'static [&'static str] = match STAGE {
        "sea" => SEA_ROOMS,
        _ => &["No Data"],
    };

    scroll_move_cursor(rooms);

    if pressed_a {
        match STAGE {
            "sea" => unsafe {
                let room_id = cursor as u8 + 1;
                visible = false;
                console.visible = false;
                let warp = Warp::new(STAGE, 0, room_id, -1, FadeOut::NormalBlack, true);
                warp_menu_state = WarpMenu::Main;
                transition(MenuState::WarpMenu);
                warp.execute();
                return;
            },
            _ => {}
        }
    }

    for (index, (line, room)) in lines
        .into_iter()
        .skip(3)
        .zip(rooms.iter().skip(unsafe { scroll_offset }))
        .enumerate()
        .take(25)
    {
        if index == unsafe { cursor - scroll_offset } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let _ = write!(line.append(), "{}", room);
    }
}
