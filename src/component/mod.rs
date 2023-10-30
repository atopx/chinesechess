use bevy::prelude::Component;

#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum PieceCate {
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

#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum PieceColor {
    // 白色方(红色)
    White,
    // 黑色方
    Black,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Piece {
    pub cate: Option<PieceCate>,
    pub color: Option<PieceColor>,
    pub row: usize,
    pub col: usize,
}

impl Piece {
    pub fn new(color: PieceColor, cate: PieceCate, row: usize, col: usize) -> Self {
        Self {
            cate: Some(cate),
            color: Some(color),
            row,
            col,
        }
    }

    pub fn none(row: usize, col: usize) -> Self {
        Self {
            cate: None,
            color: None,
            row,
            col,
        }
    }
}

#[derive(Component)]
pub struct PieceSelect;
