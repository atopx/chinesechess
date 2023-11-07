use crate::{
    component::piece::Side,
    event::GameoverEvent,
    game::{Data, GameMode},
    public,
    status::ChessState,
};
use bevy::prelude::*;

pub fn event_listen(
    mut data: ResMut<Data>,
    mut commands: Commands,
    mut entitys: ResMut<public::BroadEntitys>,
    mut events: EventReader<GameoverEvent>,
    sound_handles: Res<public::asset::Sounds>,
    image_handles: Res<public::asset::Images>,
    mut chess_state: ResMut<NextState<ChessState>>,
) {
    for event in events.iter() {
        let (sound, image) = match event.0 {
            chessai::pregen::Winner::White => {
                info!("红方胜利");
                match data.mode.unwrap() {
                    GameMode::AiGame => match data.ai_side.unwrap() {
                        Side::White => {
                            (sound_handles.loss.clone(), image_handles.flag_loss.clone())
                        }
                        Side::Black => (sound_handles.win.clone(), image_handles.flag_win.clone()),
                    },
                    GameMode::DeduceGame => {
                        (sound_handles.win.clone(), image_handles.flag_win.clone())
                    }
                    GameMode::InterGame => {
                        // todo
                        (sound_handles.win.clone(), image_handles.flag_win.clone())
                    }
                }
            }
            chessai::pregen::Winner::Black => {
                info!("黑方胜利");
                match data.mode.unwrap() {
                    GameMode::AiGame => match data.ai_side.unwrap() {
                        Side::White => (sound_handles.win.clone(), image_handles.flag_win.clone()),
                        Side::Black => {
                            (sound_handles.loss.clone(), image_handles.flag_loss.clone())
                        }
                    },
                    GameMode::DeduceGame => {
                        (sound_handles.win.clone(), image_handles.flag_win.clone())
                    }
                    GameMode::InterGame => {
                        // todo
                        (sound_handles.win.clone(), image_handles.flag_win.clone())
                    }
                }
            }
            chessai::pregen::Winner::Tie => {
                info!("和棋");
                (sound_handles.draw.clone(), image_handles.flag_draw.clone())
            }
        };

        commands.spawn(super::audio::play_once(sound));
        let gameover = commands
            .spawn(SpriteBundle {
                texture: image,
                transform: Transform::from_xyz(0., 0., 1_f32),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(320_f32, 80_f32)),
                    ..default()
                },
                ..default()
            })
            .id();
        entitys.gameover = Some(gameover);

        chess_state.set(ChessState::Gameover);
        return;
    }
}
