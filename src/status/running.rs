use crate::event::{EventAction, GameChangeEvent};
use bevy::prelude::*;

pub fn from_pending_enter(
    // mut commands: Commands,
    // mut data: ResMut<Data>,
    // mut entitys: ResMut<BroadEntitys>,
    // image_handles: Res<public::asset::Images>,
    // piece_handles: Res<public::asset::Pieces>,
    mut event: EventWriter<GameChangeEvent>,
) {
    info!("pending to runnint");
    event.send(GameChangeEvent(EventAction::Spawn));
}

pub fn from_paused_enter(mut event: EventWriter<GameChangeEvent>) {
    info!("paused to runnint");
    event.send(GameChangeEvent(EventAction::Visibie));
}

pub fn exit_state(mut event: EventWriter<GameChangeEvent>) {
    info!("退出RUNNING");
    event.send(GameChangeEvent(EventAction::Hidden));
}
