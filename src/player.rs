use bevy::prelude::*;

use crate::component::piece::Side;

#[derive(Component, Clone, Debug)]
pub struct Record {
    pub serial: usize,
    pub code: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Id {
    // 电脑方
    Ai,
    // 主场方
    #[default]
    Home,
    // 客场方
    Away,
}

#[derive(Component, Clone, Debug, Copy)]
pub struct Player {
    pub id: Id,
    pub side: Side,
}

impl Player {
    pub fn new_white() -> Self {
        Self {
            side: Side::White,
            id: Id::default(),
        }
    }

    pub fn new_black() -> Self {
        Self {
            side: Side::Black,
            id: Id::default(),
        }
    }
}

#[derive(Component)]
pub struct PlayerFocus;
