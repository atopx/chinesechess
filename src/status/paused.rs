use bevy::prelude::*;

use super::GameState;
use crate::public;

// 主菜单配置
pub const PAUSED_MANU_NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const PAUSED_MANU_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.30, 0.30);
pub const PAUSED_MANU_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

pub const PAUSED_MENU_CONTINUE_GAME_TEXT: &str = "继续游戏";
pub const PAUSED_MENU_SETTING_GAME_TEXT: &str = "系统设置";
pub const PAUSED_MENU_EXIT_GAME_TEXT: &str = "返回主页";

/// PausedMenu 游戏全局菜单, ESC呼出, 位于界面中央
#[derive(Component)]
pub enum PausedMenu {
    // 继续游戏
    ContinueGame,
    // 系统设置
    SettingGame,
    // 返回主菜单
    PendingMenu,
}

pub fn enter_state(
    mut commands: Commands,
    fonts: Res<public::asset::Fonts>,
    mut entitys: ResMut<public::EntityResources>,
) {
    info!("进入PAUSED");
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
                PAUSED_MENU_CONTINUE_GAME_TEXT,
                PausedMenu::ContinueGame,
                16_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                PAUSED_MENU_SETTING_GAME_TEXT,
                PausedMenu::SettingGame,
                26_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                PAUSED_MENU_EXIT_GAME_TEXT,
                PausedMenu::PendingMenu,
                36_f32,
            );
        })
        .id();
    entitys.paused_menus = Some(menus);
}

pub fn in_state(
    mut state: ResMut<NextState<GameState>>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &PausedMenu),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interact, mut color, menu) in &mut query {
        match *interact {
            Interaction::Pressed => {
                *color = PAUSED_MANU_PRESSED_BUTTON_COLOR.into();
                match menu {
                    PausedMenu::ContinueGame => {
                        info!("todo ContinueGame");
                        state.set(GameState::RUNNING);
                    }

                    PausedMenu::SettingGame => {
                        info!("todo SettingGame");
                        state.set(GameState::RUNNING);
                    }

                    PausedMenu::PendingMenu => {
                        info!("todo PendingMenu");
                        state.set(GameState::PENDING);
                    }
                }
            }
            Interaction::Hovered => {
                *color = PAUSED_MANU_HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = PAUSED_MANU_NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn exit_state(mut commands: Commands, entitys: Res<public::EntityResources>) {
    info!("退出PAUSED");
    commands.entity(entitys.paused_menus.unwrap()).despawn_recursive();
}

fn make_main_menu_text_bundle(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    menu: PausedMenu,
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
                background_color: PAUSED_MANU_NORMAL_BUTTON_COLOR.into(),
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
