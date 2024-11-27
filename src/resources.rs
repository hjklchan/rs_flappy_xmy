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