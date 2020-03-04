mod menu;
mod menu_state;
mod user_input;
mod room;
mod inventory;
mod item;
mod entity;
use menu_state::begin_menu_cycle;
use room::*;

fn main() {
    let width = 60;
    begin_menu_cycle(width);
    let mut r = Room::new(
        String::from("Lorem Ipsum"),
        String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lectus mauris ultrices eros in cursus. Risus pretium quam vulputate dignissim suspendisse in est ante. Eget magna fermentum iaculis eu non diam phasellus. At lectus urna duis convallis. Hendrerit dolor magna eget est lorem ipsum. Fermentum leo vel orci porta non. Pellentesque pulvinar pellentesque habitant morbi tristique senectus et netus et. Lacus luctus accumsan tortor posuere ac ut. Nisi quis eleifend quam adipiscing vitae. Facilisis leo vel fringilla est ullamcorper eget."),
        //String::from("All that is gold does not glitter, not all those who wander are lost. The old that is strong shall not wither, deep roots are not reached by the frost."),
        String::from("From the ashes a fire shall be woken, a light from the shadows shall spring, renewed shall be blade that was broken, the crownless again shall be king."),
        String::from("This is a dimly lit room..."),
        Doors::new(true, true, false, false, false, false),
        Lighting::BRIGHT,
        width
    );
    r.display();
    r.lighting = Lighting::DIM;
    r.display();
    r.lighting = Lighting::DARK;
    r.display();
}