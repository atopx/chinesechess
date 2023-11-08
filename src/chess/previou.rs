use bevy::prelude::*;

use crate::public::{get_piece_render_percent, Pos};

#[derive(Component)]
pub struct PiecePreviouStart;

#[derive(Component)]
pub struct PiecePreviouEnd;

#[derive(Component)]
pub struct PiecePreviouTimer(Timer);

#[derive(Component)]
pub struct PiecePreviouMove(pub Pos, pub Pos);

pub fn piece_previou_move(
    mut commands: Commands,
    mut params: ParamSet<(
        Query<(&mut Transform, &mut Visibility), With<PiecePreviouStart>>,
        Query<(&mut Transform, &mut Visibility), With<PiecePreviouEnd>>,
    )>,
    query: Query<(Entity, &PiecePreviouMove), With<PiecePreviouMove>>,
) {
    for (entity, previou) in query.iter() {
        let PiecePreviouMove(src, dst) = previou;

        for (mut tf, mut vis) in params.p0().iter_mut() {
            *vis = Visibility::Hidden;
            (tf.translation.x, tf.translation.y) = get_piece_render_percent(src.row, src.col);
            *vis = Visibility::Inherited;
        }

        for (mut tf, mut vis) in params.p1().iter_mut() {
            *vis = Visibility::Hidden;
            (tf.translation.x, tf.translation.y) = get_piece_render_percent(dst.row, dst.col);
            *vis = Visibility::Inherited;
        }

        commands.entity(entity).despawn();
    }
}

impl Default for PiecePreviouTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

pub fn piece_previou_animate(
    time: Res<Time>,
    mut query: Query<(&mut PiecePreviouTimer, &mut TextureAtlasSprite), With<PiecePreviouEnd>>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = if sprite.index >= 7 {
                0
            } else {
                sprite.index + 1
            };
        }
    }
}
