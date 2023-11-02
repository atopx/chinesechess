use bevy::{prelude::*, window::PrimaryWindow};
use chessai::position::{iccs2move, pos2iccs};

use crate::{
    component::{piece::Piece, ChineseBroadCamera, SelectedPiece},
    game::{BroadEntitys, Data},
    public::{self, get_piece_render_percent},
};

pub fn selection(
    mut data: ResMut<Data>,
    mut entitys: ResMut<BroadEntitys>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    sound_handles: Res<public::asset::Sounds>,
    image_handles: Res<public::asset::Images>,
    piece_handles: Res<public::asset::Pieces>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<ChineseBroadCamera>>,
    mut q_select: Query<&mut Transform, (With<SelectedPiece>, Without<Piece>)>,
    mut q_piece: Query<(&mut Parent, &mut Piece, &mut Transform, &mut Visibility), With<Piece>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if buttons.just_pressed(MouseButton::Left) {
            let (min_x, min_y) = get_piece_render_percent(0, 0);
            let (max_x, max_y) = get_piece_render_percent(10, 9);

            // 判断是否在棋盘内
            if pos.x >= min_x - 27_f32
                && pos.y >= min_y - 27_f32
                && pos.x <= max_x + 27_f32
                && pos.y <= max_y + 27_f32
            {
                // 计算棋盘坐标
                let col = ((pos.x + 274_f32) / 68_f32).round() as usize;
                let row = ((pos.y + 285_f32) / 68_f32).round() as usize;
                let (x, y) = get_piece_render_percent(row, col);

                // 计算选择点是否超出棋子边缘: 选择点到棋心的直线距离是否大于30
                if ((x - pos.x).abs().powi(2) + (y - pos.y).abs().powi(2)).sqrt() > 30_f32 {
                    return;
                }

                let piece_opt = data.broad_map[row][col];

                // 如果当前没有选子并且选择的棋子为空, 跳出
                if data.selected.is_none() && piece_opt.is_none() {
                    return;
                }

                // 选择棋子
                if data.selected.is_none() && piece_opt.is_some() {
                    data.selected = piece_opt;
                    trace!("选择棋子: {}", piece_opt.unwrap().name(),);

                    let (mut parent, piece, mut transform, mut visibile) =
                        q_piece.get_mut(entitys.pieces[row][col].unwrap()).unwrap();

                    // 隐藏棋子
                    *visibile = Visibility::Hidden;

                    // 抬起棋子
                    commands.entity(**parent).with_children(|parent| {
                        let selected_entity = parent
                            .spawn((
                                SpriteBundle {
                                    texture: piece_handles.get_handle(&piece, true),
                                    transform: Transform::from_xyz(x, y, 1_f32),
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
                    // 选棋音效
                    commands.spawn(super::audio::play_once(sound_handles.select.clone()));
                    return;
                }

                // 判断行子或吃子是否合法
                let select_piece = data.selected.unwrap();

                let iccs = pos2iccs(select_piece.row, select_piece.col, row, col);

                // 非法行棋
                if !data.engine.legal_move(iccs2move(&iccs)) {
                    let (_, _, _, mut visibile) =
                        q_piece.get_mut(entitys.pieces[row][col].unwrap()).unwrap();
                    // 取消选择
                    data.selected = None;
                    // 取消选棋子动画
                    commands.entity(entitys.selected.unwrap()).despawn_recursive();
                    // 恢复棋子
                    *visibile = Visibility::Inherited;
                    // 播放无效音效
                    commands.spawn(super::audio::play_once(sound_handles.invalid.clone()));
                    return;
                }

                if piece_opt.is_none() {
                    // todo 移动棋子到空位
                    trace!("棋子{}移动到 row:{} col:{}", data.selected.unwrap().name(), row, col);
                    let mut transform = q_select.single_mut();
                    // 移动次数
                    let (src_x, src_y) =
                        get_piece_render_percent(select_piece.row, select_piece.col);
                    let x_offset = x - src_x;
                    let y_offset = y - src_y;
                    let move_num = x_offset.powi(2) + y_offset.powi(2); // 移动次数
                    let x_single_move = move_num / x_offset;
                    let y_single_move = move_num / y_offset;
                    trace!(
                        "move {} => {}:{} {} {}",
                        move_num,
                        x_single_move,
                        y_single_move,
                        x_offset,
                        y_offset
                    );
                    let trs = &mut transform.translation;
                    while (x - trs.x).max(y - trs.y) > 1. {
                        trs.x += 1.;
                        trs.y += 1.;
                    }
                    trs.x = x;
                    trs.y = y;

                    // 放下棋子
                    let piece_entity = entitys.pieces[select_piece.row][select_piece.col].unwrap();
                    let (mut parent, mut piece, mut transform, mut visibile) =
                        q_piece.get_mut(piece_entity).unwrap();
                    // 取消选棋子动画
                    commands.entity(entitys.selected.unwrap()).despawn_recursive();
                    // 改变棋子位置
                    transform.translation.x = x;
                    transform.translation.y = y;
                    // 改变游戏数据
                    piece.col = col;
                    piece.row = row;
                    data.broad_map[select_piece.row][select_piece.col] = None;
                    data.broad_map[row][col] = Some(*piece);
                    entitys.pieces[select_piece.row][select_piece.col] = None;
                    entitys.pieces[row][col] = Some(piece_entity);
                    // 显示棋子
                    *visibile = Visibility::Inherited;
                    // 播放移动棋子音效
                    commands.spawn(super::audio::play_once(sound_handles.go.clone()));
                    // todo 切换棋手
                } else {
                    // todo 吃子
                    trace!("棋子{}吃{}", data.selected.unwrap().name(), piece_opt.unwrap().name());
                    commands.spawn(super::audio::play_once(sound_handles.eat.clone()));
                }
                // 取消选择
                data.selected = None;
            }
        }
    }
}
