use bevy::prelude::*;

// We need to create a resource to store some information
// - Game state (active, inactive, game over, etc.)
// - The score of player (Bird)
#[derive(Resource, Default)]
pub struct Game {
    pub score: u32,
    pub state: GameState,
}

#[derive(PartialEq, Eq, Default)]
pub enum GameState {
    Active,
    #[default]
    Inactive,
    GameOver,
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pub background_texture: Handle<Image>,
    pub ground_texture: Handle<Image>,
    pub game_over_texture: Handle<Image>,
    pub space_texture: Handle<Image>,
    pub numbers_texture: Handle<Image>,
    pub bird_xmy_texture: Handle<Image>,
    pub pipe_texture: Handle<Image>,
    pub hit_audio: Handle<AudioSource>,
    pub wing_audio: Handle<AudioSource>,
}
