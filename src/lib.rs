#![no_std]
#![feature(const_fn)]
#![allow(non_upper_case_globals)]

extern crate libtww;

use libtww::game::Console;
use libtww::system;

pub mod cheat_menu;
pub mod controller;
pub mod flag_menu;
pub mod inventory_menu;
pub mod main_menu;
pub mod popups;
pub mod quest_menu;
pub mod spawn_menu;
pub mod utils;
pub mod warp_menu;

use utils::*;

pub static mut visible: bool = false;

#[no_mangle]
#[inline(never)]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    let console = Console::get();
    console.line_count = 32;
    console.x = 0;
    console.y = 16;
    console.font_scale_x *= 1.2;
    console.font_scale_y *= 1.2;
    console.background_color.a = 150;
    console.clear();
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn game_loop() {
    cheat_menu::apply_cheats();

    if unsafe { visible } {
        match unsafe { menu_state } {
            MenuState::MainMenu => main_menu::render(),
            MenuState::WarpMenu => warp_menu::render(),
            MenuState::FlagMenu => flag_menu::render(),
            MenuState::InventoryMenu => inventory_menu::render(),
            MenuState::CheatMenu => cheat_menu::render(),
            MenuState::SpawnMenu => spawn_menu::render(),
            MenuState::QuestMenu => quest_menu::render(),
        }
    } else if controller::DPAD_DOWN.is_pressed() && unsafe { !popups::visible } {
        let console = Console::get();
        console.visible = true;
        unsafe {
            visible = true;
        }
    } else {
        // Only check popups if the Debug Menu is not open
        popups::check_global_flags();
    }
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
    let _ = controller::read_controller();
}
