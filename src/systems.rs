use bevy::prelude::*;

use crate::{
    components::{Background, Bird, GameOverText, Ground, PressSpaceBarText},
    resources::{Game, GameState},
};

pub fn blink_space_bar_text(
    time: Res<Time>,
    mut query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
) {
    let delta = time.delta();
    let (mut text, mut visibility) = query.single_mut();
    // Use delta to drive the timer to run
    let timer = text.0.tick(delta);

    // Check if the timer is finished
    if timer.finished() {
        // If visibility is Hidden, set to Visible
        if *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn move_background(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
    let mut transform = query.single_mut();
    let delta = time.delta_seconds();
    let delta_x = 10.0 * delta;

    transform.translation.x -= delta_x;

    // Reset the x-axis value
    if transform.translation.x < -288.0 {
        transform.translation.x = 0.0;
    }
}

pub fn move_ground(time: Res<Time>, mut query: Query<&mut Transform, With<Ground>>) {
    let mut transform = query.single_mut();
    let delta = time.delta_seconds();
    let delta_x = 20.0 * delta;

    transform.translation.x -= delta_x;

    if transform.translation.x < 112.0 {
        transform.translation.x = 0.0;
    }
}

pub fn animate_bird(mut query: Query<(&mut Bird, &mut TextureAtlas)>, time: Res<Time>) {
    for (mut bird, mut texture_atlas) in query.iter_mut() {
        let delta = time.delta();
        // Use delta to drive the animation_timer to run
        bird.animation_timer.tick(delta);

        if bird.animation_timer.finished() {
            let index = if texture_atlas.index == 2 {
                0
            } else {
                // Previous index + 1
                // Similar to `index += 1`
                texture_atlas.index + 1
            };

            texture_atlas.index = index;
        }
    }
}

// is_game_active
// Create a condition system to check the game state
pub fn is_game_active(game: Res<Game>) -> bool {
    game.state == GameState::Active
}

// is_game_not_active
// Create a condition system to check the game state
//
// Similar to is_game_active system
pub fn is_game_not_active(game: Res<Game>) -> bool {
    game.state != GameState::Active
}

pub fn start_game(
    mut game: ResMut<Game>,
    mut press_space_text: Query<(&mut PressSpaceBarText, &mut Visibility)>,
    mut game_over_text: Query<&mut Visibility, (With<GameOverText>, Without<PressSpaceBarText>)>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    if !button_input.just_pressed(KeyCode::Space) {
        return;
    }

    // Update the GameState to Active
    game.state = GameState::Active;

    // Hide the PressSpaceBarText
    // and reset the timer
    let (mut press_space_text, mut visibility) = press_space_text.single_mut();
    press_space_text.0.reset();
    *visibility = Visibility::Hidden;

    // If game restart after game over,
    // should reset the GameOverText to hidden
    let mut game_over_text_visibility = game_over_text.single_mut();
    *game_over_text_visibility = Visibility::Hidden;
}
