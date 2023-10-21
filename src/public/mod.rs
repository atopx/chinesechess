use bevy::prelude::Color;

pub mod assets;


pub const WIN_SIZE: (f32, f32) = (1535., 800.);
pub const WIN_TITLE: &str = "中国象棋";
pub const ROUTE_OFFSET: (u8, u8) = (97, 48);


pub const MAIN_MANU_NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const MAIN_MANU_HOVERED_BUTTON: Color = Color::rgb(0.30, 0.30, 0.30);
pub const MAIN_MANU_PRESSED_BUTTON: Color = Color::rgb(0.45, 0.45, 0.45);


pub const MAIN_MENU_CONTINUE_GAME_TEXT: &str = " 继续游戏";
pub const MAIN_MENU_AI_GAME_TEXT: &str = " 人机对战";
pub const MAIN_MENU_DEDUCE_GAME_TEXT: &str = " 打谱推演";
pub const MAIN_MENU_SETTING_GAME_TEXT: &str = " 系统设置";
pub const MAIN_MENU_EXIT_GAME_TEXT: &str = " 退出游戏";
