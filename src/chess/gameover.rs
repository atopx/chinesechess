use crate::{event::GameoverEvent, game::Data, public, status::ChessState};
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
        match event.0 {
            chessai::pregen::Winner::White => {
                trace!("红方胜利");
                commands.spawn(super::audio::play_once(sound_handles.loss.clone()));
                let gameover = commands
                    .spawn(SpriteBundle {
                        texture: image_handles.flag_loss.clone(),
                        transform: Transform::from_xyz(0., 0., 1_f32),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(320_f32, 80_f32)),
                            ..default()
                        },
                        ..default()
                    })
                    .id();
                entitys.gameover = Some(gameover);
            }
            chessai::pregen::Winner::Black => {
                // 黑方胜利
                trace!("黑方胜利");
                commands.spawn(super::audio::play_once(sound_handles.win.clone()));
                let gameover = commands
                    .spawn(SpriteBundle {
                        texture: image_handles.flag_win.clone(),
                        transform: Transform::from_xyz(0., 0., 1_f32),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(320_f32, 80_f32)),
                            ..default()
                        },
                        ..default()
                    })
                    .id();
                entitys.gameover = Some(gameover);
            }
            chessai::pregen::Winner::Tie => {
                // 和棋
                trace!("和棋");
                commands.spawn(super::audio::play_once(sound_handles.draw.clone()));
                let gameover = commands
                    .spawn(SpriteBundle {
                        texture: image_handles.flag_draw.clone(),
                        transform: Transform::from_xyz(0., 0., 1_f32),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(320_f32, 80_f32)),
                            ..default()
                        },
                        ..default()
                    })
                    .id();
                entitys.gameover = Some(gameover);
            }
        }
        chess_state.set(ChessState::Gameover);
    }
}
