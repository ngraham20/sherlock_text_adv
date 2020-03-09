mod entity;
mod game_state;
mod inventory;
mod item;
mod menu;
mod map;
mod menu_state;
mod room;
mod room_state;
mod user_input;

fn main() {
    // let screen_width = 60;
    // crate::game_state::begin(screen_width);
    let map = crate::map::test_map();
    crate::map::display(&map);
    crate::user_input::input();
}
