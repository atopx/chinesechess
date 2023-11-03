use bevy::prelude::*;

use crate::component::piece::Side;

#[derive(Component, Clone, Debug)]
pub struct Record {
    pub serial: usize,
    pub code: String,
    pub value: String,
}

#[derive(Component, Clone, Debug, Copy)]
pub struct Player {
    pub side: Side,
}

impl Player {
    pub fn new_white() -> Self {
        Self { side: Side::White }
    }

    pub fn new_black() -> Self {
        Self { side: Side::Black }
    }
}

#[derive(Component)]
pub struct PlayerFocus;
