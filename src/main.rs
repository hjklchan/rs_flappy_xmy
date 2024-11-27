use bevy::prelude::*;
use plugin::OhMyPlugin;

mod components;
mod constants;
mod plugin;
mod setup;
mod systems;
mod resources;

use resources::Game;
use setup::setup;
use systems::{animate_bird, blink_space_bar_text, move_background, move_ground};

fn main() {
    // Create a bevy app
    App::new()
        // Customize window size
        .add_plugins(OhMyPlugin)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text)
        .add_systems(Update, move_background)
        .add_systems(Update, move_ground)
        .add_systems(Update, animate_bird)
        .run();
}
