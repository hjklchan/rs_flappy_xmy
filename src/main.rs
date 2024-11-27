use bevy::prelude::*;
use plugin::OhMyPlugin;

mod components;
mod constants;
mod plugin;
mod resources;
mod setup;
mod systems;

use resources::Game;
use setup::setup;
use systems::{
    animate_bird, blink_space_bar_text, is_game_active, is_game_not_active, move_background,
    move_ground,
};

fn main() {
    // Create a bevy app
    App::new()
        // Customize window size
        .add_plugins(OhMyPlugin)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text.run_if(is_game_not_active))
        .add_systems(Update, move_background.run_if(is_game_active))
        .add_systems(Update, move_ground.run_if(is_game_active))
        .add_systems(Update, animate_bird.run_if(is_game_active))
        .run();
}
