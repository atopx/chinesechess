use bevy::prelude::*;

use crate::public;

#[derive(Component, Debug, Clone, Copy)]
pub struct Piece {
    pub side: Side,
    pub kind: Kind,
    pub row: usize,
    pub col: usize,
}

impl Piece {
    pub fn white(kind: Kind, row: usize, col: usize) -> Self {
        Self {
            side: Side::White,
            kind,
            row,
            col,
        }
    }
    pub fn black(kind: Kind, row: usize, col: usize) -> Self {
        Self {
            side: Side::Black,
            kind,
            row,
            col,
        }
    }
    pub fn code(&self) -> String {
        match self.side {
            Side::White => self.kind.code().to_uppercase(),
            Side::Black => self.kind.code().to_string(),
        }
    }

    pub fn name(&self) -> String {
        format!("{}{}", self.side.name(), self.kind.name())
    }

    pub fn image_handle(&self, piece_handles: public::asset::Pieces) -> Handle<Image> {
        match self.side {
            Side::White => match self.kind {
                Kind::Rook => piece_handles.white_rook,
                Kind::Knight => piece_handles.white_knight,
                Kind::Bishop => piece_handles.white_bishop,
                Kind::Advisor => piece_handles.white_advisor,
                Kind::Cannon => piece_handles.white_cannon,
                Kind::Pawn => piece_handles.white_pawn,
                Kind::King => piece_handles.white_king,
            },
            Side::Black => match self.kind {
                Kind::Rook => piece_handles.black_rook,
                Kind::Knight => piece_handles.black_knight,
                Kind::Bishop => piece_handles.black_bishop,
                Kind::Advisor => piece_handles.black_advisor,
                Kind::Cannon => piece_handles.black_cannon,
                Kind::Pawn => piece_handles.black_pawn,
                Kind::King => piece_handles.black_king,
            },
        }
    }

    pub fn select_image_handle(&self, piece_handles: public::asset::Pieces) -> Handle<Image> {
        match self.side {
            Side::White => match self.kind {
                Kind::Rook => piece_handles.white_rook_select,
                Kind::Knight => piece_handles.white_knight_select,
                Kind::Bishop => piece_handles.white_bishop_select,
                Kind::Advisor => piece_handles.white_advisor_select,
                Kind::Cannon => piece_handles.white_cannon_select,
                Kind::Pawn => piece_handles.white_pawn_select,
                Kind::King => piece_handles.white_king_select,
            },
            Side::Black => match self.kind {
                Kind::Rook => piece_handles.black_rook_select,
                Kind::Knight => piece_handles.black_knight_select,
                Kind::Bishop => piece_handles.black_bishop_select,
                Kind::Advisor => piece_handles.black_advisor_select,
                Kind::Cannon => piece_handles.black_cannon_select,
                Kind::Pawn => piece_handles.black_pawn_select,
                Kind::King => piece_handles.black_king_select,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    // 白色方(红色)
    White,
    // 黑色方
    Black,
}

impl Side {
    pub fn code(&self) -> &str {
        match self {
            Self::White => "w",
            Self::Black => "b",
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Self::White => "红",
            Self::Black => "黑",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Kind {
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

impl Kind {
    pub fn code(&self) -> &str {
        match self {
            Kind::Rook => "r",
            Kind::Knight => "k",
            Kind::Bishop => "b",
            Kind::Advisor => "a",
            Kind::Cannon => "c",
            Kind::Pawn => "p",
            Kind::King => "k",
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Kind::Rook => "车",
            Kind::Knight => "马",
            Kind::Bishop => "象",
            Kind::Advisor => "士",
            Kind::Cannon => "炮",
            Kind::Pawn => "兵",
            Kind::King => "将",
        }
    }
}

#[derive(Resource)]
pub struct ImageHandle {
    pub black_advisor: Handle<Image>,
    pub black_advisor_select: Handle<Image>,
    pub black_bishop: Handle<Image>,
    pub black_bishop_select: Handle<Image>,
    pub black_cannon: Handle<Image>,
    pub black_cannon_select: Handle<Image>,
    pub black_king: Handle<Image>,
    pub black_king_select: Handle<Image>,
    pub black_knight: Handle<Image>,
    pub black_knight_select: Handle<Image>,
    pub black_pawn: Handle<Image>,
    pub black_pawn_select: Handle<Image>,
    pub black_rook: Handle<Image>,
    pub black_rook_select: Handle<Image>,
    pub white_advisor: Handle<Image>,
    pub white_advisor_select: Handle<Image>,
    pub white_bishop: Handle<Image>,
    pub white_bishop_select: Handle<Image>,
    pub white_cannon: Handle<Image>,
    pub white_cannon_select: Handle<Image>,
    pub white_king: Handle<Image>,
    pub white_king_select: Handle<Image>,
    pub white_knight: Handle<Image>,
    pub white_knight_select: Handle<Image>,
    pub white_pawn: Handle<Image>,
    pub white_pawn_select: Handle<Image>,
    pub white_rook: Handle<Image>,
    pub white_rook_select: Handle<Image>,
}
