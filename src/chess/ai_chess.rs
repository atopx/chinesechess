use crate::{
    component::{piece::Piece, SelectedPiece},
    event::{GameoverEvent, SwithPlayerEvent},
    game::Data,
    public::{self, get_piece_render_percent, BroadEntitys},
};
use bevy::prelude::*;
use chessai::position;

pub fn ai_move(
    mut data: ResMut<Data>,
    mut entitys: ResMut<BroadEntitys>,
    mut commands: Commands,
    mut gameover: EventWriter<GameoverEvent>,
    mut swith_player: EventWriter<SwithPlayerEvent>,
    sound_handles: Res<public::asset::Sounds>,
    image_handles: Res<public::asset::Images>,
    piece_handles: Res<public::asset::Pieces>,
    mut q_piece: Query<(&mut Parent, &mut Piece, &mut Transform, &mut Visibility), With<Piece>>,
) {
    if data.current_side.unwrap() != data.ai_side.unwrap() {
        return;
    }
    info!("start ai move {}", data.engine.to_fen());
    let mv = data.engine.search_main(64, 1000);
    let ((src_row, src_col), (dst_row, dst_col)) = position::move2pos(mv);
    info!("move {mv} {}", position::move2iccs(mv));
    let (parent, mut select_piece, mut select_transform, mut select_visiable) =
        q_piece.get_mut(entitys.pieces[src_row][src_col].unwrap()).unwrap();
    let (src_x, src_y) = get_piece_render_percent(src_row, src_col);
    data.selected = data.broad_map[src_row][src_col];
    // 隐藏棋子
    *select_visiable = Visibility::Hidden;
    // 选棋音效
    commands.spawn(super::audio::play_once(sound_handles.select.clone()));

    let mut select_tf = Transform::from_xyz(src_x, src_y, 1_f32);

    // 抬起棋子
    commands.entity(parent.get()).with_children(|parent| {
        let selected_entity = parent
            .spawn((
                SpriteBundle {
                    texture: piece_handles.get_handle(&select_piece, true),
                    // transform: Transform::from_xyz(src_x, src_y, 1_f32),
                    transform: select_tf,
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
    let piece_opt = data.broad_map[dst_row][dst_col];
    let (dst_x, dst_y) = get_piece_render_percent(dst_row, dst_col);
    info!("棋子{}移动到 row:{} col:{}", data.selected.unwrap().name(), dst_row, dst_col);

    if piece_opt.is_some() {
        // 吃子: 删除新位置的棋子
        commands.entity(entitys.pieces[dst_row][dst_col].unwrap()).despawn_recursive();
    }

    // 移动(直接瞬移) fixme: 这里是个bug, 在这一帧内获取不到这个实体
    // let mut select_tf = q_select.get_mut(entitys.selected.unwrap()).unwrap();
    select_tf.translation.x = dst_x;
    select_tf.translation.y = dst_y;

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

    // 设置引擎移动
    data.engine.make_move(mv);

    // 检测是否胜利
    if let Some(winner) = data.engine.winner() {
        gameover.send(GameoverEvent(winner));
        return;
    }

    // 检测是否将军
    if data.engine.in_check() {
        // 将军
        info!("将军");
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
    // 切换棋手
    info!("send swith event");
    swith_player.send(SwithPlayerEvent);
}
