mod browse_top;
mod stage_selection;
mod consts;
mod main;
mod room_selection;

pub static mut warp_menu_state: WarpMenu = WarpMenu::Main;

pub fn transition_into() {
    match unsafe { warp_menu_state } {
        WarpMenu::RoomSelection => room_selection::transition_into(),
        WarpMenu::StageSelection => stage_selection::transition_into(),
        WarpMenu::BrowseTop => browse_top::transition_into(),
        WarpMenu::Main => main::transition_into(),
    }
}

pub fn render() {
    match unsafe { warp_menu_state } {
        WarpMenu::RoomSelection => room_selection::render(),
        WarpMenu::StageSelection => stage_selection::render(),
        WarpMenu::BrowseTop => browse_top::render(),
        WarpMenu::Main => main::render(),
    }
}

#[derive(Copy, Clone)]
pub enum WarpMenu {
    RoomSelection,
    StageSelection,
    BrowseTop,
    Main,
}
