use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PressSpaceBar;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct UpperPipe;

#[derive(Component)]
pub struct LowerPipe;

#[derive(Component)]
pub enum Pipe {
    UpperPipe,
    LowerPipe,
}