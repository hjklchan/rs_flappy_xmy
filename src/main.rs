use bevy::{prelude::*, window::WindowResolution};
use plugin::OhMyPlugin;

mod plugin;
mod constants;
mod setup;
mod components;

use setup::setup;

fn main() {
    // Create a bevy app
    App::new()
        // Customize window size
        .add_plugins(OhMyPlugin)
        .add_systems(Startup, setup)
        .run();
}
