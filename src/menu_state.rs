// use crate::menu::*;
// use crate::user_input::input;

// pub struct MenuState {
//     menu: Menu,
//     selected_option: String,
//     callback: fn(),
// }

// impl MenuState {

//     pub fn new() -> MenuState {
//         MenuState {
//         }
//     }
//     pub fn select_item(&mut self, item: String) {
//         if let Some(index) = self.menu.options.iter().position(|s| *s == item) {
//             let callback: fn() = self.menu.callbacks[index];
//             self.selected_option = item;
//             self.callback = callback;
//         } else {
//             println!("Error: {} is not a valid menu option.", item);
//         };
//     }

//     pub fn begin_menu_cycle(&mut self) {
//         self.menu = main_menu();
//         self.menu.display_menu();
//         let uinput = input();
//         self.select_item(uinput);
//         let callback = self.callback;
//         callback();
//     }
// }

// fn play_game() {

// }

// pub fn main_menu() -> Menu {

//     let options = vec![String::from("Play"), String::from("Settings"), String::from("Quit")];
//     let callbacks: Vec<fn()> = vec![play_game, settings, quit_game];

//     Menu::new(String::from("Main Menu"), 44, options, callbacks, false)

// }

// fn play_menu() -> Menu {
//     let options = vec![String::from("New Game"), String::from("Load Game")];
//     let callbacks: Vec<fn()> = vec![new_game, load_game];
//     Menu::new(String::from("Play Game"), 44, options, callbacks, true)
// }

// fn new_game() {

// }

// fn continue_game() {

// }

// fn load_game() {

// }

// fn quit_game() {
//     println!("Quit");
// }

// fn settings() {
//     println!("Options");
// }
