use bevy::prelude::*;

use crate::game::Status;
use crate::public;

// 主菜单配置
pub const MAIN_MANU_NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const MAIN_MANU_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.30, 0.30);
pub const MAIN_MANU_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

pub const MAIN_MENU_CONTINUE_GAME_TEXT: &str = "继续游戏";
pub const MAIN_MENU_AI_GAME_TEXT: &str = "人机对弈";
pub const MAIN_MENU_INTER_GAME_TEXT: &str = "联机对弈";
pub const MAIN_MENU_DEDUCE_GAME_TEXT: &str = "打谱推演";
pub const MAIN_MENU_SETTING_GAME_TEXT: &str = "系统设置";
pub const MAIN_MENU_EXIT_GAME_TEXT: &str = "退出游戏";

/// MainMenu 游戏全局菜单, ESC呼出, 位于界面中央
#[derive(Component)]
pub enum MainMenu {
    // 人机对弈
    NewAiGame,
    // 打谱推演
    NewDeduceGame,
    // 联机对弈
    NewInterGame,
    // 继续游戏
    ContinueGame,
    // 系统设置
    SettingGame,
    // 退出游戏
    ExitGame,
}

fn make_main_menu_text_bundle(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    menu: MainMenu,
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
                background_color: MAIN_MANU_NORMAL_BUTTON_COLOR.into(),
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

pub fn setup_pending(mut commands: Commands, fonts: Res<public::asset::Fonts>) {
    trace!("进入PENDING");
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
                MAIN_MENU_CONTINUE_GAME_TEXT,
                MainMenu::ContinueGame,
                16_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                MAIN_MENU_AI_GAME_TEXT,
                MainMenu::NewAiGame,
                26_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                MAIN_MENU_INTER_GAME_TEXT,
                MainMenu::NewInterGame,
                36_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                MAIN_MENU_DEDUCE_GAME_TEXT,
                MainMenu::NewDeduceGame,
                46_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                MAIN_MENU_SETTING_GAME_TEXT,
                MainMenu::SettingGame,
                56_f32,
            );
            make_main_menu_text_bundle(
                parent,
                fonts.xiaoli.clone(),
                MAIN_MENU_EXIT_GAME_TEXT,
                MainMenu::ExitGame,
                66_f32,
            );
        })
        .id();

    commands.insert_resource(public::EntityResources {
        main_menus: Some(menus),
        chessbroad: None,
        selected: None,
    });
}

pub fn pending_state_system(
    mut state: ResMut<NextState<Status>>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenu),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interact, mut color, menu) in &mut query {
        match *interact {
            Interaction::Pressed => {
                *color = MAIN_MANU_PRESSED_BUTTON_COLOR.into();
                match menu {
                    MainMenu::NewAiGame => {
                        trace!("todo NewAiGame");
                        state.set(Status::RUNNING);
                    }

                    MainMenu::NewDeduceGame => {
                        trace!("todo NewDeduceGame");
                        state.set(Status::RUNNING);
                    }

                    MainMenu::NewInterGame => {
                        trace!("todo NewDeduceGame");
                        state.set(Status::RUNNING);
                    }

                    MainMenu::ContinueGame => {
                        trace!("ContinueGame");
                        state.set(Status::RUNNING);
                    }

                    MainMenu::SettingGame => {
                        trace!("todo SettingGame");
                        state.set(Status::RUNNING);
                    }

                    MainMenu::ExitGame => {
                        trace!("ExitGame");
                        state.set(Status::EXIT);
                    }
                }
            }
            Interaction::Hovered => {
                *color = MAIN_MANU_HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = MAIN_MANU_NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, entity: Res<public::EntityResources>) {
    trace!("退出PENDING");
    commands
        .entity(entity.main_menus.unwrap())
        .despawn_recursive();
}
