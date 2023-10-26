use bevy::prelude::Color;

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


// 主菜单配置
pub const MAIN_MANU_NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const MAIN_MANU_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.30, 0.30);
pub const MAIN_MANU_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

pub const MAIN_MENU_CONTINUE_GAME_TEXT: &str = " 继续游戏";
pub const MAIN_MENU_AI_GAME_TEXT: &str = " 人机对弈";
pub const MAIN_MENU_INTER_GAME_TEXT: &str = " 联机对弈";
pub const MAIN_MENU_DEDUCE_GAME_TEXT: &str = " 打谱推演";
pub const MAIN_MENU_SETTING_GAME_TEXT: &str = " 系统设置";
pub const MAIN_MENU_EXIT_GAME_TEXT: &str = " 退出游戏";


// 游戏菜单配置
pub const GAME_MENU_NORMAL_BUTTON_COLOR: Color = Color::NONE;
pub const GAME_MENU_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.30, 0.30);
pub const GAME_MENU_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

pub const GAME_MENU_NEW_GAME_TEXT: &str = "  新局";
pub const GAME_MENU_RETRACT_TEXT: &str = "  悔棋";
pub const GAME_MENU_PEACE_TEXT: &str = "  求和";
pub const GAME_MENU_PROMPT_TEXT: &str = "  提示";
pub const GAME_MENU_ADMIT_DEFEAT_TEXT: &str = "  认输";
pub const GAME_MENU_SWAP_TEXT: &str = "  换边";
pub const GAME_MENU_ROLL_TEXT: &str = "  翻转";

// 棋子坐标配置
pub const PIECE_POS_MAP: [[(f32, f32); 9]; 10] = [
    [(70_f32, 100_f32), (138_f32, 100_f32), (207_f32, 100_f32), (275_f32, 100_f32), (344_f32, 100_f32), (413_f32, 100_f32), (480_f32, 100_f32), (549_f32, 100_f32), (618_f32, 100_f32)],
    [(70_f32, 168_f32), (138_f32, 168_f32), (207_f32, 168_f32), (275_f32, 168_f32), (344_f32, 168_f32), (413_f32, 168_f32), (480_f32, 168_f32), (549_f32, 168_f32), (618_f32, 168_f32)],
    [(70_f32, 236_f32), (138_f32, 236_f32), (207_f32, 236_f32), (275_f32, 236_f32), (344_f32, 236_f32), (413_f32, 236_f32), (480_f32, 236_f32), (549_f32, 236_f32), (618_f32, 236_f32)],
    [(70_f32, 304_f32), (138_f32, 304_f32), (207_f32, 304_f32), (275_f32, 304_f32), (344_f32, 304_f32), (413_f32, 304_f32), (480_f32, 304_f32), (549_f32, 304_f32), (618_f32, 304_f32)],
    [(70_f32, 372_f32), (138_f32, 372_f32), (207_f32, 372_f32), (275_f32, 372_f32), (344_f32, 372_f32), (413_f32, 372_f32), (480_f32, 372_f32), (549_f32, 372_f32), (618_f32, 372_f32)],
    [(70_f32, 440_f32), (138_f32, 440_f32), (207_f32, 440_f32), (275_f32, 440_f32), (344_f32, 440_f32), (413_f32, 440_f32), (480_f32, 440_f32), (549_f32, 440_f32), (618_f32, 440_f32)],
    [(70_f32, 508_f32), (138_f32, 508_f32), (207_f32, 508_f32), (275_f32, 508_f32), (344_f32, 508_f32), (413_f32, 508_f32), (480_f32, 508_f32), (549_f32, 508_f32), (618_f32, 508_f32)],
    [(70_f32, 576_f32), (138_f32, 576_f32), (207_f32, 576_f32), (275_f32, 576_f32), (344_f32, 576_f32), (413_f32, 576_f32), (480_f32, 576_f32), (549_f32, 576_f32), (618_f32, 576_f32)],
    [(70_f32, 644_f32), (138_f32, 644_f32), (207_f32, 644_f32), (275_f32, 644_f32), (344_f32, 644_f32), (413_f32, 644_f32), (480_f32, 644_f32), (549_f32, 644_f32), (618_f32, 644_f32)],
    [(70_f32, 712_f32), (138_f32, 712_f32), (207_f32, 712_f32), (275_f32, 712_f32), (344_f32, 712_f32), (413_f32, 712_f32), (480_f32, 712_f32), (549_f32, 712_f32), (618_f32, 712_f32)],
];
