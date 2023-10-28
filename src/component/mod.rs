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
    pub cate: PieceCate,
    pub color: PieceColor,
    pub row: usize,
    pub col: usize,
}

impl Piece {
    pub fn new(color: PieceColor, cate: PieceCate, row: usize, col: usize) -> Self {
        Self {
            cate,
            color,
            row,
            col,
        }
    }
}
