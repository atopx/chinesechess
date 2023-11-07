use bevy::prelude::*;

use super::{ChessState, GameState};
use crate::{
    component::piece::Side,
    game::{Data, GameMode},
    player::Id,
    public,
};

// 主菜单配置
pub const PENDING_MANU_NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const PENDING_MANU_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.30, 0.30);
pub const PENDING_MANU_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);
pub const PENDING_MENU_AI_GAME_TEXT: &str = "人机对弈";
pub const PENDING_MENU_INTER_GAME_TEXT: &str = "联机对弈";
pub const PENDING_MENU_DEDUCE_GAME_TEXT: &str = "打谱推演";
pub const PENDING_MENU_SETTING_GAME_TEXT: &str = "系统设置";
pub const PENDING_MENU_EXIT_GAME_TEXT: &str = "退出游戏";

/// MainMenu 游戏全局菜单, ESC呼出, 位于界面中央
#[derive(Component)]
pub enum PendingMenu {
    // 人机对弈
    NewAiGame,
    // 打谱推演
    NewDeduceGame,
    // 联机对弈
    NewInterGame,
    // 系统设置
    SettingGame,
    // 退出游戏
    ExitGame,
}

pub fn enter_state(
    mut commands: Commands,
    fonts: Res<public::asset::Fonts>,
    // mut entitys: ResMut<public::EntityResources>,
) {
    info!("进入PENDING");
    let menus = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100_f32),
                height: Val::Percent(100_f32),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                PENDING_MENU_AI_GAME_TEXT,
                PendingMenu::NewAiGame,
                16_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                PENDING_MENU_INTER_GAME_TEXT,
                PendingMenu::NewInterGame,
                26_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                PENDING_MENU_DEDUCE_GAME_TEXT,
                PendingMenu::NewDeduceGame,
                36_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                PENDING_MENU_SETTING_GAME_TEXT,
                PendingMenu::SettingGame,
                46_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                PENDING_MENU_EXIT_GAME_TEXT,
                PendingMenu::ExitGame,
                56_f32,
            );
        })
        .id();
    commands.insert_resource(public::EntityResources {
        pending_menus: Some(menus),
        paused_menus: None,
        chessbroad: None,
        selected: None,
    });
}

pub fn in_state(
    mut game_state: ResMut<NextState<GameState>>,
    mut chess_state: ResMut<NextState<ChessState>>,
    mut data: ResMut<Data>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &PendingMenu),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interact, mut color, menu) in &mut query {
        match *interact {
            Interaction::Pressed => {
                *color = PENDING_MANU_PRESSED_BUTTON_COLOR.into();
                match menu {
                    PendingMenu::NewAiGame => {
                        info!("todo NewAiGame");
                        data.mode = Some(GameMode::AiGame);
                        data.ai_side = Some(Side::Black);
                        data.black_player.id = Id::Ai;
                        data.white_player.id = Id::Home;
                        data.current_side = Some(Side::White);
                        data.engine.from_fen(public::START_POS);
                        game_state.set(GameState::RUNNING);
                        chess_state.set(ChessState::HomePlay);
                    }

                    PendingMenu::NewDeduceGame => {
                        info!("todo NewDeduceGame");
                        game_state.set(GameState::RUNNING);
                    }

                    PendingMenu::NewInterGame => {
                        info!("todo NewDeduceGame");
                        game_state.set(GameState::RUNNING);
                    }

                    PendingMenu::SettingGame => {
                        info!("todo SettingGame");
                        game_state.set(GameState::RUNNING);
                    }

                    PendingMenu::ExitGame => {
                        info!("ExitGame");
                        game_state.set(GameState::EXITED);
                    }
                }
            }
            Interaction::Hovered => {
                *color = PENDING_MANU_HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = PENDING_MANU_NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn exit_state(mut commands: Commands, entitys: Res<public::EntityResources>) {
    info!("退出PENDING");
    commands.entity(entitys.pending_menus.unwrap()).despawn_recursive();
}

fn make_main_menu_text_bundle(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    menu: PendingMenu,
    top_px: f32,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(204_f32),
                    height: Val::Px(65_f32),
                    top: Val::Percent(top_px),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: PENDING_MANU_NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            menu,
        ))
        .with_children(|text_parent| {
            text_parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
