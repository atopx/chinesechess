use crate::component::PieceColor;
use crate::{component, player};
use crate::{
    game::Data,
    public::{self, BROAD_SIZE, PIECE_SIZE, WIN_SIZE},
};
use bevy::prelude::*;

fn make_piece_bundle(
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
                    left: Val::Px(left),
                    bottom: Val::Px(bottom),
                    height: Val::Px(PIECE_SIZE.h - 3_f32),
                    width: Val::Px(PIECE_SIZE.w - 3_f32),
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
            // background_color: BackgroundColor(Color::NONE),
            style: Style {
                width: Val::Percent(100_f32),
                height: Val::Percent(100_f32),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // 渲染棋盘
            let broad_left = ((WIN_SIZE.w - BROAD_SIZE.w) / 2_f32) / WIN_SIZE.w * 100_f32; // min x
            let broad_bottom = (10_f32);
            trace!("渲染棋盘: left {}, bottom {}", broad_left, broad_bottom);
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
                        for (col, piece_some) in rows_data.iter_mut().enumerate() {
                            if let Some(piece) = piece_some {
                                if let Some(image) = pieces.get_handle(piece, false) {
                                    let (left, bottom) = PIECE_POS_MAP[row][col];
                                    if piece.color == PieceColor::White {
                                        make_piece_bundle(
                                            parent,
                                            data.white_player.clone(),
                                            *piece,
                                            image,
                                            left,
                                            bottom,
                                        );
                                    } else {
                                        make_piece_bundle(
                                            parent,
                                            data.black_player.clone(),
                                            *piece,
                                            image,
                                            left,
                                            bottom,
                                        );
                                    }
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
                        top: Val::Percent(20_f32),
                        left: Val::Percent(6_f32),
                        width: Val::Px(200_f32),
                        height: Val::Px(400_f32),
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
                            top: Val::Percent(10_f32),
                            left: Val::Percent(25_f32),
                            width: Val::Percent(50_f32),
                            height: Val::Percent(25_f32),
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

                    // 计时器

                    // 机器评分
                });

            // 红色方信息框
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(images.player_frame.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(20_f32),
                        right: Val::Percent(6_f32),
                        width: Val::Px(200_f32),
                        height: Val::Px(400_f32),
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
                            top: Val::Percent(10_f32),
                            left: Val::Percent(25_f32),
                            width: Val::Percent(50_f32),
                            height: Val::Percent(25_f32),
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


                    // 计时器

                    // 机器评分
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

// 棋子坐标配置
pub const PIECE_POS_MAP: [[(f32, f32); 9]; 10] = [
    [
        (70_f32, 100_f32),
        (138_f32, 100_f32),
        (207_f32, 100_f32),
        (275_f32, 100_f32),
        (344_f32, 100_f32),
        (413_f32, 100_f32),
        (480_f32, 100_f32),
        (549_f32, 100_f32),
        (618_f32, 100_f32),
    ],
    [
        (70_f32, 168_f32),
        (138_f32, 168_f32),
        (207_f32, 168_f32),
        (275_f32, 168_f32),
        (344_f32, 168_f32),
        (413_f32, 168_f32),
        (480_f32, 168_f32),
        (549_f32, 168_f32),
        (618_f32, 168_f32),
    ],
    [
        (70_f32, 236_f32),
        (138_f32, 236_f32),
        (207_f32, 236_f32),
        (275_f32, 236_f32),
        (344_f32, 236_f32),
        (413_f32, 236_f32),
        (480_f32, 236_f32),
        (549_f32, 236_f32),
        (618_f32, 236_f32),
    ],
    [
        (70_f32, 304_f32),
        (138_f32, 304_f32),
        (207_f32, 304_f32),
        (275_f32, 304_f32),
        (344_f32, 304_f32),
        (413_f32, 304_f32),
        (480_f32, 304_f32),
        (549_f32, 304_f32),
        (618_f32, 304_f32),
    ],
    [
        (70_f32, 372_f32),
        (138_f32, 372_f32),
        (207_f32, 372_f32),
        (275_f32, 372_f32),
        (344_f32, 372_f32),
        (413_f32, 372_f32),
        (480_f32, 372_f32),
        (549_f32, 372_f32),
        (618_f32, 372_f32),
    ],
    [
        (70_f32, 440_f32),
        (138_f32, 440_f32),
        (207_f32, 440_f32),
        (275_f32, 440_f32),
        (344_f32, 440_f32),
        (413_f32, 440_f32),
        (480_f32, 440_f32),
        (549_f32, 440_f32),
        (618_f32, 440_f32),
    ],
    [
        (70_f32, 508_f32),
        (138_f32, 508_f32),
        (207_f32, 508_f32),
        (275_f32, 508_f32),
        (344_f32, 508_f32),
        (413_f32, 508_f32),
        (480_f32, 508_f32),
        (549_f32, 508_f32),
        (618_f32, 508_f32),
    ],
    [
        (70_f32, 576_f32),
        (138_f32, 576_f32),
        (207_f32, 576_f32),
        (275_f32, 576_f32),
        (344_f32, 576_f32),
        (413_f32, 576_f32),
        (480_f32, 576_f32),
        (549_f32, 576_f32),
        (618_f32, 576_f32),
    ],
    [
        (70_f32, 644_f32),
        (138_f32, 644_f32),
        (207_f32, 644_f32),
        (275_f32, 644_f32),
        (344_f32, 644_f32),
        (413_f32, 644_f32),
        (480_f32, 644_f32),
        (549_f32, 644_f32),
        (618_f32, 644_f32),
    ],
    [
        (70_f32, 712_f32),
        (138_f32, 712_f32),
        (207_f32, 712_f32),
        (275_f32, 712_f32),
        (344_f32, 712_f32),
        (413_f32, 712_f32),
        (480_f32, 712_f32),
        (549_f32, 712_f32),
        (618_f32, 712_f32),
    ],
];
