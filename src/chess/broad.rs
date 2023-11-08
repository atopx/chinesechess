use crate::chess::previou::{PiecePreviouEnd, PiecePreviouStart, PiecePreviouTimer};
use crate::component;
use crate::event::{EventAction, GameChangeEvent};
use crate::public::BroadEntitys;
use crate::{game::Data, public};
use bevy::prelude::*;

pub fn event_listen(
    mut events: EventReader<GameChangeEvent>,
    mut commands: Commands,
    mut data: ResMut<Data>,
    mut entitys: ResMut<BroadEntitys>,
    image_handles: Res<public::asset::Images>,
    animate_handles: Res<public::asset::Animates>,
    piece_handles: Res<public::asset::Pieces>,
    mut broad_query: Query<(Entity, &mut Visibility), With<component::Broad>>,
) {
    for event in events.iter() {
        match event.0 {
            EventAction::Spawn => {
                // 渲染棋盘
                info!("渲染棋盘");
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
                        // 渲染历史移动棋子的动画
                        parent.spawn((
                            SpriteBundle {
                                texture: image_handles.start_pos.clone(),
                                visibility: Visibility::Hidden,
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(20_f32, 20_f32)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.1),
                                ..default()
                            },
                            PiecePreviouStart,
                        ));

                        parent.spawn((
                            SpriteSheetBundle {
                                sprite: TextureAtlasSprite {
                                    index: 0,
                                    custom_size: Some(Vec2::new(76_f32, 76_f32)),
                                    ..default()
                                },
                                visibility: Visibility::Hidden,
                                texture_atlas: animate_handles.endposflag.clone(),
                                transform: Transform::from_xyz(0., 0., 1.1_f32),
                                ..default()
                            },
                            PiecePreviouEnd,
                            PiecePreviouTimer::default(),
                        ));

                        // 渲染棋子
                        for (row, pieces) in data.broad_map.iter_mut().enumerate() {
                            for (col, piece) in pieces.iter_mut().enumerate() {
                                if let Some(piece) = piece {
                                    let (x, y) = public::get_piece_render_percent(row, col);
                                    info!("渲染棋子: {} x:{}, y:{}", piece.name(), x, y);
                                    let entity = parent
                                        .spawn((
                                            SpriteBundle {
                                                texture: piece_handles.get_handle(&piece, false),
                                                transform: Transform::from_xyz(x, y, 1_f32),
                                                sprite: Sprite {
                                                    custom_size: Some(Vec2::new(76_f32, 76_f32)),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            *piece,
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
            EventAction::Hidden => {
                let (_, mut broad_visible) = broad_query.single_mut();
                *broad_visible = Visibility::Hidden;
            }
            EventAction::Despawn => {
                let (entity, _) = broad_query.single_mut();
                commands.entity(entity).despawn_recursive();
            }
            EventAction::Visibie => {
                let (_, mut broad_visible) = broad_query.single_mut();
                *broad_visible = Visibility::Inherited;
            }
        }
    }
}
