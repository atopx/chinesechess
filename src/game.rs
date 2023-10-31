// use crate::component::PieceCate::{Advisor, Bishop, Cannon, King, Knight, Pawn, Rook};
use crate::component::piece::{Kind, Piece, Side};
// use crate::component::{Piece, PieceColor};
use crate::public::{ROUTE_OFFSET, START_POS};
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
    data: Res<Data>,
) {
    for key in key_events.iter() {
        if Some(KeyCode::Escape) == key.key_code && key.state.is_pressed() {
            match app_state.get() {
                Status::PENDING => {
                    if data.gameing {
                        trace!("pending to running");
                        state.set(Status::RUNNING);
                    }
                }
                Status::RUNNING => {
                    trace!("running to pending");
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
    // 没有吃子的步数
    pub noeat_move_num: usize,
    // 当前行棋方
    pub current_side: Side,
    // 游戏引擎
    pub engine: chessai::Engine,
    // 是否已经开始过游戏
    pub gameing: bool,
}

impl Data {
    pub fn new() -> Self {
        trace!("init system data");
        Self {
            engine: chessai::Engine::new(),
            white_player: player::Player::new_white(),
            black_player: player::Player::new_black(),
            broad_map: [
                [
                    Some(Piece::white(Kind::Rook)),
                    Some(Piece::white(Kind::Knight)),
                    Some(Piece::white(Kind::Bishop)),
                    Some(Piece::white(Kind::Advisor)),
                    Some(Piece::white(Kind::King)),
                    Some(Piece::white(Kind::Advisor)),
                    Some(Piece::white(Kind::Bishop)),
                    Some(Piece::white(Kind::Knight)),
                    Some(Piece::white(Kind::Rook)),
                ],
                [None, None, None, None, None, None, None, None, None],
                [
                    None,
                    Some(Piece::white(Kind::Cannon)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::white(Kind::Cannon)),
                    None,
                ],
                [
                    Some(Piece::white(Kind::Pawn)),
                    None,
                    Some(Piece::white(Kind::Pawn)),
                    None,
                    Some(Piece::white(Kind::Pawn)),
                    None,
                    Some(Piece::white(Kind::Pawn)),
                    None,
                    Some(Piece::white(Kind::Pawn)),
                ],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [
                    Some(Piece::black(Kind::Pawn)),
                    None,
                    Some(Piece::black(Kind::Pawn)),
                    None,
                    Some(Piece::black(Kind::Pawn)),
                    None,
                    Some(Piece::black(Kind::Pawn)),
                    None,
                    Some(Piece::black(Kind::Pawn)),
                ],
                [
                    None,
                    Some(Piece::black(Kind::Cannon)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::black(Kind::Cannon)),
                    None,
                ],
                [None, None, None, None, None, None, None, None, None],
                [
                    Some(Piece::black(Kind::Rook)),
                    Some(Piece::black(Kind::Knight)),
                    Some(Piece::black(Kind::Bishop)),
                    Some(Piece::black(Kind::Advisor)),
                    Some(Piece::black(Kind::King)),
                    Some(Piece::black(Kind::Advisor)),
                    Some(Piece::black(Kind::Bishop)),
                    Some(Piece::black(Kind::Knight)),
                    Some(Piece::black(Kind::Rook)),
                ],
            ],
            round: 0,
            noeat_move_num: 0,
            current_side: Side::White,
            gameing: false,
        }
    }

    // pub fn from_fen(&mut self, fen: &str) {}

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for pieces in self.broad_map.iter() {
            let mut line = String::new();
            let mut num = 0;
            for piece in pieces {
                if let Some(piece) = piece {
                    if num > 0 {
                        line.push_str(&num.to_string());
                        num = 0;
                    }
                    line.push_str(&piece.code());
                } else {
                    num += 1;
                }
            }
            if num > 0 {
                line.push_str(&num.to_string());
            }
            fen.push_str(&line);
        }
        fen.push_str(&format!(
            " {} -- {} {}",
            self.current_side.code(),
            self.noeat_move_num,
            self.round
        ));
        fen
    }

    // pub fn set_ai_game(&mut self, player_color: PieceColor) {
    //     self.engine.from_fen(START_POS);
    //     match player_color {
    //         PieceColor::White => {
    //             self.white_player.set_id("0");
    //             self.white_player.set_name("玩家");
    //             self.black_player.set_id("1");
    //             self.black_player.set_name("AI");
    //         }
    //         _ => {
    //             self.white_player.set_id("0");
    //             self.white_player.set_name("AI");
    //             self.black_player.set_id("1");
    //             self.black_player.set_name("玩家");
    //         }
    //     }
    // }

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

    // #[test]
    // fn test_parse_route() {
    //     // [97, 48, 105, 57]
    //     let test_str = String::from("a0i9");
    //     let ((row, col), (dst_row, dst_col)) = Data::new().parse_route(test_str);
    //     assert_eq!((row, col), (0, 0));
    //     assert_eq!((dst_row, dst_col), (9, 8));
    // }

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
