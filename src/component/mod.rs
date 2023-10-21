use bevy::prelude::{Component, Entity};
use crate::component::PieceColor::White;

#[derive(Default, Clone, Copy, Debug, Component)]
pub enum PieceCate {
    /// 空, 默认值
    #[default]
    None,
    // 車
    Rook,
    // 马
    Knight,
    // 象
    Bishop,
    // 士
    Advisor,
    // 炮
    Cannon,
    // 兵
    Pawn,
    // 帅
    King,
}

#[derive(Default, Clone, Copy, Debug, Component)]
pub enum PieceColor {
    /// 空的
    #[default]
    None,
    /// 白色方(红色)
    White,
    /// 黑色方
    Black,
}

pub const PIECE_NONE: Piece = Piece::new(PieceColor::None, PieceCate::None, None);

#[derive(Component, Clone, Copy, Debug, Default)]
pub enum PlayerIdentity {
    AI,
    #[default]
    Person,
}

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub color: PieceColor,
    pub identity: PlayerIdentity,
    pub records: Vec<String>,
}

impl Player {
    pub fn new(name: &str, color: PieceColor, identity: PlayerIdentity, records: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            color,
            identity,
            records,
        }
    }
    pub fn new_white() -> Self {
        return Self::new("红色方", White, PlayerIdentity::default(), Vec::new());
    }

    pub fn new_black() -> Self {
        return Self::new("黑色方", White, PlayerIdentity::default(), Vec::new());
    }

    pub fn set_identity(&mut self, identity: PlayerIdentity) {
        self.identity = identity
    }
}


#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Piece {
    pub cate: PieceCate,
    pub color: PieceColor,
    pub entity: Option<Entity>,
}

impl Piece {
    pub const fn new(color: PieceColor, cate: PieceCate, entity: Option<Entity>) -> Self {
        Self { cate, color, entity }
    }
}
