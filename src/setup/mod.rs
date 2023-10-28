// pub mod asset;
mod animate;
mod font;
mod image;
pub mod sound;

use bevy::prelude::*;

#[derive(Resource)]
pub struct AssetLoading;

impl Plugin for AssetLoading {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                loading,
                font::loading,
                sound::loading,
                image::loading,
                animate::loading,
            ),
        );
    }
}

pub fn loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("loading...");
    // 创建默认镜头
    commands.spawn(Camera2dBundle::default());
}
