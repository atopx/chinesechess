mod audio;
mod button;

use crate::{
    event::{GameChangeEvent, GameoverEvent, SwithPlayerEvent},
    status::{ChessState, GameState},
};
use bevy::prelude::*;

mod ai_chess;
mod broad;
mod chess;
mod gameover;
mod info;
mod previou;
mod swith_player;

#[derive(Resource)]
pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app
            // 测试事件
            .add_event::<GameChangeEvent>()
            .add_event::<SwithPlayerEvent>()
            .add_event::<GameoverEvent>()
            .add_state::<ChessState>()
            .add_systems(
                Update,
                (
                    button::event_listen,
                    info::event_listen,
                    broad::event_listen,
                    swith_player::event_listen,
                    gameover::event_listen,
                    previou::piece_previou_animate,
                    previou::piece_previou_move,
                ),
            )
            .add_systems(
                Update, // 对局功能按钮
                button::chess_button_system.run_if(in_state(ChessState::HomePlay)),
            )
            .add_systems(
                Update, // 玩家棋子系统
                chess::selection
                    .run_if(in_state(ChessState::HomePlay))
                    .after(swith_player::event_listen),
            )
            .add_systems(
                Update, // AI棋子系统
                ai_chess::ai_move
                    .run_if(in_state(ChessState::AiPlay))
                    .after(swith_player::event_listen),
            );
    }
}
