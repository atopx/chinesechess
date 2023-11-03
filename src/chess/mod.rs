mod audio;
mod button;

use crate::{
    game::{ChessState, Data, Status},
    public::START_POS,
};
use bevy::prelude::*;

mod broad;
mod chess;
mod event;
mod info;

#[derive(Resource)]
pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app
            // 测试事件
            // .add_event::<event::SwithPlayerEvent>()
            // .add_systems(Update, event::swith_player_event)
            // 进入RUNNING状态
            .add_systems(
                OnEnter(Status::RUNNING),
                (
                    new_game,
                    broad::setup_broad,
                    button::setup_bottons,
                    info::setup_black_info,
                    info::setup_white_info,
                ),
            )
            // 退出RUNNING状态
            .add_systems(
                OnExit(Status::RUNNING),
                (broad::cleanup_chessbroad, button::cleanup_button, info::cleanup_info),
            )
            // 对局中系统
            .add_systems(
                Update,
                (
                    // 棋子系统
                    chess::selection.run_if(in_state(Status::RUNNING)),
                    // 对局功能按钮
                    button::chess_button_system.run_if(in_state(Status::RUNNING)),
                    info::refresh_player_action,
                    // info::refresh_player_timer,
                ),
            );
    }
}

fn new_game(mut commands: Commands, mut data: ResMut<Data>, time: Res<Time>) {
    if data.state.is_none() {
        data.state = Some(ChessState::Game);
        data.engine.from_fen(START_POS);
    }
}
