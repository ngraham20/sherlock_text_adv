use crate::menu::main_menu;
use crate::menu;
use crate::menu::Menu;

fn select_item(menu: Box<Menu>, item: &String) -> menu::Command {
    // get the index, if it exists, and set the command
    if let Some(index) = menu
        .options
        .iter()
        .position(|s| *s.to_lowercase() == item.to_lowercase())
    {
        menu.commands[index].to_owned()
    } else {
        menu::Command::InputError
    }
}

pub fn begin_menu_cycle(width: usize) {
    use crate::user_input::*;
    let start = &main_menu(width);
    let mut cmd = menu::Command::Continue;
    let mut cur_menu = start.clone();
    let mut message = String::new();
    let mut menu_stack = vec![];
    let mut uinput = String::new();
    while cmd != menu::Command::Quit {
        cur_menu.display_menu();
        if cfg!(debug_assertions) {
            println!(
                "[DEBUG]: Command Acknowledged: '{}', Type: {}",
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
            menu::Command::RunFunc(call) => call(),
            menu::Command::SetMenu(menu) => {
                menu_stack.push(cur_menu.clone());
                cur_menu = menu.clone()
            }
            menu::Command::InputError => message = format!("Unknown input: {}\n", uinput),
            menu::Command::Back => cur_menu = menu_stack.pop().unwrap().clone(),
            _ => {}
        }
    }
}
