mod audio;
pub mod button;
pub mod chess;
use crate::game::Status;
use bevy::prelude::*;
mod broad;
mod event;

#[derive(Resource)]
pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app
            // 测试事件
            .add_event::<event::TmpDebugEvent>()
            .add_systems(Update, event::tmp_debug_consumer)
            // 进入RUNNING状态
            .add_systems(OnEnter(Status::RUNNING), broad::setup_running)
            // 退出RUNNING状态
            .add_systems(OnExit(Status::RUNNING), broad::cleanup_chessbroad)
            // 对局中系统
            .add_systems(
                Update,
                (
                    // 棋子系统
                    chess::game_chess_system.run_if(in_state(Status::RUNNING)),
                    // 对局功能按钮
                    button::chess_button_system.run_if(in_state(Status::RUNNING)),
                ),
            );
    }
}
