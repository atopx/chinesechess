use bevy::prelude::Component;

#[derive(Default, Clone, Copy, Debug)]
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

#[derive(Default, Component, Clone, Copy, Debug)]
pub enum PieceColor {
    /// 空的
    #[default]
    NONE,
    /// 白色方(红色)
    White,
    /// 黑色方
    Black,
}

pub const PIECE_NONE: Piece = Piece::new(PieceCate::None, PieceColor::NONE);
pub const PIECE_WHITE_ROOK: Piece = Piece::new(PieceCate::Rook, PieceColor::White);
pub const PIECE_WHITE_KNIGHT: Piece = Piece::new(PieceCate::Knight, PieceColor::White);
pub const PIECE_WHITE_BISHOP: Piece = Piece::new(PieceCate::Bishop, PieceColor::White);
pub const PIECE_WHITE_ADVISOR: Piece = Piece::new(PieceCate::Advisor, PieceColor::White);
pub const PIECE_WHITE_CANNON: Piece = Piece::new(PieceCate::Cannon, PieceColor::White);
pub const PIECE_WHITE_PAWN: Piece = Piece::new(PieceCate::Pawn, PieceColor::White);
pub const PIECE_WHITE_KING: Piece = Piece::new(PieceCate::King, PieceColor::White);

pub const PIECE_BLACK_ROOK: Piece = Piece::new(PieceCate::Rook, PieceColor::Black);
pub const PIECE_BLACK_KNIGHT: Piece = Piece::new(PieceCate::Knight, PieceColor::Black);
pub const PIECE_BLACK_BISHOP: Piece = Piece::new(PieceCate::Bishop, PieceColor::Black);
pub const PIECE_BLACK_ADVISOR: Piece = Piece::new(PieceCate::Advisor, PieceColor::Black);
pub const PIECE_BLACK_CANNON: Piece = Piece::new(PieceCate::Cannon, PieceColor::Black);
pub const PIECE_BLACK_PAWN: Piece = Piece::new(PieceCate::Pawn, PieceColor::Black);
pub const PIECE_BLACK_KING: Piece = Piece::new(PieceCate::King, PieceColor::Black);


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
}

impl Player {
    pub fn new_white() -> Self {
        Self {
            name: "红色方".to_string(),
            color: PieceColor::White,
            identity: PlayerIdentity::default(),
        }
    }

    pub fn new_black() -> Self {
        Self {
            name: "黑色方".to_string(),
            color: PieceColor::Black,
            identity: PlayerIdentity::default(),
        }
    }

    pub fn set_identity(&mut self, identity: PlayerIdentity) {
        self.identity = identity
    }
}


#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Piece {
    pub cate: PieceCate,
    pub color: PieceColor,
}

impl Piece {
    const fn new(cate: PieceCate, color: PieceColor) -> Self {
        Self { cate, color }
    }
}
