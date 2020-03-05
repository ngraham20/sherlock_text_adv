use crate::menu;
use crate::menu_state;
use crate::room_state;

pub fn begin(screen_width: usize) {
    let mut cmd = menu::Command::Continue;
    while cmd != menu::Command::Quit {
        cmd = menu_state::begin(screen_width);
        match cmd {
            menu::Command::Play => room_state::begin(screen_width),
            menu::Command::Quit => {}
            _ => {}
        }
    }
}
