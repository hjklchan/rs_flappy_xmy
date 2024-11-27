use bevy::{prelude::*, window::WindowResolution};
use plugin::OhMyPlugin;

mod plugin;
mod constants;
mod setup;
mod components;
mod systems;

use setup::setup;
use systems::{blink_space_bar_text, move_background};

fn main() {
    // Create a bevy app
    App::new()
        // Customize window size
        .add_plugins(OhMyPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text)
        .add_systems(Update, move_background)
        .run();
}
