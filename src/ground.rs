use bevy::prelude::*;

use crate::{
    constants::WINDOW_WIDTH,
    // resources::GameAssets
};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // game_assets: Res<GameAssets>,
) {
    commands.spawn((
        SpriteBundle {
            // texture: game_assets.ground_texture.clone(),
            texture: asset_server.load("texture/base.png"),
            transform: Transform::from_xyz(0.0, -256.0, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH + 112.0 * 2.0, 112.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.0,
        },
    ));
}
