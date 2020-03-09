use crate::menu;
use crate::menu_state;
use crate::room_state;
use crate::user_input::*;
use crate::map::Map;

pub struct GameSettings {
    pub screen_width: usize,
}

impl GameSettings {
    pub fn _new() -> Self {
        GameSettings { screen_width: 60 }
    }

    pub fn from(screen_width: usize) -> Self {
        GameSettings { screen_width }
    }

    pub fn modify_setting(&mut self, setting: GameSetting) {
        match setting {
            GameSetting::ScreenWidth => {
                println!("What should the new screen width be?");
                flush_output();
                let size = input();
                self.screen_width = size.parse().unwrap()
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum GameSetting {
    ScreenWidth,
}

pub fn begin(screen_width: usize) {
    let mut settings = GameSettings::from(screen_width);
    let mut cmd = menu::Command::Continue;
    while cmd != menu::Command::Quit {
        cmd = menu_state::begin(&mut settings);
        match cmd {
            menu::Command::Play => room_state::begin(&settings),
            menu::Command::Quit => {}
            _ => {}
        }
    }
}
