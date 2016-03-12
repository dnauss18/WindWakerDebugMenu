use libtww::prelude::*;
use libtww::game::{controller, Console};
use libtww::Link;
use libtww::system;

use utils::*;
use cursor;

static mut scroll_offset: usize = 0;

pub fn transition_into() {
    unsafe {
        scroll_offset = 0;
    }
}

pub fn scroll_move_cursor(len: usize) {
    if is_pressed(controller::DPAD_UP) && unsafe { cursor } > 0 {
        unsafe {
            cursor -= 1;
            if cursor >= 4 && cursor - 4 < scroll_offset {
                scroll_offset = cursor - 4;
            }
        }
    } else if is_pressed(controller::DPAD_DOWN) && unsafe { cursor + 1 } < len {
        unsafe {
            cursor += 1;
            if cursor + 4 < len && cursor > scroll_offset + 20 {
                scroll_offset = cursor - 20;
            }
        }
    }
}

pub struct Cheat {
    pub name: &'static str,
    pub active: bool,
    pub togglable: bool,
}

impl Cheat {
    const fn new(name: &'static str, togglable: bool) -> Self {
        Cheat {
            name: name,
            active: false,
            togglable: togglable,
        }
    }
}

pub fn apply_cheats() {
    let mut link = Link::get();

    for (cheat_id, cheat) in unsafe { cheats.iter().enumerate() } {
        if cheat.active {
            match cheat_id {
                CHEAT_MOON_JUMP => {
                    Link::position().y += 175.0;
                }
                CHEAT_INVINCIBLE => {
                    link.heart_quarters = link.heart_pieces;
                }
                CHEAT_INFINITE_MAGIC => {
                    link.max_magic = 32;
                    link.magic = link.max_magic;
                }
                CHEAT_SWIFT_WIND => {
                    let direction = Link::direction();
                    let wind = if direction < 0x1000 || direction > 0xF000 {
                        2
                    } else if direction < 0x3000 {
                        1
                    } else if direction < 0x5000 {
                        0
                    } else if direction < 0x7000 {
                        7
                    } else if direction < 0x9000 {
                        6
                    } else if direction < 0xB000 {
                        5
                    } else if direction < 0xD000 {
                        4
                    } else {
                        3
                    };
                    system::set_wind(wind);
                }
                _ => {}
            }
        }
    }
}

static mut cheats: [Cheat; 4] = [Cheat::new("Invincible", true),
                                 Cheat::new("Infinite Magic", true),
                                 Cheat::new("Swift Wind", true),
                                 Cheat::new("Moon Jump", false)];

const CHEAT_INVINCIBLE: usize = 0;
const CHEAT_INFINITE_MAGIC: usize = 1;
const CHEAT_SWIFT_WIND: usize = 2;
const CHEAT_MOON_JUMP: usize = 3;

pub fn render() {
    let console = Console::get();
    let mut lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Cheat Menu");
    let _ = write!(lines[1].begin(), "==========");

    let down_a = is_down(controller::A);
    let pressed_a = is_pressed(controller::A);
    let pressed_b = is_pressed(controller::B);
    // let dpad_left = is_pressed(controller::DPAD_LEFT);
    // let dpad_right = is_pressed(controller::DPAD_RIGHT);

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    scroll_move_cursor(unsafe { cheats.len() });

    let cheat_id = unsafe { cursor };
    let cheat = unsafe { &mut cheats[cheat_id] };

    if cheat.togglable {
        cheat.active ^= pressed_a;
    } else {
        cheat.active = down_a;
    }

    for (index, (line, cheat)) in lines.into_iter()
                                       .skip(3)
                                       .zip(unsafe {
                                           cheats.iter()
                                                 .skip(scroll_offset)
                                       })
                                       .enumerate()
                                       .take(25) {
        let index = index + unsafe { scroll_offset };
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let checkbox = if cheat.active {
            "[x] "
        } else {
            "[ ] "
        };

        let text = cheat.name;
        let text = if text.len() > 45 {
            &text[..45]
        } else {
            text
        };

        let _ = write!(line.append(), "{}{}", checkbox, text);
    }
}
