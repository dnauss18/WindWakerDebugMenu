use libtww::prelude::*;
use libtww::system;
use libtww::Link;
use libtww::link::CollisionType;
use libtww::game::{controller, Console};

use cursor;
use main_menu;
use warp_menu;
use flag_menu;
use inventory_menu;
use cheat_menu;

use visible as debug_menu_visible;

static mut buttons_pressed: u16 = 0;
static mut buttons_down: u16 = 0;
static mut buttons_down_last_frame: u16 = 0;

#[no_mangle]
#[inline(never)]
pub extern "C" fn read_controller() -> u32 {
    unsafe {
        buttons_down_last_frame = buttons_down;
        buttons_down = system::memory::read(0x803E0CF8);
        buttons_pressed = buttons_down & (0xFFFF ^ buttons_down_last_frame);
    }
    if unsafe { debug_menu_visible } {
        controller::set_buttons_down(0x0);
        controller::set_buttons_pressed(0x0);
        system::memory::write::<u16>(0x803E0D42, 0x0);
        system::memory::write::<u16>(0x803E0CF8, 0x0);
    }
    0x80000000
}

pub fn clear_menu() {
    let console = Console::get();
    let mut lines = &mut console.lines;
    for line in lines.into_iter().skip(3) {
        line.clear();
    }
}

pub fn transition(state: MenuState) {
    clear_menu();
    unsafe {
        menu_state = state;
        cursor = 0; // TODO Improve
    }
    match state {
        MenuState::MainMenu => main_menu::transition_into(),
        MenuState::WarpMenu => warp_menu::transition_into(),
        MenuState::FlagMenu => flag_menu::transition_into(),
        MenuState::InventoryMenu => inventory_menu::transition_into(),
        MenuState::CheatMenu => cheat_menu::transition_into(),
    }
}

pub fn move_cursor(len: usize) {
    let console = Console::get();
    let lines = &mut console.lines;

    if is_pressed(controller::DPAD_UP) && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            while lines[cursor + 3].len() < 3 {
                cursor -= 1;
            }
        }
    } else if is_pressed(controller::DPAD_DOWN) && unsafe { cursor + 1 } < len {
        unsafe {
            cursor += 1;
            while lines[cursor + 3].len() < 3 {
                cursor += 1;
            }
        }
    }
}

pub fn is_pressed(buttons: u16) -> bool {
    unsafe { buttons_pressed & buttons == buttons }
}

pub fn is_down(buttons: u16) -> bool {
    unsafe { buttons_down_last_frame & buttons == buttons }
}

pub fn next_collision() {
    let collision = Link::get_collision();
    let collision = match collision {
        CollisionType::Default => CollisionType::ChestStorage,
        CollisionType::ChestStorage => CollisionType::DoorCancel,
        CollisionType::DoorCancel => CollisionType::Default,
    };
    Link::set_collision(collision);
}

pub fn bool_to_text(b: bool) -> &'static str {
    if b {
        "on"
    } else {
        "off"
    }
}

#[derive(Copy, Clone)]
pub enum MenuState {
    MainMenu,
    WarpMenu,
    FlagMenu,
    InventoryMenu,
    CheatMenu,
}

pub static mut menu_state: MenuState = MenuState::MainMenu;

