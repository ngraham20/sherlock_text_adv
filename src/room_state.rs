use crate::game_state::GameSettings;
use crate::room::{Lighting, Room};

pub fn begin(settings: &GameSettings) {
    let mut r = Room::new(
        String::from("Lorem Ipsum"),
        String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lectus mauris ultrices eros in cursus. Risus pretium quam vulputate dignissim suspendisse in est ante. Eget magna fermentum iaculis eu non diam phasellus. At lectus urna duis convallis. Hendrerit dolor magna eget est lorem ipsum. Fermentum leo vel orci porta non. Pellentesque pulvinar pellentesque habitant morbi tristique senectus et netus et. Lacus luctus accumsan tortor posuere ac ut. Nisi quis eleifend quam adipiscing vitae. Facilisis leo vel fringilla est ullamcorper eget."),
        String::from("From the ashes a fire shall be woken, a light from the shadows shall spring, renewed shall be blade that was broken, the crownless again shall be king."),
        String::from("It is much too dark to see anything."),
        Lighting::BRIGHT,
        settings.screen_width,
    );
    r.display();
    r.lighting = Lighting::DIM;
    r.display();
    r.lighting = Lighting::DARK;
    r.display();
}

enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}
