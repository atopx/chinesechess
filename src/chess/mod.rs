mod audio;
mod button;
use crate::game::Status;
use bevy::prelude::*;
mod broad;
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
            .add_systems(OnEnter(Status::RUNNING), (
                broad::setup_broad,
                button::setup_bottons,
                info::setup_black_info,
                info::setup_white_info,
            ))
            // 退出RUNNING状态
            .add_systems(OnExit(Status::RUNNING), (
                broad::cleanup_chessbroad,
                button::cleanup_button,
                info::cleanup_info,
            ))
            // 对局中系统
            .add_systems(
                Update,
                (
                    // 棋子系统
                    // chess::game_chess_system.run_if(in_state(Status::RUNNING)),
                    // 对局功能按钮
                    button::chess_button_system.run_if(in_state(Status::RUNNING)),
                ),
            );
    }
}

