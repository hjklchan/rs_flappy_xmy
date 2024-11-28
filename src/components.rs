use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PressSpaceBarText(pub Timer);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Bird {
    pub animation_timer: Timer,
    pub velocity: f32,
}

#[derive(Component)]
pub struct UpperPipe;

#[derive(Component)]
pub struct LowerPipe;

#[derive(Component)]
pub enum Pipe {
    UpperPipe,
    LowerPipe,
}

/// Movement
///
/// Can be bound to components or entities with mobile characteristics
#[derive(Component)]
pub struct Movement;

#[derive(Component)]
pub struct Velocity {
    value: Vec3,
}

impl From<Vec3> for Velocity {
    fn from(value: Vec3) -> Self {
        Self {
            value,
        }
    }
}

#[derive(Component)]
pub struct Gravity {
    value: f32,
}
