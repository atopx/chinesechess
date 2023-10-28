use bevy::prelude::{Resource, Entity};

pub mod path;
pub mod asset;

pub struct Size {
    pub w: f32,
    pub h: f32,
}

// 全局配置
pub const WIN_TITLE: &str = "中国象棋";
pub const BROAD_SIZE: Size = Size { w: 767_f32, h: 842_f32 };
pub const WIN_SIZE: Size = Size {
    w: 1280_f32,
    h: 820_f32,
};
pub const PIECE_SIZE: Size = Size { w: 76_f32, h: 77_f32 };
pub const ROUTE_OFFSET: (u8, u8) = (97, 48);


#[derive(Resource)]
pub struct EntityResources {
    pub main_menus: Option<Entity>,
    pub chessbroad: Option<Entity>,
}