use bevy::prelude::*;

use crate::public;

pub fn loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("loading asset animates");
    // 动画
    let animates = public::asset::Animates {
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
        endposflag: vec![
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_0),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_1),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_2),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_3),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_4),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_5),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_6),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_7),
        ],
    };
    commands.insert_resource(animates);
}
