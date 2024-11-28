use bevy::prelude::*;

use crate::{
    components::{GameOverText, PressSpaceBarText, ScoreText},
    constants::{WINDOW_HALF_HEIGHT, WINDOW_HALF_WIDTH},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_score_text);
        app.add_systems(Startup, spawn_game_over_text);
        app.add_systems(Startup, spawn_press_space_bar_text);
    }
}

fn spawn_score_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // We can adjust this padding variable as much as we want to
    // achieve the desired display position
    let padding: f32 = 20.0;

    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let layout_handle = texture_atlas_layouts.add(texture_atlas_layout);

    // Distance between numbers
    // 0<->0<->0<->
    let space: f32 = 10.0;

    for i in 0..3 {
        let offset = i as f32 * 24.0 + space;

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/numbers.png"),
                transform: Transform::from_xyz(
                    -WINDOW_HALF_WIDTH + padding + offset,
                    WINDOW_HALF_HEIGHT - padding - 10.0,
                    1.0,
                ),
                ..Default::default()
            },
            TextureAtlas {
                index: 0,
                layout: layout_handle.clone(),
            },
            ScoreText,
        ));
    }
}

fn spawn_game_over_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/game-over.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        GameOverText,
    ));
}

fn spawn_press_space_bar_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/space.png"),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            ..Default::default()
        },
        PressSpaceBarText(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));
}

fn press_space_bar_text_blinking() {
    // TODO    
}
