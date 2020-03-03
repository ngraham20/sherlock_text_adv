use crate::menu::main_menu;
use crate::menu::Command;
use crate::menu::Menu;

fn select_item(menu: Box<Menu>, item: &String) -> Command {
    // get the index, if it exists, and set the command
    if let Some(index) = menu
        .options
        .iter()
        .position(|s| *s.to_lowercase() == item.to_lowercase())
    {
        menu.commands[index].to_owned()
    } else {
        Command::InputError
    }
}

pub fn menu_cycle() {
    use crate::user_input::*;
    let start = &main_menu();
    let mut cmd = Command::Continue;
    let mut cur_menu = start.clone();
    let mut message = String::new();
    let mut menu_stack = vec![];
    let mut uinput = String::new();
    while cmd != Command::Quit {
        cur_menu.display_menu();
        if cfg!(debug_assertions) {
            println!(
                "Command Acknowledged: '{}', Type: {}",
                uinput,
                cmd.to_string()
            );
        }
        print!("{}", message);
        message = String::new();
        uinput = input();
        cmd = select_item(Box::from(cur_menu.to_owned()), &uinput).to_owned();
        flush_output();
        match &cmd {
            Command::RunFunc(call) => call(),
            Command::SetMenu(menu) => {
                menu_stack.push(cur_menu.clone());
                cur_menu = menu.clone()
            }
            Command::InputError => message = format!("Unknown input: {}\n", uinput),
            Command::Back => cur_menu = menu_stack.pop().unwrap().clone(),
            _ => {}
        }
    }
}
