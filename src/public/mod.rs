use bevy::prelude::{Resource, Entity};

pub mod path;
pub mod asset;

pub struct Size {
    pub w: f32,
    pub h: f32,
}

// 全局配置
pub const START_POS: &str = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";
pub const WIN_TITLE: &str = "中国象棋";
pub const BROAD_SIZE: Size = Size { w: 767_f32, h: 842_f32 };
pub const WIN_SIZE: Size = Size {
    w: 1280_f32,
    h: 842_f32,
};
pub const ROUTE_OFFSET: (u8, u8) = (97, 48);

pub fn get_piece_render_percent(row: usize, col: usize) -> (f32, f32) {
    (
        9.9_f32 + (col as f32 * 8.8),
        12_f32 + (row as f32 * 8.0)
    )
}
 
#[derive(Resource)]
pub struct EntityResources {
    pub main_menus: Option<Entity>,
    pub chessbroad: Option<Entity>,
    pub selected: Option<Entity>,
}