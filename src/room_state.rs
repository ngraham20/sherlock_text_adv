use crate::game_state::GameSettings;
use crate::room::{Lighting, Room};

pub fn begin(settings: &GameSettings) {
    let mut cur_room = lorem_ipsum(settings);
    let mut cmd = Command::Continue;
    while cmd != Command::Quit {
        cur_room.display();
        cmd = get_next_instruction();
        match cmd {
            Command::Move(Direction::North) => cur_room.lighting = Lighting::BRIGHT,
            Command::Move(Direction::South) => cur_room.lighting = Lighting::DIM,
            Command::Move(Direction::East) => cur_room.lighting = Lighting::DARK,
            _ => {}
        };
    }
}

fn calculate_lighting(room: &Room) -> Lighting {
    // TODO calculate this based on inventory and entities
    Lighting::BRIGHT
}

fn lorem_ipsum(settings: &GameSettings) -> Room {
    Room::from(
        String::from("Lorem Ipsum"),
        String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lectus mauris ultrices eros in cursus. Risus pretium quam vulputate dignissim suspendisse in est ante. Eget magna fermentum iaculis eu non diam phasellus. At lectus urna duis convallis. Hendrerit dolor magna eget est lorem ipsum. Fermentum leo vel orci porta non. Pellentesque pulvinar pellentesque habitant morbi tristique senectus et netus et. Lacus luctus accumsan tortor posuere ac ut. Nisi quis eleifend quam adipiscing vitae. Facilisis leo vel fringilla est ullamcorper eget."),
        String::from("From the ashes a fire shall be woken, a light from the shadows shall spring, renewed shall be blade that was broken, the crownless again shall be king."),
        String::from("It is much too dark to see anything."),
        Lighting::BRIGHT,
        settings.screen_width,
        vec![],
    )
}

fn get_next_instruction() -> Command {
    use crate::user_input::*;

    let uinput = input().to_owned().to_lowercase();
    let split: Vec<_> = uinput.split(' ').collect();
    match split[0] {
        "move" => Command::Move(match split[1] {
            "north" => Direction::North,
            "south" => Direction::South,
            "east" => Direction::East,
            "west" => Direction::West,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => Direction::Here }),
        "quit" => Command::Quit,
        _ => Command::Continue,
    }
}

#[derive(PartialEq)]
enum Command {
    Move(Direction),
    Continue,
    Pause,
    Quit,
}

#[derive(PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
    Here,
}
