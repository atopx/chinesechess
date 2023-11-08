use bevy::prelude::*;

use crate::public;

pub fn loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("loading asset animates");

    let endposflag = asset_server.load(public::path::ANIMATE_ENDPOSFLAG);
    let texture_atlas =
        TextureAtlas::from_grid(endposflag, Vec2::new(106.0, 106.0), 8, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // 动画
    let animates = public::asset::Animates {
        endposflag: texture_atlas_handle,
        check: vec![
            asset_server.load(public::path::ANIMATE_CHECK_0),
            asset_server.load(public::path::ANIMATE_CHECK_1),
            asset_server.load(public::path::ANIMATE_CHECK_2),
            asset_server.load(public::path::ANIMATE_CHECK_3),
            asset_server.load(public::path::ANIMATE_CHECK_4),
        ],
        checkmate: vec![
            asset_server.load(public::path::ANIMATE_CHECKMATE_0),
            asset_server.load(public::path::ANIMATE_CHECKMATE_1),
            asset_server.load(public::path::ANIMATE_CHECKMATE_2),
            asset_server.load(public::path::ANIMATE_CHECKMATE_3),
            asset_server.load(public::path::ANIMATE_CHECKMATE_4),
        ],
    };
    commands.insert_resource(animates);
}
