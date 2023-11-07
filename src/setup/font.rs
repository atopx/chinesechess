use bevy::prelude::*;

use crate::public;

pub fn loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("loading asset fonts");
    // 字体
    let fonts = public::asset::Fonts {
        wenkai: asset_server.load(public::path::FONT_WENKAI),
        xiaoli: asset_server.load(public::path::FONT_XIAOLI),
    };
    commands.insert_resource(fonts);
}
