use bevy::prelude::*;

use crate::{
    components::{Background, GameOverText, Ground, PressSpaceBar, ScoreText},
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn a new Camera
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });

    // Spawn the game Background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/background.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
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
    ));

    // Spawn the Ground
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/base.png"),
            transform: Transform::from_xyz(0.0, -250.0, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH, 112.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.0,
        },
        Ground,
    ));

    // Spawn "GameOver" text
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/game-over.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            // Default hide
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        GameOverText,
    ));

    // Spawn "PressSpaceBar" text
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/space.png"),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            ..Default::default()
        },
        PressSpaceBar,
    ));

    // Spawn "Score" text
    const SCORE_SPACE_X: f32 = 2.0;
    let score_number_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let score_number_handle = texture_atlas_layouts.add(score_number_layout);
    // Spawn three the score texts at once
    for i in 0..3 {
        let offset_x = -350.0 + ((24.0 + SCORE_SPACE_X) * i as f32);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/numbers.png"),
                transform: Transform::from_xyz(offset_x, 200.0, 1.0),
                ..Default::default()
            },
            TextureAtlas {
                index: 0,
                layout: score_number_handle.clone(),
            },
            ScoreText,
        ));
    }

    // TODO: Spawn a bird that use human avatars
}
