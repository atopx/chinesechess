use bevy::prelude::Event;

pub enum EventAction {
    Spawn,
    Hidden,
    Despawn,
    Visibie,
}

#[derive(Event)]
pub struct GameChangeEvent(pub EventAction);

#[derive(Event)]
pub struct SwithPlayerEvent;

#[derive(Event)]
pub struct GameoverEvent(pub chessai::pregen::Winner);
