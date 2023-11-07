use crate::component::PlayerInfo;
use crate::event::{EventAction, GameChangeEvent};
use crate::game::Data;
use crate::public;
use bevy::prelude::*;

// 玩家信息标题
#[derive(Component, Debug, Default, Clone)]
pub struct PlayerInfoTitle;

// 玩家动作
#[derive(Component, Debug, Default, Clone)]
pub struct PlayerInfoAction;

// 全局计时器
#[derive(Component, Debug, Default, Clone)]
pub struct PlayerInfoGlobalTimer;

// 步计时器
#[derive(Component, Debug, Default, Clone)]
pub struct PlayerInfoCurrentTimer;

pub fn event_listen(
    mut events: EventReader<GameChangeEvent>,
    mut commands: Commands,
    data: Res<Data>,
    image_handles: Res<public::asset::Images>,
    fonts: Res<public::asset::Fonts>,
    mut query: Query<(Entity, &mut Visibility), With<PlayerInfo>>,
) {
    for event in events.iter() {
        match event.0 {
            EventAction::Spawn => {
                info!("加载游戏玩家信息");
                // 黑色玩家信息框
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(0_f32),
                                bottom: Val::Percent(0_f32),
                                width: Val::Percent(50_f32),
                                height: Val::Percent(100_f32),
                                ..default()
                            },
                            ..default()
                        },
                        PlayerInfo,
                        data.black_player,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(ImageBundle {
                                image: UiImage::new(image_handles.player_frame.clone()),
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    top: Val::Percent(8_f32),
                                    left: Val::Percent(10_f32),
                                    width: Val::Px(200_f32),
                                    height: Val::Px(250_f32),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                // 头像
                                parent.spawn(ImageBundle {
                                    image: UiImage::new(image_handles.black_avatar.clone()),
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

                                // 标题
                                parent.spawn((
                                    TextBundle {
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
                                    },
                                    PlayerInfoTitle,
                                    data.black_player,
                                ));

                                // 局计时器
                                parent.spawn((
                                    TextBundle {
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            top: Val::Percent(65_f32),
                                            left: Val::Percent(22_f32),
                                            ..default()
                                        },
                                        text: Text::from_section(
                                            // data.black_player.get_global_timer(),
                                            "局时: 00:00",
                                            TextStyle {
                                                font: fonts.wenkai.clone(),
                                                font_size: 24_f32,
                                                color: Color::ANTIQUE_WHITE,
                                            },
                                        ),
                                        ..default()
                                    }
                                    .with_no_wrap(),
                                    PlayerInfoGlobalTimer,
                                    data.black_player,
                                ));

                                // 步计时器
                                parent.spawn((
                                    TextBundle {
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            top: Val::Percent(80_f32),
                                            left: Val::Percent(22_f32),
                                            ..default()
                                        },
                                        text: Text::from_section(
                                            // data.black_player.get_current_timer(),
                                            "步时: 00:00",
                                            TextStyle {
                                                font: fonts.wenkai.clone(),
                                                font_size: 24_f32,
                                                color: Color::ANTIQUE_WHITE,
                                            },
                                        ),
                                        ..default()
                                    }
                                    .with_no_wrap(),
                                    PlayerInfoCurrentTimer,
                                    data.black_player,
                                ));
                            });

                        // 黑方行动信息
                        parent
                            .spawn(ImageBundle {
                                image: UiImage::new(image_handles.popup.clone()),
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    top: Val::Percent(50_f32),
                                    left: Val::Percent(10_f32),
                                    width: Val::Px(200_f32),
                                    height: Val::Px(60_f32),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        "空闲中",
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
                                    PlayerInfoAction,
                                    data.black_player,
                                ));
                            });
                    });

                // 红方信息框
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                right: Val::Percent(0_f32),
                                bottom: Val::Percent(0_f32),
                                width: Val::Percent(50_f32),
                                height: Val::Percent(100_f32),
                                ..default()
                            },
                            ..default()
                        },
                        PlayerInfo,
                        data.white_player,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(ImageBundle {
                                image: UiImage::new(image_handles.player_frame.clone()),
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    top: Val::Percent(8_f32),
                                    right: Val::Percent(10_f32),
                                    width: Val::Px(200_f32),
                                    height: Val::Px(250_f32),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                // 头像
                                parent.spawn(ImageBundle {
                                    image: UiImage::new(image_handles.white_avatar.clone()),
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

                                // 标题
                                parent.spawn((
                                    TextBundle {
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
                                    },
                                    PlayerInfoTitle,
                                    data.white_player,
                                ));

                                // 局计时器
                                parent.spawn((
                                    TextBundle {
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            top: Val::Percent(65_f32),
                                            left: Val::Percent(22_f32),
                                            ..default()
                                        },
                                        text: Text::from_section(
                                            // data.white_player.get_global_timer(),
                                            "局时: 00:00",
                                            TextStyle {
                                                font: fonts.wenkai.clone(),
                                                font_size: 24_f32,
                                                color: Color::ANTIQUE_WHITE,
                                            },
                                        ),
                                        ..default()
                                    }
                                    .with_no_wrap(),
                                    PlayerInfoGlobalTimer,
                                    data.white_player,
                                ));

                                // 步计时器
                                parent.spawn((
                                    TextBundle {
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            top: Val::Percent(80_f32),
                                            left: Val::Percent(22_f32),
                                            ..default()
                                        },
                                        text: Text::from_section(
                                            // data.white_player.get_current_timer(),
                                            "步时: 00:00",
                                            TextStyle {
                                                font: fonts.wenkai.clone(),
                                                font_size: 24_f32,
                                                color: Color::ANTIQUE_WHITE,
                                            },
                                        ),
                                        ..default()
                                    }
                                    .with_no_wrap(),
                                    PlayerInfoCurrentTimer,
                                    data.white_player,
                                ));
                            });

                        // 红方行动信息
                        parent
                            .spawn(ImageBundle {
                                image: UiImage::new(image_handles.popup.clone()),
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    top: Val::Percent(50_f32),
                                    right: Val::Percent(10_f32),
                                    width: Val::Px(200_f32),
                                    height: Val::Px(60_f32),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        "空闲中",
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
                                    PlayerInfoAction,
                                    data.white_player,
                                ));
                            });
                    });
            }
            EventAction::Hidden => {
                info!("隐藏游戏玩家信息");
                for (_, mut visibie) in query.iter_mut() {
                    *visibie = Visibility::Hidden;
                }
            }
            EventAction::Despawn => {
                info!("删除游戏玩家信息");
                for (entity, _) in query.iter_mut() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            EventAction::Visibie => {
                info!("显示游戏玩家信息");
                for (_, mut visibie) in query.iter_mut() {
                    *visibie = Visibility::Inherited;
                }
            }
        }
    }
}
