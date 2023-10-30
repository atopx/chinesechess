use bevy::prelude::*;
use chessai::position;

use crate::{
    component::{self, PieceSelect},
    game::Data,
    player::Player,
    public,
};

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
        (
            Entity,
            &Interaction,
            &component::Piece,
            Option<&Player>,
            &Style,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, piece, player, style) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                trace!("{:?} {:?}", piece, player);

                // 选择的非空棋子
                if let Some(player) = player {
                    if let Some(current_select_piece) = data.current_select {
                        // 取消选子
                        data.current_select = None;

                        // 不能吃自己的棋子
                        if piece.color.unwrap() == current_select_piece.color.unwrap() {
                            // invalid音效
                            commands.spawn(super::audio::play_once(sounds.invalid.clone()));
                        } else {
                            // todo 判断是否是合法的吃子
                            // 判断行棋是否合法
                            let iccs = position::pos2iccs(
                                current_select_piece.row,
                                current_select_piece.col,
                                piece.row,
                                piece.col,
                            );
                            trace!("move to eat: iccs: {}", iccs);
                            if data.engine.legal_move(position::iccs2move(&iccs)) {
                                // 合法吃子

                                // 吃子音效
                                commands.spawn(super::audio::play_once(sounds.eat.clone()));
                            } else {
                                // 非法吃子, 取消selected
                                commands
                                    .entity(entitys.selected.unwrap())
                                    .despawn_descendants();
                                // 在原位置渲染原本棋子

                                // invalid音效
                                commands.spawn(super::audio::play_once(sounds.invalid.clone()));
                            }
                        }

                        // 取消选子动画
                        commands.entity(entity).despawn_descendants();
                        // 在此位置插入选择棋子的动画
                        commands
                            .entity(entity)
                            .insert((
                                ButtonBundle {
                                    style: style.clone(),
                                    background_color: BackgroundColor::from(Color::NONE),
                                    ..default()
                                },
                                *piece,
                                player.clone(),
                            ))
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    image: UiImage::new(pieces.get_handle(&piece, false).unwrap()),
                                    ..default()
                                });
                            });
                    } else {
                        // 选棋：只有当前行棋方才能操作自己的棋子
                        if data.current_color == player.color
                            && data.current_color == piece.color.unwrap()
                        {
                            // 设置当前选择棋子
                            data.current_select = Some(*piece);
                            // 播放音效
                            commands.spawn(super::audio::play_once(sounds.select.clone()));
                            // 删除原有棋子渲染
                            commands.entity(entity).despawn_descendants();
                            // 渲染抬棋(加阴影)
                            let selected_entity = commands
                                .entity(entity)
                                .insert((
                                    ButtonBundle {
                                        style: style.clone(),
                                        background_color: BackgroundColor::from(Color::NONE),
                                        ..default()
                                    },
                                    *piece,
                                    player.clone(),
                                    PieceSelect,
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
                            entitys.selected = Some(selected_entity);
                        }
                    }
                } else {
                    // 移动点是空位置
                    if let Some(current_select_piece) = data.current_select {
                        // 不论怎样, 取消选子
                        data.current_select = None;
                        // 判断行棋是否合法
                        let iccs = position::pos2iccs(
                            current_select_piece.row,
                            current_select_piece.col,
                            piece.row,
                            piece.col,
                        );
                        trace!("move to none: iccs: {}", iccs);
                        if data.engine.legal_move(position::iccs2move(&iccs)) {
                            // 合法行棋：移动selected到目标位置, 取消selected，在该位置渲染原本棋子，go音效
                            commands.spawn(super::audio::play_once(sounds.go.clone()));
                            swith_player_event.send(event::SwithPlayerEvent);
                        } else {
                            // 非法行棋, 取消selected
                            commands
                                .entity(entitys.selected.unwrap())
                                .despawn_descendants();
                            // 在原位置渲染原本棋子

                            // invalid音效
                            commands.spawn(super::audio::play_once(sounds.invalid.clone()));
                        }
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
