use bevy::input::keyboard::KeyboardInput;

use bevy::prelude::*;

use crate::game::{Data, Status};
use crate::public::{MAIN_MANU_HOVERED_BUTTON, MAIN_MANU_NORMAL_BUTTON, MAIN_MANU_PRESSED_BUTTON, MAIN_MENU_CONTINUE_GAME_TEXT, MAIN_MENU_EXIT_GAME_TEXT, MAIN_MENU_AI_GAME_TEXT, MAIN_MENU_DEDUCE_GAME_TEXT, MAIN_MENU_SETTING_GAME_TEXT};
use crate::resource;
use crate::resource::asset::Fonts;

#[derive(Resource)]
pub struct Menus {
    pub main_menus: Entity,
}

#[derive(Component)]
pub enum MainMenu {
    NewAiGame,
    NewDeduceGame,
    ContinueGame,
    SettingGame,
    ExitGame,
}


fn make_main_menu_text_bundle(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    menu: MainMenu,
    top_px: f32,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(158.),
                height: Val::Px(65.),
                top: Val::Px(top_px),
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: MAIN_MANU_NORMAL_BUTTON.into(),
            ..default()
        },
        menu
    )).with_children(|text_parent| {
        text_parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            }));
    });
}

pub fn setup_pending(mut commands: Commands, fonts: Res<Fonts>) {
    info!("进入PENDING");
    let menus = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            make_main_menu_text_bundle(parent, fonts.wenkai.clone(), MAIN_MENU_CONTINUE_GAME_TEXT, MainMenu::ContinueGame, 160_f32);
            make_main_menu_text_bundle(parent, fonts.wenkai.clone(), MAIN_MENU_AI_GAME_TEXT, MainMenu::NewAiGame, 250_f32);
            make_main_menu_text_bundle(parent, fonts.wenkai.clone(), MAIN_MENU_DEDUCE_GAME_TEXT, MainMenu::NewDeduceGame, 340_f32);
            make_main_menu_text_bundle(parent, fonts.wenkai.clone(), MAIN_MENU_SETTING_GAME_TEXT, MainMenu::SettingGame, 430_f32);
            make_main_menu_text_bundle(parent, fonts.wenkai.clone(), MAIN_MENU_EXIT_GAME_TEXT, MainMenu::ExitGame, 520_f32);
        })
        .id();

    commands.insert_resource(Menus { main_menus: menus });
}


pub fn running_state_system(
    mut state: ResMut<NextState<Status>>,
    mut key_events: EventReader<KeyboardInput>,
) {
    for key in key_events.iter() {
        if key.key_code.unwrap() == KeyCode::Escape {
            info!("running to pending");
            state.set(Status::PENDING)
        }
    }
}

pub fn pending_state_system(
    mut state: ResMut<NextState<Status>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenu),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, menu) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = MAIN_MANU_PRESSED_BUTTON.into();
                match menu {
                    MainMenu::NewAiGame => {
                        info!("todo NewAiGame");
                        state.set(Status::RUNNING)
                    }

                    MainMenu::NewDeduceGame => {
                        info!("todo NewDeduceGame");
                        state.set(Status::RUNNING)
                    }

                    MainMenu::ContinueGame => {
                        info!("ContinueGame");
                        state.set(Status::RUNNING)
                    }

                    MainMenu::SettingGame => {
                        info!("todo SettingGame");
                        state.set(Status::RUNNING)
                    }

                    MainMenu::ExitGame => {
                        info!("ExitGame");
                        state.set(Status::EXIT)
                    }
                }
            }
            Interaction::Hovered => {
                *color = MAIN_MANU_HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = MAIN_MANU_NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<Menus>) {
    info!("退出PENDING");
    commands.entity(menu_data.main_menus).despawn_recursive();
}


// first to running
pub fn setup_running(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut data: ResMut<Data>,
    images: Res<resource::asset::Images>,
) {
    info!("进入RUNNING");
    // if data.
    commands.spawn(SpriteBundle {
        texture: images.broad.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 767., y: 842. }),
            ..Default::default()
        },
        ..Default::default()
    });
}
