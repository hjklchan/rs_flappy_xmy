use asset_loader::AssetLoaderPlugin;
use background::BackgroundPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use ground::GroundPlugin;
use plugin::OhMyPlugin;
use ui::UiPlugin;

mod asset_loader;
mod background;
mod camera;
mod components;
mod constants;
mod ground;
mod plugin;
mod resources;
mod setup;
mod states;
mod systems;
mod ui;

use resources::{Game, GameScore};
use setup::setup;
use systems::{
    animate_bird, bird_gravity, blink_space_bar_text, is_game_active, is_game_not_active, jump,
    move_background, move_ground, move_pipe, start_game,
};

fn main() {
    // Create a bevy app
    App::new()
        // Customize window size
        .init_resource::<GameScore>() // Newest
        .add_plugins(OhMyPlugin)
        .add_plugins(AssetLoaderPlugin) // Newest
        .add_plugins(CameraPlugin)
        .add_plugins(UiPlugin)    
        .add_plugins(BackgroundPlugin)
        .add_plugins(GroundPlugin)
        // .init_resource::<Game>()
        // .add_systems(Startup, setup)
        // .add_systems(Update, blink_space_bar_text.run_if(is_game_not_active))
        // .add_systems(Update, move_background.run_if(is_game_active))
        // .add_systems(Update, move_ground.run_if(is_game_active))
        // .add_systems(Update, animate_bird.run_if(is_game_active))
        // .add_systems(Update, start_game.run_if(is_game_not_active))
        // .add_systems(Update, bird_gravity.run_if(is_game_active))
        // .add_systems(Update, jump.run_if(is_game_active))
        // .add_systems(Update, move_pipe.run_if(is_game_active))
        .run();
}
