use bevy::prelude::*;

use crate::{event::SwithPlayerEvent, game::Data, player, status::ChessState};

use super::info::PlayerInfoAction;

pub fn event_listen(
    mut events: EventReader<SwithPlayerEvent>,
    mut data: ResMut<Data>,
    mut chess_state: ResMut<NextState<ChessState>>,
    mut action_q: Query<(&player::Player, &mut Text), With<PlayerInfoAction>>,
) {
    for _ in events.iter() {
        // 换边
        data.change_side();

        // 切换对局状态
        match data.mode.unwrap() {
            crate::game::GameMode::AiGame => {
                chess_state.set(ChessState::AiPlay);
            }
            crate::game::GameMode::DeduceGame => {
                chess_state.set(ChessState::HomePlay);
            }
            crate::game::GameMode::InterGame => {
                chess_state.set(ChessState::AwayPlay);
            }
        }

        // 刷新双方行动信息
        for (player, mut text) in action_q.iter_mut() {
            if data.current_side.unwrap() == player.side {
                text.sections[0].value = String::from("思考中");
                text.sections[0].style.color = Color::ORANGE_RED;
            } else {
                text.sections[0].value = String::from("空闲中");
                text.sections[0].style.color = Color::DARK_GREEN;
            }
        }
    }
}
