use bevy::prelude::*;

use crate::resources::GameAssets;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>();
        app.add_systems(PostStartup, load_assets);
    }
}

fn load_assets(mut game_assets: ResMut<GameAssets>, asset_server: Res<AssetServer>) {
    // Load assets
    let resource = GameAssets {
        // background_texture: asset_server.load("texture/background.png"),
        // ground_texture: asset_server.load("texture/base.png"),
        game_over_texture: asset_server.load("texture/game-over.png"),
        space_texture: asset_server.load("texture/space.png"),
        numbers_texture: asset_server.load("texture/numbers.png"),
        bird_xmy_texture: asset_server.load("texture/bird-xmy.png"),
        pipe_texture: asset_server.load("texture/pipe.png"),
        hit_audio: asset_server.load("audio/hit.ogg"),
        wing_audio: asset_server.load("audio/wing.ogg"),
    };

    *game_assets = resource;
}
