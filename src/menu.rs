#[derive(Clone, std::cmp::PartialEq, Debug)]
pub struct Menu {
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
        let empty: String = format!("|{:1$}|", " ", width - 2) + &String::from("\n");
        let title_bar: String = format!("{:^1$}", title, width) + &String::from("\n");
        let title_cluster: String = String::new() + &bar + &title_bar + &bar;

        if is_sub_menu {
            options.push(String::from("Back"));
            commands.push(Command::Back);
        }

        let mut option_bars = String::new();
        for i in 0..options.len() {
            let obar: String = format!("|{:^1$}|\n", options[i], width - 2);
            option_bars += &obar;
        }

        let full_menu =
            String::new() + &title_cluster + &empty + &empty + &option_bars + &empty + &empty + &bar;

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

#[derive(Clone, std::cmp::PartialEq, Debug)]
pub enum Command {
    SetMenu(Menu),
    RunFunc(fn()),
    Play,
    Back,
    Continue,
    InputError,
    Quit,
}

impl Command {
    pub fn to_string(&self) -> &str {
        match self {
            Command::SetMenu(_) => "SetMenu",
            Command::RunFunc(_) => "RunFunc",
            Command::Play => "Play",
            Command::Back => "Back",
            Command::Continue => "Continue",
            Command::InputError => "InputError",
            Command::Quit => "Quit",
        }
    }
}

// -------------------------- PUBLIC FUNCTIONS --------------------------

pub fn main_menu(width: usize) -> Menu {
    let mut options = vec![
        String::from("Play"),
        String::from("Settings"),
        String::from("Quit"),
    ];
    let mut commands: Vec<Command> = vec![
        Command::Play,
        Command::SetMenu(settings_menu(width)),
        Command::Quit,
    ];

    Menu::new(
        String::from("Main Menu"),
        width,
        &mut options,
        &mut commands,
        false,
    )
}

// -------------------------- PUBLIC FUNCTIONS --------------------------

fn settings_menu(width: usize) -> Menu {
    let mut options = vec![String::from("Sound"), String::from("Brightness")];
    let mut commands: Vec<Command> = vec![Command::RunFunc(|| {}), Command::RunFunc(|| {})];

    Menu::new(
        String::from("Settings"),
        width,
        &mut options,
        &mut commands,
        true,
    )
}

fn play() {}