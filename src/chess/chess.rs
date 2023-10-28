use bevy::prelude::*;

use crate::{component, game::Data, public, player::{Player}};

use super::event;

// 棋子系统
pub fn game_chess_system(
    mut commands: Commands,
    mut data: ResMut<Data>,
    mut tmp_debug_event: EventWriter<event::TmpDebugEvent>,
    sounds: Res<public::asset::Sounds>,
    mut interaction_query: Query<
        (Entity, &Parent, &Interaction, &component::Piece, &Player),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, parent, interaction, piece, player) in &mut interaction_query {
        tmp_debug_event.send(event::TmpDebugEvent(entity));
        
        match *interaction {
            Interaction::Pressed => {
                trace!("{:?} {:?}", piece, player);
                if let Some(current_color) = &data.current_color {
                    // 只有当前行棋方才有效
                    if *current_color == piece.color && player.color == piece.color {
                        if data.current_select.is_none() {
                            // 选择的棋子
                            let piece = data.broad_map[piece.row][piece.col].unwrap();
                            data.current_select = Some(piece);
                            // 播放音效
                            commands.spawn(super::audio::play_once(sounds.select.clone()));
                            // todo 抬起棋子 怎么抬？
                        } else {
                            // 落子或取消 todo 空棋子也要做成按钮
                            data.current_select = None;
                            // 播放音效
                            commands.spawn(super::audio::play_once(sounds.go.clone()));
                        }
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
