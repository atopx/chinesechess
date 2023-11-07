use bevy::prelude::*;

use crate::public;

pub fn loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("loading sounds");
    // 声音
    let sounds = public::asset::Sounds {
        bgm: asset_server.load(public::path::SOUND_BGM),
        eat: asset_server.load(public::path::SOUND_EAT),
        go: asset_server.load(public::path::SOUND_GO),
        invalid: asset_server.load(public::path::SOUND_INVALID),
        select: asset_server.load(public::path::SOUND_SELECT),
        check: asset_server.load(public::path::SOUND_CHECK),
        loss: asset_server.load(public::path::SOUND_LOSS),
        win: asset_server.load(public::path::SOUND_WIN),
        draw: asset_server.load(public::path::SOUND_DRAW),
        alarm: asset_server.load(public::path::SOUND_ALARM),
    };
    commands.insert_resource(sounds);
}
