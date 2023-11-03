use std::{thread::sleep, time::Duration};

use crate::{
    component::{piece::Piece, ChineseBroadCamera, SelectedPiece},
    game::{BroadEntitys, ChessState, Data},
    player::{Player, PlayerFocus},
    public::{self, get_piece_render_percent},
};
use bevy::{prelude::*, window::PrimaryWindow};
use chessai::position;

#[derive(Event)]
pub struct AIMoveEvent;

pub fn ai_move_event(
    mut events: EventReader<AIMoveEvent>,
    mut data: ResMut<Data>,
    mut entitys: ResMut<BroadEntitys>,
    mut commands: Commands,
    sound_handles: Res<public::asset::Sounds>,
    image_handles: Res<public::asset::Images>,
    piece_handles: Res<public::asset::Pieces>,
    mut q_select: Query<&mut Transform, (With<SelectedPiece>, Without<Piece>)>,
    mut q_piece: Query<(&mut Parent, &mut Piece, &mut Transform, &mut Visibility), With<Piece>>,
) {
    for event in events.iter() {
        trace!("start AI move {}", data.engine.to_fen());
        let mv = data.engine.search_main(64, 1000);
        let ((src_row, src_col), (dst_row, dst_col)) = position::move2pos(mv);
        trace!("move {mv} {}", position::move2iccs(mv));
        let (parent, mut select_piece, mut select_transform, mut select_visiable) =
            q_piece.get_mut(entitys.pieces[src_row][src_col].unwrap()).unwrap();
        let (src_x, src_y) = get_piece_render_percent(src_row, src_col);
        data.selected = data.broad_map[src_row][src_col];
        // 隐藏棋子
        *select_visiable = Visibility::Hidden;
        // 选棋音效
        commands.spawn(super::audio::play_once(sound_handles.select.clone()));
        // 抬起棋子
        commands.entity(**parent).with_children(|parent| {
            let selected_entity = parent
                .spawn((
                    SpriteBundle {
                        texture: piece_handles.get_handle(&select_piece, true),
                        transform: Transform::from_xyz(src_x, src_y, 1_f32),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(75_f32, 75_f32)),
                            ..default()
                        },
                        ..default()
                    },
                    SelectedPiece,
                ))
                .with_children(|parent| {
                    // 添加阴影
                    parent.spawn(SpriteBundle {
                        texture: image_handles.select_shadow.clone(),
                        transform: Transform::from_xyz(-10., -38., -1_f32),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(62_f32, 74_f32)),
                            flip_x: true,
                            ..default()
                        },
                        ..default()
                    });
                })
                .id();
            entitys.selected = Some(selected_entity);
        });
        sleep(Duration::from_millis(100));
        let piece_opt = data.broad_map[dst_row][dst_col];
        let (dst_x, dst_y) = get_piece_render_percent(dst_row, dst_col);

        commands.get_entity(entitys.selected.unwrap()).unwrap();

        if piece_opt.is_none() {
            // 移动棋子到空位
            trace!("棋子{}移动到 row:{} col:{}", data.selected.unwrap().name(), dst_row, dst_col);
        } else {
            // todo 吃子
            trace!("棋子{}吃{}", data.selected.unwrap().name(), piece_opt.unwrap().name());
            // 删除新位置的棋子
            commands.entity(entitys.pieces[dst_row][dst_col].unwrap()).despawn_recursive();
        }

        let mut select_tf = q_select.single_mut();
        // 移动(直接瞬移)
        select_tf.translation.x = dst_x;
        select_tf.translation.y = dst_y;

        // 放下棋子
        // let piece_entity = entitys.pieces[src_row][src_col].unwrap();
        // let (entity, _, mut piece, mut transform) = q_piece.get_mut(piece_entity).unwrap();
        // 改变棋子位置
        select_transform.translation.x = dst_x;
        select_transform.translation.y = dst_y;
        *select_visiable = Visibility::Inherited;
        // 改变游戏数据
        select_piece.col = dst_col;
        select_piece.row = dst_row;
        let piece_entity = entitys.pieces[src_row][src_col];
        data.broad_map[src_row][src_col] = None;
        data.broad_map[dst_row][dst_col] = Some(*select_piece);
        entitys.pieces[src_row][src_col] = None;
        entitys.pieces[dst_row][dst_col] = piece_entity;

        // 取消选棋子动画
        commands.entity(entitys.selected.unwrap()).despawn_recursive();

        // 切换棋手
        data.change_side();
        data.engine.make_move(mv);

        // 检测是否胜利
        if let Some(winner) = data.engine.winner() {
            match winner {
                chessai::pregen::Winner::White => {
                    // todo 红方胜利
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
            data.state = Some(ChessState::Over);
            return;
        }
        // 检测是否将军
        if data.engine.in_check() {
            // 将军
            trace!("将军");
            commands.spawn(super::audio::play_once(sound_handles.check.clone()));
        } else {
            // 是否吃子
            if data.engine.captured() {
                // 吃子
                commands.spawn(super::audio::play_once(sound_handles.eat.clone()));
            } else {
                // 移动
                commands.spawn(super::audio::play_once(sound_handles.go.clone()));
            }
        }
    }
}
