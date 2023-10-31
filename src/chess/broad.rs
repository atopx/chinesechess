use crate::component;
use crate::{game::Data, public};
use bevy::prelude::*;

// first to running
pub fn setup_broad(
    mut commands: Commands,
    mut data: ResMut<Data>,
    image_handles: Res<public::asset::Images>,
    piece_handles: Res<public::asset::Pieces>,
    mut broad_query: Query<&mut Visibility, With<component::Broad>>,
) {
    trace!("进入RUNNING");
    if data.gameing {
        let mut broad_visible = broad_query.single_mut();
        *broad_visible = Visibility::Visible;
        return;
    }
    data.gameing = true;
    // 渲染棋盘
    trace!("渲染棋盘");
    commands
        .spawn((
            SpriteBundle {
                texture: image_handles.broad.clone(),
                transform: Transform::IDENTITY,
                ..Default::default()
            },
            component::Broad,
        ))
        .with_children(|parent| {
            // 渲染棋子
            for (row, pieces) in data.broad_map.iter().enumerate() {
                for (col, piece) in pieces.iter().enumerate() {
                    if let Some(piece) = piece {
                        let (x, y) = public::get_piece_render_percent(row, col);
                        trace!("渲染棋子: {} x:{}, y:{}", piece.name(), x, y);
                        parent.spawn((
                            SpriteBundle {
                                texture: piece_handles.get_handle(piece, false),
                                transform: Transform::from_xyz(x, y, 1_f32),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(76_f32, 76_f32)),
                                    ..default()
                                },
                                ..default()
                            },
                            *piece,
                        ));
                    }
                }
            }
        });
}

pub fn cleanup_chessbroad(mut broad_query: Query<&mut Visibility, With<component::Broad>>) {
    trace!("退出RUNNING");
    let mut broad_visible = broad_query.single_mut();
    *broad_visible = Visibility::Hidden;
}
