use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::component::Piece;
use crate::game::{Data, Status};
use crate::public;
use crate::public::{BROAD_SIZE, MAIN_MANU_HOVERED_BUTTON, MAIN_MANU_NORMAL_BUTTON, MAIN_MANU_PRESSED_BUTTON, MAIN_MENU_AI_GAME_TEXT, MAIN_MENU_CONTINUE_GAME_TEXT, MAIN_MENU_DEDUCE_GAME_TEXT, MAIN_MENU_EXIT_GAME_TEXT, MAIN_MENU_SETTING_GAME_TEXT, PIECE_POS_MAP, PIECE_SIZE, WIN_SIZE};

#[derive(Resource)]
pub struct EntityResources {
    pub main_menus: Option<Entity>,
    pub chessbroad: Option<Entity>,
}


#[derive(Component)]
pub enum MainMenu {
    NewAiGame,
    NewDeduceGame,
    ContinueGame,
    SettingGame,
    ExitGame,
}

fn make_piece_bundle(
    parent: &mut ChildBuilder,
    image: Handle<Image>,
    piece: &Piece,
    left: f32,
    bottom: f32,
) -> Entity {
    trace!("渲染棋子: {:?} {:?} (left {}; bottom: {})", piece.color, piece.cate, left, bottom);
    parent.spawn((
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                bottom: Val::Px(bottom),
                height: Val::Px(PIECE_SIZE.h - 3_f32),
                width: Val::Px(PIECE_SIZE.w - 3_f32),
                ..Default::default()
            },
            background_color: BackgroundColor::from(Color::NONE),
            ..default()
        },
        piece.cate,
        piece.color,
    )).with_children(|parent| {
        parent.spawn(ImageBundle {
            image: UiImage::new(image),
            ..default()
        });
    }).id()
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

pub fn setup_pending(mut commands: Commands, fonts: Res<public::asset::Fonts>) {
    trace!("进入PENDING");
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

    commands.insert_resource(EntityResources {
        main_menus: Some(menus),
        chessbroad: None,
    });
}


pub fn esc_event_system(
    mut commands: Commands,
    app_state: Res<State<Status>>,
    mut state: ResMut<NextState<Status>>,
    mut key_events: EventReader<KeyboardInput>,
    mut data: ResMut<Data>,
) {
    for key in key_events.iter() {
        if Some(KeyCode::Escape) == key.key_code && key.state.is_pressed() {
            match app_state.get() {
                Status::PENDING => {
                    if !data.previous_state.is_none() {
                        state.set(Status::RUNNING);
                    }
                }
                Status::RUNNING => {
                    trace!("running to pending");
                    data.previous_state = Some(Status::RUNNING);
                    state.set(Status::PENDING);
                }
                Status::EXIT => {}
            }
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
                        trace!("todo NewAiGame");
                        state.set(Status::RUNNING)
                    }

                    MainMenu::NewDeduceGame => {
                        trace!("todo NewDeduceGame");
                        state.set(Status::RUNNING)
                    }

                    MainMenu::ContinueGame => {
                        trace!("ContinueGame");
                        state.set(Status::RUNNING)
                    }

                    MainMenu::SettingGame => {
                        trace!("todo SettingGame");
                        state.set(Status::RUNNING)
                    }

                    MainMenu::ExitGame => {
                        trace!("ExitGame");
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

pub fn cleanup_menu(mut commands: Commands, entity: Res<EntityResources>) {
    trace!("退出PENDING");
    commands.entity(entity.main_menus.unwrap()).despawn_recursive();
}

pub fn cleanup_chessbroad(mut commands: Commands, entity: Res<EntityResources>) {
    trace!("退出RUNNING");
    commands.entity(entity.chessbroad.unwrap()).despawn_recursive();
}


// first to running
pub fn setup_running(
    mut commands: Commands,
    mut data: ResMut<Data>,
    mut entity: ResMut<EntityResources>,
    images: Res<public::asset::Images>,
    pieces: Res<public::asset::Pieces>,
) {
    trace!("进入RUNNING");

    if !entity.chessbroad.is_none() {
        trace!("respawn");
        commands.get_or_spawn(entity.chessbroad.unwrap());
        return;
    }

    // 开局动画

    // 棋盘
    // 39/1000
    let broad_left = (WIN_SIZE.w - BROAD_SIZE.w) / 2_f32;  // min x
    let broad_bottom = 12_f32;
    trace!("渲染棋盘: left {}, bottom {}", broad_left, broad_bottom);
    let chessbroad_entity = commands.spawn(NodeBundle {
        background_color: BackgroundColor(Color::NONE),
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(broad_left),
            bottom: Val::Px(broad_bottom),
            width: Val::Px(BROAD_SIZE.w),
            height: Val::Px(BROAD_SIZE.h),
            ..default()
        },
        ..default()
    })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(images.broad.clone()),
                ..default()
            });
        })
        .with_children(|parent| {
            for (row, rows_data) in data.broad_map.iter_mut().enumerate() {
                for (col, piece) in rows_data.iter_mut().enumerate() {
                    if let Some(image) = pieces.get_handle(piece, false) {
                        let (left, bottom) = PIECE_POS_MAP[row][col];
                        piece.entity = Some(make_piece_bundle(parent, image, piece, left, bottom));
                    }
                }
            }
        }).id();
    entity.chessbroad = Some(chessbroad_entity);
}
