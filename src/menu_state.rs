use crate::menu::main_menu;
use crate::menu;
use crate::menu::Menu;

fn select_item(menu: Menu, item: &str) -> menu::Command {
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

pub fn begin(width: usize) -> menu::Command {
    use crate::user_input::*;
    let start = &main_menu(width);
    let mut cmd = menu::Command::Continue;
    let mut cur_menu = start.clone();
    let mut message = String::new();
    let mut menu_stack = vec![];
    let mut uinput: String;
    while cmd != menu::Command::Quit && cmd != menu::Command::Play {
        cur_menu.display_menu();
        print!("{}", message);
        message = String::new();
        uinput = input();
        cmd = select_item(cur_menu.to_owned(), &uinput).to_owned();
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
    cmd
}
