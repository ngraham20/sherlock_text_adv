extern crate textwrap;
use textwrap::Wrapper;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum DoorState {
    Open,
    Locked,
    Hidden,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Door {
    North(DoorState),
    South(DoorState),
    East(DoorState),
    West(DoorState),
    Up(DoorState),
    Down(DoorState),
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Room {
    title: String,
    description_lit: String,
    description_dark: String,
    description_dim: String,
    pub lighting: Lighting,
    screen_width: usize,
    pub doors: Vec<Door>,
}

impl Room {

    pub fn new(screen_width: usize) -> Room {
        Room {
            title: String::from("Default Room"),
            description_lit: String::from("You are in a brightly lit room."),
            description_dim: String::from("You are in a dimly lit room."),
            description_dark: String::from("It is too dark here to see anyting."),
            lighting: Lighting::BRIGHT,
            screen_width,
            doors: vec![],
        }
    }

    pub fn from(
        title: String,
        description_lit: String,
        description_dark: String,
        description_dim: String,
        lighting: Lighting,
        screen_width: usize,
        doors: Vec<Door>,
    ) -> Self {
        let mut fdescription_lit: String = String::new();
        let wrapper = Wrapper::new(screen_width - 4);
        let lines = wrapper.wrap(&description_lit);
        for line in &lines {
            fdescription_lit += &format!("| {:1$} |\n", line, wrapper.width);
        }

        let mut fdescription_dim: String = String::new();
        let wrapper = Wrapper::new(screen_width - 4);
        let lines = wrapper.wrap(&description_dim);
        for line in &lines {
            fdescription_dim += &format!("| {:1$} |\n", line, wrapper.width);
        }

        let mut fdescription_dark: String = String::new();
        let wrapper = Wrapper::new(screen_width - 4);
        let lines = wrapper.wrap(&description_dark);
        for line in &lines {
            fdescription_dark += &format!("| {:1$} |\n", line, wrapper.width);
        }

        Room {
            title,
            description_lit: fdescription_lit,
            description_dark: fdescription_dim,
            description_dim: fdescription_dark,
            lighting,
            screen_width,
            doors,
        }
    }

    pub fn display(&self) {
        let (disp_title, disp_desc) = match self.lighting {
            Lighting::BRIGHT => (self.title.to_owned(), self.description_lit.to_owned()),
            Lighting::DIM => (
                format!("{} ({})", self.title, "Dimly Lit"),
                self.description_dim.to_owned(),
            ),
            Lighting::DARK => (
                format!("{} ({})", self.title, "Unlit"),
                self.description_dark.to_owned(),
            ),
        };

        let dash_bar: String =
            vec!['-'; self.screen_width].iter().collect::<String>() + &String::from("\n");
        //let empty: String = format!("|{:1$}|", " ", self.width - 2) + &String::from("\n");
        let title_bar: String = format!("{:^1$}", disp_title, self.screen_width) + &String::from("\n");
        let title_cluster: String = String::new() + &dash_bar + &title_bar + &dash_bar;

        println!("{}{}", title_cluster, disp_desc);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Lighting {
    BRIGHT,
    DIM,
    DARK,
}
