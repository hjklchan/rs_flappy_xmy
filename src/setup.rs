use bevy::prelude::*;

use crate::components::{Bird, Pipe};
use crate::{
    components::{Background, GameOverText, Ground, PressSpaceBarText, ScoreText},
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
};
use rand::{self, Rng};

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
    // Top position.x is -200
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/base.png"),
            transform: Transform::from_xyz(0.0, -256.0, 1.0),
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
        PressSpaceBarText(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));

    // Spawn "Score" text
    const SCORE_SPACE_X: f32 = 2.0;
    let score_number_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let score_number_handle = texture_atlas_layouts.add(score_number_layout);
    // Spawn three score texts at once
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

    // Spawn a bird that use human avatars
    let bird_layout = TextureAtlasLayout::from_grid(UVec2::new(42, 40), 3, 1, None, None);
    let bird_handle = texture_atlas_layouts.add(bird_layout);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/bird-xmy.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::splat(1.5),
                ..Default::default()
            },
            ..Default::default()
        },
        TextureAtlas {
            index: 0,
            layout: bird_handle,
        },
        Bird,
    ));

    // const DEL: f32 = 15.0;
    const HALF_PIPE_H: f32 = 320.0 / 2.0;
    // Space between pairs of pipes
    const DELTA_X: f32 = 200.0;
    // Spawn five pair of pipes at once
    for i in 0..5 {
        let gen_x = (i as f32 + 1.0) * DELTA_X;
        // First spawn the LowerPipe
        let mut rng = rand::thread_rng();
        // Randomly generated y-axis position
        let mut random_pipe_position = || {
            let lower = rng.gen_range(-200.0 - (HALF_PIPE_H + 20.0)..-200.0 + HALF_PIPE_H);
            (lower, lower + 425.0)
        };
        let (lower_y, upper_y) = random_pipe_position();

        // Spawn the LowerPipe
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/pipe.png"),
                transform: Transform::from_xyz(gen_x, lower_y, 0.5),
                ..Default::default()
            },
            Pipe::LowerPipe,
        ));

        // Spawn the UpperPipe
        // Need to rotate the pipe by 180 degree
        // and have a gap between itself and the lower pipe -> 60px <-
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/pipe.png"),
                transform: Transform {
                    translation: Vec3::new(gen_x, upper_y, 0.5),
                    // translation: Vec3::new(0.0, 0.0, 0.5),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI),
                    ..Default::default()
                },
                ..Default::default()
            },
            Pipe::UpperPipe,
        ));
    }
}
