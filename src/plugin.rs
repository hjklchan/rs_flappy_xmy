use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub struct OhMyPlugin;

impl Plugin for OhMyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Xmy Puppy".into(),
                // Setting the window resolution to 800*512 pixels
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                // The window size can not be changed
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}
