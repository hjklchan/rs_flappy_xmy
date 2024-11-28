use bevy::prelude::*;

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}