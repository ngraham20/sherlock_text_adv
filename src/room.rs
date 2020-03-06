extern crate textwrap;
use textwrap::Wrapper;

pub struct _Doors {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub up: bool,
    pub down: bool,
}

impl _Doors {
    pub fn _new(north: bool, south: bool, east: bool, west: bool, up: bool, down: bool) -> Self {
        _Doors {
            north,
            south,
            east,
            west,
            up,
            down,
        }
    }
}

pub struct Room {
    title: String,
    description_lit: String,
    description_dark: String,
    description_dim: String,
    pub lighting: Lighting,
    width: usize,
}

impl Room {
    pub fn new(
        title: String,
        description_lit: String,
        description_dark: String,
        description_dim: String,
        lighting: Lighting,
        width: usize,
    ) -> Self {
        let mut fdescription_lit: String = String::new();
        let wrapper = Wrapper::new(width - 4);
        let lines = wrapper.wrap(&description_lit);
        for line in &lines {
            fdescription_lit += &format!("| {:1$} |\n", line, wrapper.width);
        }

        let mut fdescription_dim: String = String::new();
        let wrapper = Wrapper::new(width - 4);
        let lines = wrapper.wrap(&description_dim);
        for line in &lines {
            fdescription_dim += &format!("| {:1$} |\n", line, wrapper.width);
        }

        let mut fdescription_dark: String = String::new();
        let wrapper = Wrapper::new(width - 4);
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
            width,
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
            vec!['-'; self.width].iter().collect::<String>() + &String::from("\n");
        //let empty: String = format!("|{:1$}|", " ", self.width - 2) + &String::from("\n");
        let title_bar: String = format!("{:^1$}", disp_title, self.width) + &String::from("\n");
        let title_cluster: String = String::new() + &dash_bar + &title_bar + &dash_bar;

        println!("{}{}", title_cluster, disp_desc);
    }
}

pub enum Lighting {
    BRIGHT,
    DIM,
    DARK,
}
