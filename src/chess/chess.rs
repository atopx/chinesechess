use bevy::prelude::*;

use crate::{component, game::Data, player::Player, public};

use super::event;

// 棋子系统
pub fn game_chess_system(
    mut commands: Commands,
    mut data: ResMut<Data>,
    mut swith_player_event: EventWriter<event::SwithPlayerEvent>,
    mut entitys: ResMut<public::EntityResources>,
    pieces: Res<public::asset::Pieces>,
    images: Res<public::asset::Images>,
    sounds: Res<public::asset::Sounds>,
    mut query: Query<
        (Entity, &Interaction, &component::Piece, &Player, &Style),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, piece, player, style) in &mut query {
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
                            commands.entity(entity).despawn_descendants();
                            // 抬起棋子(加阴影)
                            let selected = commands
                                .entity(entity)
                                .insert((
                                    ButtonBundle {
                                        style: style.clone(),
                                        background_color: BackgroundColor::from(Color::NONE),
                                        ..default()
                                    },
                                    component::PieceSelect,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        image: UiImage::new(images.select_shadow.clone()),
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            left: Val::Percent(10_f32),
                                            top: Val::Percent(40_f32),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                    parent.spawn(ImageBundle {
                                        image: UiImage::new(
                                            pieces.get_handle(&piece, true).unwrap(),
                                        ),
                                        ..default()
                                    });
                                })
                                .id();
                            entitys.selected = Some(selected);
                        } else {
                            // 落子或取消
                            data.current_select = None;
                            if let Some(selected) = entitys.selected {
                                commands.entity(selected).despawn();
                                entitys.selected = None;
                            }
                            // 播放音效
                            commands.spawn(super::audio::play_once(sounds.go.clone()));
                            swith_player_event.send(event::SwithPlayerEvent)
                        }
                    } else {
                        data.current_select = None;
                        if let Some(selected) = entitys.selected {
                            commands.entity(selected).despawn();
                            entitys.selected = None;
                        }
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
