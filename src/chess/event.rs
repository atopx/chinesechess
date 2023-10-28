use crate::{component, game::Data, player::Player, public};
use bevy::prelude::*;

#[derive(Event)]
pub struct TmpDebugEvent(pub Entity);

pub fn tmp_debug_consumer(mut data: ResMut<Data>, mut events: EventReader<TmpDebugEvent>) {
    for event in events.iter() {
        trace!("Entity {:?} tmp debug event!", event.0);
    }
}
