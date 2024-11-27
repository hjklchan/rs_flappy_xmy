use bevy::prelude::*;

use crate::components::{Background, Ground, PressSpaceBarText};

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
    let delta_x = 20.0 * delta;

    transform.translation.x -= delta_x;
}

// pub fn move_ground(time: Res<Time>, mut query: Query<&mut Transform, With<Ground>>) {

// }
