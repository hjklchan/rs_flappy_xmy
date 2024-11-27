use bevy::prelude::*;
use plugin::OhMyPlugin;

mod components;
mod constants;
mod plugin;
mod setup;
mod systems;

use setup::setup;
use systems::{blink_space_bar_text, move_background, move_ground};

fn main() {
    // Create a bevy app
    App::new()
        // Customize window size
        .add_plugins(OhMyPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text)
        .add_systems(Update, move_background)
        .add_systems(Update, move_ground)
        .run();
}
