use crate::component::PieceCate::{Advisor, Bishop, Cannon, King, Knight, Pawn, Rook};
use crate::component::PieceColor::{Black, White};
use crate::component::{Piece, PieceColor};
use crate::public::ROUTE_OFFSET;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

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

pub fn esc_event_system(
    app_state: Res<State<Status>>,
    mut state: ResMut<NextState<Status>>,
    mut key_events: EventReader<KeyboardInput>,
    mut data: ResMut<Data>,
) {
    for key in key_events.iter() {
        if Some(KeyCode::Escape) == key.key_code && key.state.is_pressed() {
            match app_state.get() {
                Status::PENDING => {
                    if !data.previous_state.is_none() {
                        state.set(Status::RUNNING);
                    }
                }
                Status::RUNNING => {
                    trace!("running to pending");
                    data.previous_state = Some(Status::RUNNING);
                    state.set(Status::PENDING);
                }
                Status::EXIT => {}
            }
        }
    }
}

use crate::player;

#[derive(Resource)]
pub struct Data {
    // 红色方玩家
    pub white_player: player::Player,
    // 黑色方玩家
    pub black_player: player::Player,
    // 棋盘地图
    pub broad_map: [[Option<Piece>; 9]; 10],
    // 当前回合数
    pub round: usize,
    // 当前行棋方
    pub current_color: Option<PieceColor>,
    // 当前选择的棋子
    pub current_select: Option<Piece>,
    // 上一个状态
    pub previous_state: Option<Status>,
    // 游戏引擎
    pub engine: chessai::Engine,
}

impl Data {
    pub fn new() -> Self {
        trace!("init system data");

        Self {
            engine: chessai::Engine::new(),
            previous_state: None,
            white_player: player::Player::new_white(),
            black_player: player::Player::new_black(),
            broad_map: [
                [
                    Some(Piece::new(White, Rook, 0, 0)),
                    Some(Piece::new(White, Knight, 0, 1)),
                    Some(Piece::new(White, Bishop, 0, 2)),
                    Some(Piece::new(White, Advisor, 0, 3)),
                    Some(Piece::new(White, King, 0, 4)),
                    Some(Piece::new(White, Advisor, 0, 5)),
                    Some(Piece::new(White, Bishop, 0, 6)),
                    Some(Piece::new(White, Knight, 0, 7)),
                    Some(Piece::new(White, Rook, 0, 8)),
                ],
                [None, None, None, None, None, None, None, None, None],
                [
                    None,
                    Some(Piece::new(White, Cannon, 2, 1)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::new(White, Cannon, 2, 1)),
                    None,
                ],
                [
                    Some(Piece::new(White, Pawn, 3, 0)),
                    None,
                    Some(Piece::new(White, Pawn, 3, 2)),
                    None,
                    Some(Piece::new(White, Pawn, 3, 4)),
                    None,
                    Some(Piece::new(White, Pawn, 3, 6)),
                    None,
                    Some(Piece::new(White, Pawn, 3, 8)),
                ],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [
                    Some(Piece::new(Black, Pawn, 6, 0)),
                    None,
                    Some(Piece::new(Black, Pawn, 6, 2)),
                    None,
                    Some(Piece::new(Black, Pawn, 6, 4)),
                    None,
                    Some(Piece::new(Black, Pawn, 6, 6)),
                    None,
                    Some(Piece::new(Black, Pawn, 6, 6)),
                ],
                [
                    None,
                    Some(Piece::new(Black, Cannon, 7, 1)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::new(Black, Cannon, 7, 7)),
                    None,
                ],
                [None, None, None, None, None, None, None, None, None],
                [
                    Some(Piece::new(Black, Rook, 9, 0)),
                    Some(Piece::new(Black, Knight, 9, 1)),
                    Some(Piece::new(Black, Bishop, 9, 2)),
                    Some(Piece::new(Black, Advisor, 9, 3)),
                    Some(Piece::new(Black, King, 9, 4)),
                    Some(Piece::new(Black, Advisor, 9, 5)),
                    Some(Piece::new(Black, Bishop, 9, 6)),
                    Some(Piece::new(Black, Knight, 9, 7)),
                    Some(Piece::new(Black, Rook, 9, 8)),
                ],
            ],
            round: 0,
            current_color: None,
            current_select: None,
        }
    }

    pub fn set_ai_game(&mut self, player_color: PieceColor) {
        // self.white_player.next_state();
        self.current_color = Some(PieceColor::White);
        match player_color {
            PieceColor::White => {
                self.white_player.set_id("0");
                self.white_player.set_name("玩家");
                self.black_player.set_id("1");
                self.black_player.set_name("AI");
            }
            _ => {
                self.white_player.set_id("0");
                self.white_player.set_name("AI");
                self.black_player.set_id("1");
                self.black_player.set_name("玩家");
            }
        }
    }

    pub fn go(&mut self, route: String) -> bool {
        let ((row, col), (dst_row, dst_col)) = self.parse_route(route);
        let piece = self.broad_map[row][col];
        // todo 规则判断

        // 移动
        self.broad_map[row][col] = None;
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
            1 => {
                println!("1")
            }
            9 => {
                println!("9")
            }
            _ => {
                println!("n")
            }
        }
    }
}
