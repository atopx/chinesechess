use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub mod exited;
pub mod paused;
pub mod pending;
pub mod running;

/// 游戏主状态
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// 就绪
    #[default]
    PENDING,
    /// 对局中
    RUNNING,
    /// 暂停
    PAUSED,
    /// 结束游戏
    EXITED,
}

/// 游戏对局状态
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ChessState {
    /// 空
    #[default]
    None,
    // AI行棋
    AiPlay,
    // 主场玩家行棋
    HomePlay,
    // 客场行棋
    AwayPlay,
    // 结束
    Gameover,
}

pub fn esc_event_system(
    app_state: Res<State<GameState>>,
    mut state: ResMut<NextState<GameState>>,
    mut key_events: EventReader<KeyboardInput>,
) {
    for key in key_events.iter() {
        if Some(KeyCode::Escape) == key.key_code && key.state.is_pressed() {
            match app_state.get() {
                GameState::PAUSED => {
                    info!("paused to running");
                    state.set(GameState::RUNNING);
                }
                GameState::RUNNING => {
                    info!("running to paused");
                    state.set(GameState::PAUSED);
                }
                _ => { /* nothing */ }
            }
        }
    }
}
