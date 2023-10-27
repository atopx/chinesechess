use bevy::prelude::{Resource, States, trace};

use crate::component;
use crate::component::{Piece, PIECE_NONE, PieceColor};
use crate::component::PieceCate::{Advisor, Bishop, Cannon, King, Knight, Pawn, Rook};
use crate::component::PieceColor::{Black, White};
use crate::public::ROUTE_OFFSET;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Status {
    /// 就绪
    #[default]
    PENDING,
    /// 对局中
    RUNNING,
    /// 结束游戏
    EXIT,
}


#[derive(Resource)]
pub struct Data {
    /// 红色方玩家
    pub white_player: component::Player,
    /// 黑色方玩家
    pub black_player: component::Player,
    // pub chessbroad: Option<Entity>,
    /// 棋盘地图
    pub broad_map: [[Piece; 9]; 10],
    /// 当前回合数
    pub round: usize,
    /// 当前行棋方
    pub current_color: PieceColor,
    pub previous_state: Option<Status>,
}

impl Data {
    pub fn new() -> Self {
        trace!("init system data");

        Self {
            previous_state: None,
            white_player: component::Player::new_white(),
            black_player: component::Player::new_black(),
            broad_map: [
                [
                    Piece::new(White, Rook),
                    Piece::new(White, Knight),
                    Piece::new(White, Bishop),
                    Piece::new(White, Advisor),
                    Piece::new(White, King),
                    Piece::new(White, Advisor),
                    Piece::new(White, Bishop),
                    Piece::new(White, Knight),
                    Piece::new(White, Rook),
                ],
                [
                    PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE,
                    PIECE_NONE, PIECE_NONE, PIECE_NONE,
                ],
                [
                    PIECE_NONE,
                    Piece::new(White, Cannon),
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    Piece::new(White, Cannon),
                    PIECE_NONE,
                ],
                [
                    Piece::new(White, Pawn),
                    PIECE_NONE,
                    Piece::new(White, Pawn),
                    PIECE_NONE,
                    Piece::new(White, Pawn),
                    PIECE_NONE,
                    Piece::new(White, Pawn),
                    PIECE_NONE,
                    Piece::new(White, Pawn),
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
                    Piece::new(Black, Pawn),
                    PIECE_NONE,
                    Piece::new(Black, Pawn),
                    PIECE_NONE,
                    Piece::new(Black, Pawn),
                    PIECE_NONE,
                    Piece::new(Black, Pawn),
                    PIECE_NONE,
                    Piece::new(Black, Pawn),
                ],
                [
                    PIECE_NONE,
                    Piece::new(Black, Cannon),
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    PIECE_NONE,
                    Piece::new(Black, Cannon),
                    PIECE_NONE,
                ],
                [
                    PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE, PIECE_NONE,
                    PIECE_NONE, PIECE_NONE, PIECE_NONE,
                ],
                [
                    Piece::new(Black, Rook),
                    Piece::new(Black, Knight),
                    Piece::new(Black, Bishop),
                    Piece::new(Black, Advisor),
                    Piece::new(Black, King),
                    Piece::new(Black, Advisor),
                    Piece::new(Black, Bishop),
                    Piece::new(Black, Knight),
                    Piece::new(Black, Rook),
                ]
            ],
            round: 0,
            current_color: PieceColor::None,
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

    #[test]
    fn test_match() {
        let n = 9;
        match n {
            1 => { println!("1") }
            9 => { println!("9") }
            _ => { println!("n") }
        }
    }
}
