use bevy::prelude::*;

use crate::{
    components::{Background, Movement, Velocity},
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    // resources::GameAssets,
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // game_assets: Res<GameAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/background.png"),
            // texture: game_assets.background_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH + 288.0 * 2.0, WINDOW_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.0,
        },
        Background,
        Movement,                        // Owned Movement
        Velocity::from(-Vec3::X * 10.0), // Owned Velocity
    ));
}
