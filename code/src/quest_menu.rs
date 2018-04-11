use core::fmt::{self, Write};
use libtww::Link;
use libtww::game::Console;
use libtww::link::item::*;
use libtww::link::quest_items::*;

use controller;
use utils::*;

static mut cursor: usize = 0;

static mut scroll_offset: usize = 0;

pub fn transition_into() {}

const ITEM_SLOTS: [&str; 2] = ["Sword", "Shield"];

fn handle_item_switch() {
    let link = Link::get();
    let quest_items = QuestItems::get();
    let dpad_left = controller::DPAD_LEFT.is_pressed();
    let dpad_right = controller::DPAD_RIGHT.is_pressed();
    let index = unsafe { cursor };
    if dpad_left {
        match index {
            0 => match quest_items.sword {
                Sword::None => link.set_sword(Sword::FullyChargedMasterSword),
                Sword::HerosSword => link.set_sword(Sword::None),
                Sword::UnchargedMasterSword => link.set_sword(Sword::HerosSword),
                Sword::HalfChargedMasterSword => link.set_sword(Sword::UnchargedMasterSword),
                Sword::FullyChargedMasterSword => link.set_sword(Sword::HalfChargedMasterSword),
            },
            1 => match quest_items.shield {
                Shield::None => link.set_shield(Shield::MirrorShield),
                Shield::HerosShield => link.set_shield(Shield::None),
                Shield::MirrorShield => link.set_shield(Shield::HerosShield),
            },
            _ => {}
        }
    } else if dpad_right {
        match index {
            0 => match quest_items.sword {
                Sword::None => link.set_sword(Sword::HerosSword),
                Sword::HerosSword => link.set_sword(Sword::UnchargedMasterSword),
                Sword::UnchargedMasterSword => link.set_sword(Sword::HalfChargedMasterSword),
                Sword::HalfChargedMasterSword => link.set_sword(Sword::FullyChargedMasterSword),
                Sword::FullyChargedMasterSword => link.set_sword(Sword::None),
            },
            1 => match quest_items.shield {
                Shield::None => link.set_shield(Shield::HerosShield),
                Shield::HerosShield => link.set_shield(Shield::MirrorShield),
                Shield::MirrorShield => link.set_shield(Shield::None),
            },
            _ => {}
        }
    }
}

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Quest Menu");
    let _ = write!(lines[1].begin(), "==========");

    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    move_cursor(ITEM_SLOTS.len(), unsafe { &mut cursor });

    handle_item_switch();

    for (index, (line, &text)) in lines
        .into_iter()
        .skip(3)
        .zip(ITEM_SLOTS.iter().skip(unsafe { scroll_offset }))
        .enumerate()
        .take(25)
    {
        let link = Link::get();
        let index = index + unsafe { scroll_offset };
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }
        let item_text = match index {
            0 => item_id_to_str(link.sword_id),
            1 => item_id_to_str(link.shield_id),
            _ => unreachable!(),
        };

        let item_text = if item_text.len() > 45 {
            &item_text[..45]
        } else {
            item_text
        };
        let _ = write!(line.append(), "{} {}", ColonWrapper(text), item_text);
    }
}

struct ColonWrapper<'a>(&'a str);

impl<'a> fmt::Display for ColonWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:", self.0)?;
        for _ in 0..(6 - self.0.len()) {
            write!(f, " ")?;
        }
        Ok(())
    }
}
