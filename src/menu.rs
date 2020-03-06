use crate::game_state::*;

#[derive(Clone, std::cmp::PartialEq)]
pub struct Menu {
    title: String,
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
        let dash_bar: String = vec!['-'; width].iter().collect::<String>() + &String::from("\n");
        let empty: String = format!("|{:1$}|", " ", width - 2) + &String::from("\n");
        let title_bar: String = format!("{:^1$}", title, width) + &String::from("\n");
        let title_cluster: String = String::new() + &dash_bar + &title_bar + &dash_bar;

        if is_sub_menu {
            options.push(String::from("Back"));
            commands.push(Command::Back);
        }

        let mut option_bars = String::new();
        for i in options.iter() {
            let obar: String = format!("|{:^1$}|\n", i, width - 2);
            option_bars += &obar;
        }

        let full_menu = String::new()
            + &title_cluster
            + &empty
            + &empty
            + &option_bars
            + &empty
            + &empty
            + &dash_bar;

        Menu {
            title,
            body: full_menu,
            options: options.to_vec(),
            commands: commands.to_vec(),
            is_sub_menu,
        }
    }

    pub fn display_menu(&self) {
        print!("{}", self.body);
    }

    pub fn update_settings(&mut self, settings: &GameSettings) {
        let width = settings.screen_width;
        let dash_bar: String = vec!['-'; width].iter().collect::<String>() + &String::from("\n");
        let empty: String = format!("|{:1$}|", " ", width - 2) + &String::from("\n");
        let title_bar: String = format!("{:^1$}", self.title, width) + &String::from("\n");
        let title_cluster: String = String::new() + &dash_bar + &title_bar + &dash_bar;

        let mut option_bars = String::new();
        for i in self.options.iter() {
            let obar: String = format!("|{:^1$}|\n", i, width - 2);
            option_bars += &obar;
        }

        let full_menu = String::new()
            + &title_cluster
            + &empty
            + &empty
            + &option_bars
            + &empty
            + &empty
            + &dash_bar;

        self.body = full_menu;
    }
}

#[derive(Clone, std::cmp::PartialEq)]
pub enum Command {
    SetMenu(Menu),
    RunFunc(fn()),
    ChangeSetting(GameSetting),
    Play,
    Back,
    Continue,
    InputError,
    Quit,
}

impl Command {
    pub fn _to_string(&self) -> &str {
        match self {
            Command::SetMenu(_) => "SetMenu",
            Command::RunFunc(_) => "RunFunc",
            Command::ChangeSetting(_) => "Change Setting",
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
    let mut options = vec![
        String::from("Sound"),
        String::from("Brightness"),
        String::from("Screen Size"),
    ];
    let mut commands: Vec<Command> = vec![
        Command::RunFunc(|| {}),
        Command::RunFunc(|| {}),
        Command::ChangeSetting(GameSetting::ScreenWidth),
    ];

    Menu::new(
        String::from("Settings"),
        width,
        &mut options,
        &mut commands,
        true,
    )
}
