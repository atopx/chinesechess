use bevy::prelude::{Resource, States};

use crate::component;
use crate::component::{
    PIECE_BLACK_ADVISOR, PIECE_BLACK_BISHOP, PIECE_BLACK_CANNON, PIECE_BLACK_KING,
    PIECE_BLACK_KNIGHT, PIECE_BLACK_PAWN, PIECE_BLACK_ROOK, PIECE_NONE, PIECE_WHITE_ADVISOR,
    PIECE_WHITE_BISHOP, PIECE_WHITE_CANNON, PIECE_WHITE_KING, PIECE_WHITE_KNIGHT, PIECE_WHITE_PAWN,
    PIECE_WHITE_ROOK,
};
use crate::public::ROUTE_OFFSET;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Status {
    /// 就绪
    #[default]
    PENDING,
    /// 对局中
    RUNNING,
    /// 暂停
    PAUSED,
}

#[derive(Resource)]
pub struct Data {
    /// 红色方玩家
    pub white_player: component::Player,
    /// 黑色方玩家
    pub black_player: component::Player,
    /// 棋盘地图
    pub broad_map: [[component::Piece; 9]; 10],
    /// 当前回合数
    pub round: usize,
    /// 当前行棋方
    pub current_color: component::PieceColor,
    /// 行棋记录(ICCS坐标记录)
    pub records: Vec<String>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            white_player: component::Player::new_white(),
            black_player: component::Player::new_black(),
            broad_map: [
                [
                    PIECE_WHITE_ROOK,
                    PIECE_WHITE_KNIGHT,
                    PIECE_WHITE_BISHOP,
                    PIECE_WHITE_ADVISOR,
                    PIECE_WHITE_KING,
                    PIECE_WHITE_ADVISOR,
                    PIECE_WHITE_BISHOP,
                    PIECE_WHITE_KNIGHT,
                    PIECE_WHITE_ROOK,
                ],
                [
                    PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE,
                    PIECE_NONE, PIECE_NONE, PIECE_NONE,
                ],
                [
                    PIECE_NONE,
                    PIECE_WHITE_CANNON,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_WHITE_CANNON,
                    PIECE_NONE,
                ],
                [
                    PIECE_WHITE_PAWN,
                    PIECE_NONE,
                    PIECE_WHITE_PAWN,
                    PIECE_NONE,
                    PIECE_WHITE_PAWN,
                    PIECE_NONE,
                    PIECE_WHITE_PAWN,
                    PIECE_NONE,
                    PIECE_WHITE_PAWN,
                ],
                [
                    PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE,
                    PIECE_NONE, PIECE_NONE, PIECE_NONE,
                ],
                [
                    PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE,
                    PIECE_NONE, PIECE_NONE, PIECE_NONE,
                ],
                [
                    PIECE_BLACK_PAWN,
                    PIECE_NONE,
                    PIECE_BLACK_PAWN,
                    PIECE_NONE,
                    PIECE_BLACK_PAWN,
                    PIECE_NONE,
                    PIECE_BLACK_PAWN,
                    PIECE_NONE,
                    PIECE_BLACK_PAWN,
                ],
                [
                    PIECE_NONE,
                    PIECE_BLACK_CANNON,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_BLACK_CANNON,
                    PIECE_NONE,
                ],
                [
                    PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE,
                    PIECE_NONE, PIECE_NONE, PIECE_NONE,
                ],
                [
                    PIECE_BLACK_ROOK,
                    PIECE_BLACK_KNIGHT,
                    PIECE_BLACK_BISHOP,
                    PIECE_BLACK_ADVISOR,
                    PIECE_BLACK_KING,
                    PIECE_BLACK_ADVISOR,
                    PIECE_BLACK_BISHOP,
                    PIECE_BLACK_KNIGHT,
                    PIECE_BLACK_ROOK,
                ],
            ],
            round: 0,
            current_color: component::PieceColor::NONE,
            records: Vec::new(),
        }
    }

    // pub fn from_fen() -> Self {}
    // pub fn to_fen(&self) -> String {}
    pub fn go(&mut self, route: String) -> bool {
        let ((row, col), (dst_row, dst_col)) = self.parse_route(route);
        let piece = self.broad_map[row][col];
        // todo 规则判断

        // 移动
        self.broad_map[row][col] = PIECE_NONE;
        self.broad_map[dst_row][dst_col] = piece;

        return true;
    }

    pub fn parse_route(&self, route: String) -> ((usize, usize), (usize, usize)) {
        let bytes = route.as_bytes();
        let src_col = (bytes[0] - ROUTE_OFFSET.0) as usize;
        let src_row = (bytes[1] - ROUTE_OFFSET.1) as usize;
        let dst_col = (bytes[2] - ROUTE_OFFSET.0) as usize;
        let dst_row = (bytes[3] - ROUTE_OFFSET.1) as usize;
        return ((src_row, src_col), (dst_row, dst_col));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_route() {
        // [97, 48, 105, 57]
        let test_str = String::from("a0i9");
        let ((row, col), (dst_row, dst_col)) = Data::new().parse_route(test_str);
        assert_eq!((row, col), (0, 0));
        assert_eq!((dst_row, dst_col), (9, 8));
    }
}
