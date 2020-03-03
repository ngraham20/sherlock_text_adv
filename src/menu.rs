#[derive(Clone, std::cmp::PartialEq, Debug)]
struct Menu {
    body: String,
    is_sub_menu: bool,
    pub options: Vec<String>,
    pub commands: Vec<Command>,
}

impl Menu {
    pub fn new(
        title: String,
        width: usize,
        options: &mut Vec<String>,
        commands: &mut Vec<Command>,
        is_sub_menu: bool,
    ) -> Self {
        let bar: String = vec!['-'; width].iter().collect::<String>() + &String::from("\n");
        let empty: String = String::from("|")
            + &vec![' '; width - 2].iter().collect::<String>()
            + &String::from("|")
            + &String::from("\n");

        let title_bar: String = vec![' '; ((width - title.len() + 1) / 2) as usize]
            .iter()
            .collect::<String>()
            + &title
            + &vec![' '; ((width - title.len() + 1) / 2) as usize]
                .iter()
                .collect::<String>()
            + &String::from("\n");

        let menu_bar: String = String::new() + &bar + &title_bar + &bar;

        if is_sub_menu {
            options.push(String::from("Back"));
            commands.push(Command::Back);
        }

        let mut option_bars = String::new();
        for i in 0..options.len() {
            let obar: String = String::from("|")
                + &vec![' '; ((width - options[i].len() + 1) / 2) - 1 as usize]
                    .iter()
                    .collect::<String>()
                + &options[i]
                + &vec![' '; ((width - options[i].len()) / 2 - 1) as usize]
                    .iter()
                    .collect::<String>()
                + &String::from("|\n");

            option_bars += &obar;
        }

        let full_menu =
            String::new() + &menu_bar + &empty + &empty + &option_bars + &empty + &empty + &bar;

        Menu {
            body: full_menu,
            options: options.to_vec(),
            commands: commands.to_vec(),
            is_sub_menu: is_sub_menu,
        }
    }

    pub fn display_menu(&self) {
        print!("{}", self.body);
    }
}

fn select_item(menu: Box<Menu>, item: String) -> Command {
    // get the index, if it exists, and set the command
    if let Some(index) = menu
        .options
        .iter()
        .position(|s| *s.to_lowercase() == item.to_lowercase())
    {
        menu.commands[index].to_owned()
    } else {
        Command::Continue
    }
}

// -------------------------- PUBLIC FUNCTIONS --------------------------

pub fn menu_cycle() {
    use crate::user_input::*;
    let start = &main_menu();
    let mut cmd = Command::Continue;
    let mut cur_menu = start.clone();
    let mut menu_stack = vec![];
    while cmd != Command::Quit {
        cur_menu.display_menu();
        let uinput = input();
        cmd = select_item(Box::from(cur_menu.to_owned()), uinput).to_owned();
        if cfg!(debug_assertions) {
            println!("Command Acknowledged: {}", cmd.to_string());
        }
        flush_output();
        match &cmd {
            Command::RunFunc(call) => call(),
            Command::SetMenu(menu) => {
                menu_stack.push(cur_menu.clone());
                cur_menu = menu.clone()
            }
            Command::Back => cur_menu = menu_stack.pop().unwrap().clone(),
            _ => {}
        }
    }
}

// -------------------------- PUBLIC FUNCTIONS --------------------------

fn main_menu() -> Menu {
    let mut options = vec![
        String::from("Play"),
        String::from("Settings"),
        String::from("Quit"),
    ];
    let mut commands: Vec<Command> = vec![
        Command::RunFunc(play),
        Command::SetMenu(settings_menu()),
        Command::Quit,
    ];

    Menu::new(
        String::from("Main Menu"),
        44,
        &mut options,
        &mut commands,
        false,
    )
}

fn settings_menu() -> Menu {
    let mut options = vec![String::from("Sound"), String::from("Brightness")];
    let mut commands: Vec<Command> = vec![Command::RunFunc(|| {}), Command::RunFunc(|| {})];

    Menu::new(
        String::from("Settings"),
        44,
        &mut options,
        &mut commands,
        true,
    )
}

fn play() {}

#[derive(Clone, std::cmp::PartialEq, Debug)]
enum Command {
    SetMenu(Menu),
    RunFunc(fn()),
    Back,
    Continue,
    Quit,
}

impl Command {
    pub fn to_string(&self) -> &str {
        match self {
            Command::SetMenu(_) => "SetMenu",
            Command::RunFunc(_) => "RunFunc",
            Command::Back => "Back",
            Command::Continue => "Continue",
            Command::Quit => "Quit",
        }
    }
}
