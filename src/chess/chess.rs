use bevy::{prelude::*, window::PrimaryWindow};
use chessai::{
    position::{self, iccs2move, pos2iccs},
    util,
};

use crate::{
    component::{piece::Piece, ChineseBroadCamera},
    game::Data,
    public::{self, get_piece_render_percent},
};

pub fn selection(
    mut data: ResMut<Data>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    sounds: Res<public::asset::Sounds>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<ChineseBroadCamera>>,
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
                    // todo
                    data.selected = piece_opt;
                    trace!("选择棋子: {}", piece_opt.unwrap().name());
                    commands.spawn(super::audio::play_once(sounds.select.clone()));
                    return;
                }

                // 判断行子或吃子是否合法
                let select_piece = data.selected.unwrap();

                let iccs = pos2iccs(select_piece.row, select_piece.col, row, col);

                // 非法行棋
                if !data.engine.legal_move(iccs2move(&iccs)) {
                    // todo 取消选择并警告
                    data.selected = None;
                    commands.spawn(super::audio::play_once(sounds.invalid.clone()));
                    return;
                }

                if piece_opt.is_none() {
                    // todo 移动棋子到空位
                    trace!(
                        "棋子{}移动到 row:{} col:{}",
                        data.selected.unwrap().name(),
                        row,
                        col
                    );
                    commands.spawn(super::audio::play_once(sounds.go.clone()));
                } else {
                    // todo 吃子
                    trace!(
                        "棋子{}吃{}",
                        data.selected.unwrap().name(),
                        piece_opt.unwrap().name()
                    );
                    commands.spawn(super::audio::play_once(sounds.eat.clone()));
                }
                // 取消选择
                data.selected = None;
            }
        }
    }
}
