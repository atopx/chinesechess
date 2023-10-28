use bevy::prelude::*;

use crate::public;

pub fn loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("loading sounds");
    // 声音
    let sounds = public::asset::Sounds {
        bgm: asset_server.load(public::path::SOUND_BGM),
        eat: asset_server.load(public::path::SOUND_EAT),
        go: asset_server.load(public::path::SOUND_GO),
        invalid: asset_server.load(public::path::SOUND_INVALID),
        select: asset_server.load(public::path::SOUND_SELECT),
        check: asset_server.load(public::path::SOUND_CHECK),
        lose: asset_server.load(public::path::SOUND_LOSE),
        win: asset_server.load(public::path::SOUND_WIN),
        alarm: asset_server.load(public::path::SOUND_ALARM),
    };
    commands.insert_resource(sounds);
}
