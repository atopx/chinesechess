use crate::component::PieceColor;
use crate::player::PlayerFocus;
use crate::{component, player};
use crate::{game::Data, public};
use bevy::prelude::*;

fn make_none(parent: &mut ChildBuilder, piece: component::Piece, left: f32, bottom: f32) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(left),
                    bottom: Val::Percent(bottom),
                    height: Val::Percent(9_f32),
                    width: Val::Percent(9_f32),
                    ..Default::default()
                },
                background_color: BackgroundColor::from(Color::NONE),
                ..default()
            },
            piece,
        ))
        .id()
}

pub fn make_piece_bundle(
    parent: &mut ChildBuilder,
    player: player::Player,
    piece: component::Piece,
    image: Handle<Image>,
    left: f32,
    bottom: f32,
) -> Entity {
    trace!("渲染棋子: {:?} (left {}; bottom: {})", piece, left, bottom);
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(left),
                    bottom: Val::Percent(bottom),
                    height: Val::Percent(9_f32),
                    width: Val::Percent(9_f32),
                    ..Default::default()
                },
                background_color: BackgroundColor::from(Color::NONE),
                ..default()
            },
            piece,
            player,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(image),
                ..default()
            });
        })
        .id()
}

// first to running
pub fn setup_running(
    mut commands: Commands,
    mut data: ResMut<Data>,
    mut entity: ResMut<public::EntityResources>,
    images: Res<public::asset::Images>,
    pieces: Res<public::asset::Pieces>,
    fonts: Res<public::asset::Fonts>,
) {
    trace!("进入RUNNING");

    if !entity.chessbroad.is_none() {
        trace!("respawn");
        commands.get_or_spawn(entity.chessbroad.unwrap());
        return;
    }

    // todo 开局动画 选边
    data.set_ai_game(component::PieceColor::White);

    let chessbroad_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100_f32),
                height: Val::Percent(100_f32),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // 渲染棋盘
            trace!("渲染棋盘");
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(images.broad.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(20_f32),
                        bottom: Val::Percent(2_f32),
                        width: Val::Percent(60_f32),
                        height: Val::Percent(100_f32),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let data = &mut *data;

                    // 渲染棋子
                    for (row, rows_data) in data.broad_map.iter_mut().enumerate() {
                        for (col, piece) in rows_data.iter_mut().enumerate() {
                            let (left, bottom) = public::get_piece_render_percent(row, col);
                            match piece.color {
                                Some(PieceColor::White) => {
                                    make_piece_bundle(
                                        parent,
                                        data.white_player.clone(),
                                        *piece,
                                        pieces.get_handle(piece, false).unwrap(),
                                        left,
                                        bottom,
                                    );
                                }
                                Some(PieceColor::Black) => {
                                    make_piece_bundle(
                                        parent,
                                        data.black_player.clone(),
                                        *piece,
                                        pieces.get_handle(piece, false).unwrap(),
                                        left,
                                        bottom,
                                    );
                                }
                                None => {
                                    make_none(parent, *piece, left, bottom);
                                }
                            }
                        }
                    }
                    super::button::make_chess_buttons(parent, fonts.xiaoli.clone());
                });

            // 黑色方信息框
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(images.player_frame.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(8_f32),
                        left: Val::Percent(6_f32),
                        width: Val::Px(200_f32),
                        height: Val::Px(250_f32),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // 头像
                    parent.spawn(ImageBundle {
                        image: UiImage::new(images.black_avatar.clone()),
                        style: Style {
                            position_type: PositionType::Relative,
                            top: Val::Percent(15_f32),
                            left: Val::Percent(25_f32),
                            width: Val::Percent(50_f32),
                            height: Val::Percent(40_f32),
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        style: Style {
                            left: Val::Percent(-10_f32),
                            ..default()
                        },
                        text: Text::from_section(
                            "黑方",
                            TextStyle {
                                font: fonts.xiaoli.clone(),
                                font_size: 25_f32,
                                color: Color::ANTIQUE_WHITE,
                            },
                        ),
                        ..default()
                    });

                    // 局计时器
                    parent.spawn(
                        TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Percent(65_f32),
                                left: Val::Percent(22_f32),
                                ..default()
                            },
                            text: Text::from_section(
                                format!("局时 {}", data.black_player.get_total_timer()),
                                TextStyle {
                                    font: fonts.wenkai.clone(),
                                    font_size: 24_f32,
                                    color: Color::ANTIQUE_WHITE,
                                },
                            ),
                            ..default()
                        }
                        .with_no_wrap(),
                    );

                    // 步计时器
                    parent.spawn(
                        TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Percent(80_f32),
                                left: Val::Percent(22_f32),
                                ..default()
                            },
                            text: Text::from_section(
                                format!(r"步时 {}", data.black_player.get_current_timer()),
                                TextStyle {
                                    font: fonts.wenkai.clone(),
                                    font_size: 24_f32,
                                    color: Color::ANTIQUE_WHITE,
                                },
                            ),
                            ..default()
                        }
                        .with_no_wrap(),
                    );
                });
            // 黑方行动信息
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(images.popup.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(50_f32),
                        left: Val::Percent(6_f32),
                        width: Val::Px(200_f32),
                        height: Val::Px(60_f32),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            data.black_player.get_action(),
                            TextStyle {
                                font: fonts.xiaoli.clone(),
                                font_size: 24_f32,
                                color: Color::DARK_GREEN,
                            },
                        )
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(32_f32),
                            top: Val::Percent(20_f32),
                            ..default()
                        }),
                    );
                });

            // 聚焦框
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(images.player_focus.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(7.5_f32),
                        right: Val::Percent(5.5_f32),
                        width: Val::Px(220_f32),
                        height: Val::Px(262_f32),
                        ..default()
                    },
                    ..default()
                },
                PlayerFocus,
            ));

            // 红色方信息框
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(images.player_frame.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(8_f32),
                        right: Val::Percent(6_f32),
                        width: Val::Px(200_f32),
                        height: Val::Px(250_f32),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // 头像
                    parent.spawn(ImageBundle {
                        image: UiImage::new(images.white_avatar.clone()),
                        style: Style {
                            position_type: PositionType::Relative,
                            top: Val::Percent(15_f32),
                            left: Val::Percent(25_f32),
                            width: Val::Percent(50_f32),
                            height: Val::Percent(40_f32),
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        style: Style {
                            left: Val::Percent(-10_f32),
                            ..default()
                        },
                        text: Text::from_section(
                            "红方",
                            TextStyle {
                                font: fonts.xiaoli.clone(),
                                font_size: 25_f32,
                                color: Color::ANTIQUE_WHITE,
                            },
                        ),
                        ..default()
                    });

                    // 局计时器
                    parent.spawn(
                        TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Percent(65_f32),
                                left: Val::Percent(22_f32),
                                ..default()
                            },
                            text: Text::from_section(
                                format!(r"局时 {}", data.white_player.get_total_timer()),
                                TextStyle {
                                    font: fonts.wenkai.clone(),
                                    font_size: 24_f32,
                                    color: Color::ANTIQUE_WHITE,
                                },
                            ),
                            ..default()
                        }
                        .with_no_wrap(),
                    );

                    // 步计时器
                    parent.spawn(
                        TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Percent(80_f32),
                                left: Val::Percent(22_f32),
                                ..default()
                            },
                            text: Text::from_section(
                                format!("步时 {}", data.white_player.get_current_timer()),
                                TextStyle {
                                    font: fonts.wenkai.clone(),
                                    font_size: 24_f32,
                                    color: Color::ANTIQUE_WHITE,
                                },
                            ),
                            ..default()
                        }
                        .with_no_wrap(),
                    );
                });

            // 红方行动信息
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(images.popup.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(50_f32),
                        right: Val::Percent(6_f32),
                        width: Val::Px(200_f32),
                        height: Val::Px(60_f32),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            data.white_player.get_action(),
                            TextStyle {
                                font: fonts.xiaoli.clone(),
                                font_size: 24_f32,
                                color: Color::DARK_GREEN,
                            },
                        )
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            right: Val::Percent(32_f32),
                            top: Val::Percent(20_f32),
                            ..default()
                        }),
                    );
                });
        })
        .id();
    entity.chessbroad = Some(chessbroad_entity);
}

pub fn cleanup_chessbroad(mut commands: Commands, entity: ResMut<public::EntityResources>) {
    trace!("退出RUNNING");
    commands
        .entity(entity.chessbroad.unwrap())
        .despawn_recursive();
}
