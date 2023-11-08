use bevy::prelude::*;

use crate::{
    chess::previou::PiecePreviouMove,
    event::SwithPlayerEvent,
    game::{Data, GameMode},
    player,
    public::BroadEntitys,
    status::ChessState,
};

use super::info::PlayerInfoAction;

pub fn event_listen(
    mut commands: Commands,
    mut events: EventReader<SwithPlayerEvent>,
    mut data: ResMut<Data>,
    mut chess_state: ResMut<NextState<ChessState>>,
    mut action_q: Query<(&player::Player, &mut Text), With<PlayerInfoAction>>,
    mut entitys: ResMut<BroadEntitys>,
) {
    for _ in events.iter() {
        data.selected = None;
        entitys.selected = None;
        let next = match data.get_current_player().id {
            player::Id::Away => ChessState::HomePlay,
            player::Id::Ai => ChessState::HomePlay,
            player::Id::Home => match data.mode.unwrap() {
                GameMode::AiGame => ChessState::AiPlay,
                GameMode::DeduceGame => ChessState::HomePlay,
                GameMode::InterGame => ChessState::AwayPlay,
            },
        };

        // 换边
        data.change_side();
        // 切换对局状态
        info!("next state: {:?} {}", next, data.engine.mv_list.last().unwrap());
        let (src, dst) = data.get_last_move().unwrap();
        commands.spawn(PiecePreviouMove(src, dst));
        chess_state.set(next);

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
        return;
    }
}
