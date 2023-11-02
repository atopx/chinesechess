use crate::component;
use crate::game::BroadEntitys;
use crate::{game::Data, public};
use bevy::prelude::*;

// first to running
pub fn setup_broad(
    mut commands: Commands,
    mut data: ResMut<Data>,
    mut entitys: ResMut<BroadEntitys>,
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
    // 渲染棋盘
    trace!("渲染棋盘");
    let broad_entity = commands
        .spawn((
            SpriteBundle {
                texture: image_handles.broad.clone(),
                transform: Transform::IDENTITY,
                ..default()
            },
            component::Broad,
        ))
        .with_children(|parent| {
            // 渲染棋子
            for (row, pieces) in data.broad_map.iter_mut().enumerate() {
                for (col, piece) in pieces.iter_mut().enumerate() {
                    if let Some(mut piece) = piece {
                        let (x, y) = public::get_piece_render_percent(row, col);
                        trace!("渲染棋子: {} x:{}, y:{}", piece.name(), x, y);
                        let entity = parent
                            .spawn((
                                SpriteBundle {
                                    texture: piece_handles.get_handle(&piece, false),
                                    transform: Transform::from_xyz(x, y, 1_f32),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(74_f32, 74_f32)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                piece,
                            ))
                            .id();
                        entitys.pieces[row][col] = Some(entity);
                    }
                }
            }
        })
        .id();
    entitys.broad = Some(broad_entity);
}

pub fn cleanup_chessbroad(mut broad_query: Query<&mut Visibility, With<component::Broad>>) {
    trace!("退出RUNNING");
    let mut broad_visible = broad_query.single_mut();
    *broad_visible = Visibility::Hidden;
}
