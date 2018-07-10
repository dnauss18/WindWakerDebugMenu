use arrayvec::{ArrayString, ArrayVec};
use core::fmt::Write;
use core::fmt::{Debug, Display, Error, Formatter};
use libtww::game::Console;
use libtww::system::memory;
use libtww::Addr;

use controller;
use core::cell::RefCell;
use utils::*;
use Mutex;

type String = ArrayString<[u8; 256]>;

pub enum Type {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Type::u8 => write!(f, "u8"),
            Type::i8 => write!(f, "i8"),
            Type::u16 => write!(f, "u16"),
            Type::i16 => write!(f, "i16"),
            Type::u32 => write!(f, "u32"),
            Type::i32 => write!(f, "i32"),
            Type::String => write!(f, "String"),
        }
    }
}

pub struct Watch {
    addr: Addr,
    x: u32,
    y: u32,
    t: Type,
    hex: bool,
}

impl Default for Watch {
    fn default() -> Self {
        Watch {
            addr: 0x80000000,
            x: 0,
            y: 0,
            t: Type::String,
            hex: true,
        }
    }
}

impl Debug for Watch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{:<9X} {:<3} {:<3} {:<4} {:<6} ",
            self.addr, self.x, self.y, self.hex, self.t
        )?;
        match self.t {
            Type::u8 => {
                let value = memory::read::<u8>(self.addr);
                write!(f, "{}", value)
            }
            Type::i8 => {
                let value = memory::read::<i8>(self.addr);
                write!(f, "{}", value)
            }
            Type::u16 => {
                let value = memory::read::<u16>(self.addr);
                write!(f, "{}", value)
            }
            Type::i16 => {
                let value = memory::read::<i16>(self.addr);
                write!(f, "{}", value)
            }
            Type::u32 => {
                let value = memory::read::<u32>(self.addr);
                write!(f, "{}", value)
            }
            Type::i32 => {
                let value = memory::read::<i32>(self.addr);
                write!(f, "{}", value)
            }
            Type::String => {
                let value = memory::read_str(memory::ptr(self.addr));
                write!(f, "{}", value)
            }
        }
    }
}

impl Display for Watch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.t {
            Type::u8 => {
                let value = memory::read::<u8>(self.addr);
                write!(f, "{}", value)
            }
            Type::i8 => {
                let value = memory::read::<i8>(self.addr);
                write!(f, "{}", value)
            }
            Type::u16 => {
                let value = memory::read::<u16>(self.addr);
                write!(f, "{}", value)
            }
            Type::i16 => {
                let value = memory::read::<i16>(self.addr);
                write!(f, "{}", value)
            }
            Type::u32 => {
                let value = memory::read::<u32>(self.addr);
                write!(f, "{}", value)
            }
            Type::i32 => {
                let value = memory::read::<i32>(self.addr);
                write!(f, "{}", value)
            }
            Type::String => {
                let value = memory::read_str(memory::ptr(self.addr));
                write!(f, "{}", value)
            }
        }
    }
}

static mut cursor: usize = 0;

lazy_static! {
    pub static ref ITEMS: Mutex<ArrayVec<[Watch; 128]>> = {
        let vec = ArrayVec::new();
        Mutex(RefCell::new(vec))
    };
}

pub fn transition_into() {}

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Memory Menu");
    let _ = write!(lines[1].begin(), "===========");
    let _ = write!(
        lines[2].begin(),
        "  {:<9} {:<3} {:<3} {:<4} {:<6} ",
        "Address",
        "X",
        "Y",
        "Hex",
        "Type"
    );

    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    move_cursor(ITEMS.borrow().len(), unsafe { &mut cursor });

    if controller::DPAD_RIGHT.is_pressed() {
        ITEMS.borrow_mut().push(Watch::default());
    }

    for (index, (line, content)) in lines
        .into_iter()
        .skip(4)
        .zip(ITEMS.borrow().iter())
        .enumerate()
    {
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let _ = write!(line.append(), "{:?}", content);
    }
}
