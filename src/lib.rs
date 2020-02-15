#![feature(alloc_error_handler)]
#![no_std]
#![allow(non_upper_case_globals)]

extern crate gcn;
extern crate libtww;
// #[macro_use]
// extern crate lazy_static;

use libtww::game::Console;
use libtww::system;

pub mod cheat_menu;
pub mod controller;
pub mod flag_menu;
pub mod inventory_menu;
pub mod main_menu;
pub mod mutex;
pub mod popups;
pub mod quest_menu;
pub mod spawn_menu;
pub mod triforce;
pub mod utils;
pub mod warp_menu;

// use mutex::*;
use utils::*;

pub static mut visible: bool = false;

extern crate alloc;

#[global_allocator]
static ALLOC: libtww::Alloc = libtww::Alloc;

#[cfg(target_arch = "powerpc")]
#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    panic!("Failed allocating")
}

extern crate futures_util;

pub static mut the_future: core::mem::MaybeUninit<
    core::pin::Pin<alloc::boxed::Box<dyn core::future::Future<Output = ()>>>,
> = core::mem::MaybeUninit::uninit();

use core::future::Future;
use futures_util::{
    future::{self, FutureExt},
    stream::{self, StreamExt},
};
use libtww::{futures::*, game::gamepad};

// async fn delete_hearts() -> ! {
//     loop {
//         gcn::report!("Deleting heart");
//         libtww::Link::get().heart_quarters -= 1;
//         delay_for(15).await;
//     }
// }

// async fn future_stuff() {
//     buttons_pressed(gamepad::X).await;
//     gcn::report!("Pressed X");
//     delay_for(30).await;
//     gcn::report!("Beginning to delete hearts");
//     future::select(delete_hearts(), buttons_pressed(gamepad::X)).await;
//     gcn::report!("Cancelled");
// }

fn future_stuff() -> impl Future<Output = ()> {
    buttons_pressed(gamepad::X)
        .then(|_| {
            gcn::report!("Pressed X");
            delay_for(30)
        })
        .then(|_| {
            gcn::report!("Beginning to delete hearts");
            future::select(
                stream::iter(0..).for_each(|_| {
                    gcn::report!("Deleting heart");
                    libtww::Link::get().heart_quarters -= 1;
                    delay_for(15)
                }),
                buttons_pressed(gamepad::X).map(|_| {
                    gcn::report!("Cancelled");
                }),
            )
            .map(drop)
        })
}

#[no_mangle]
pub extern "C" fn init() {
    // Call overridden instruction
    system::cdyl_init_async();

    let console = Console::get();
    console.line_count = 32;
    console.x = 0;
    console.y = 16;
    console.font_scale_x *= 1.2;
    console.font_scale_y *= 1.2;
    console.background_color.a = 150;
    console.clear();

    unsafe {
        core::ptr::write(
            the_future.as_mut_ptr(),
            alloc::boxed::Box::pin(future_stuff().fuse()),
        );
    }
}

#[no_mangle]
pub extern "C" fn game_loop() {
    unsafe { libtww::futures::run((&mut *the_future.as_mut_ptr()).as_mut()) };

    cheat_menu::apply_cheats();
    let d_down = controller::DPAD_DOWN.is_pressed();
    let rt_down = controller::R.is_down();

    if unsafe { visible } {
        match unsafe { menu_state } {
            MenuState::MainMenu => main_menu::render(),
            MenuState::WarpMenu => warp_menu::render(),
            MenuState::FlagMenu => flag_menu::render(),
            MenuState::InventoryMenu => inventory_menu::render(),
            MenuState::CheatMenu => cheat_menu::render(),
            MenuState::SpawnMenu => spawn_menu::render(),
            MenuState::QuestMenu => quest_menu::render(),
            MenuState::Triforce => triforce::render(),
        }
    } else if d_down && rt_down && unsafe { !popups::visible } {
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
