use bevy::prelude::Resource;

pub mod asset;

#[derive(Resource)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
