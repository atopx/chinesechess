use crate::game::Data;
use crate::game::Status;
use crate::public;
use bevy::prelude::*;

/// GameButton 对局按钮, 位于棋盘正下方
#[derive(Component)]
pub enum ChessButton {
    // 新局
    NewGame,
    // 悔棋
    Retract,
    // 求和
    Peact,
    // 提示
    Prompt,
    // 认输
    AdmitDefeat,
    // 换边
    Swap,
    // 翻转棋盘
    Roll,
}

pub const GAME_MENU_NEW_GAME_TEXT: &str = "  新局";
pub const GAME_MENU_RETRACT_TEXT: &str = "  悔棋";
pub const GAME_MENU_PEACE_TEXT: &str = "  求和";
pub const GAME_MENU_PROMPT_TEXT: &str = "  提示";
pub const GAME_MENU_ADMIT_DEFEAT_TEXT: &str = "  认输";
pub const GAME_MENU_SWAP_TEXT: &str = "  换边";
pub const GAME_MENU_ROLL_TEXT: &str = "  翻转";

// 游戏菜单配置
pub const GAME_MENU_NORMAL_BUTTON_COLOR: Color = Color::NONE;
pub const GAME_MENU_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.30, 0.30);
pub const GAME_MENU_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

pub fn make_chess_buttons(parent: &mut ChildBuilder, font: Handle<Font>) {
    make_text_bundle(
        parent,
        font.clone(),
        GAME_MENU_NEW_GAME_TEXT,
        ChessButton::NewGame,
        60_f32,
    );

    make_text_bundle(
        parent,
        font.clone(),
        GAME_MENU_RETRACT_TEXT,
        ChessButton::Retract,
        150_f32,
    );
    make_text_bundle(
        parent,
        font.clone(),
        GAME_MENU_PEACE_TEXT,
        ChessButton::Peact,
        240_f32,
    );
    make_text_bundle(
        parent,
        font.clone(),
        GAME_MENU_PROMPT_TEXT,
        ChessButton::Prompt,
        330_f32,
    );
    make_text_bundle(
        parent,
        font.clone(),
        GAME_MENU_ADMIT_DEFEAT_TEXT,
        ChessButton::AdmitDefeat,
        420_f32,
    );
    make_text_bundle(
        parent,
        font.clone(),
        GAME_MENU_SWAP_TEXT,
        ChessButton::Swap,
        510_f32,
    );
    make_text_bundle(
        parent,
        font.clone(),
        GAME_MENU_ROLL_TEXT,
        ChessButton::Roll,
        600_f32,
    );
}

pub fn make_text_bundle(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    menu: ChessButton,
    left_px: f32,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(90_f32),
                    height: Val::Px(50_f32),
                    left: Val::Px(left_px),
                    bottom: Val::Px(20_f32),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: GAME_MENU_NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            menu,
        ))
        .with_children(|text_parent| {
            text_parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 22_f32,
                    color: Color::ANTIQUE_WHITE,
                },
            ));
        });
}

pub fn chess_button_system(
    mut data: ResMut<Data>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ChessButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, menu) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = GAME_MENU_PRESSED_BUTTON_COLOR.into();
                match menu {
                    ChessButton::NewGame => {
                        trace!("todo GameMenu NewGame");
                    }
                    ChessButton::Retract => {
                        trace!("todo GameMenu Retract");
                    }
                    ChessButton::Peact => {
                        trace!("todo GameMenu Peact");
                    }
                    ChessButton::Prompt => {
                        trace!("todo GameMenu Prompt");
                    }
                    ChessButton::AdmitDefeat => {
                        trace!("todo GameMenu AdmitDefeat");
                    }
                    ChessButton::Swap => {
                        trace!("todo GameMenu Swap");
                    }
                    ChessButton::Roll => {
                        trace!("todo GameMenu Roll");
                    }
                }
            }
            Interaction::Hovered => {
                *color = GAME_MENU_HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = GAME_MENU_NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
