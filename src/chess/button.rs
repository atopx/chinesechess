use crate::component::ChessButtonGroup;
use crate::event::{EventAction, GameChangeEvent};
use crate::game::Data;
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

pub const GAME_MENU_NEW_GAME_TEXT: &str = "新局";
pub const GAME_MENU_RETRACT_TEXT: &str = "悔棋";
pub const GAME_MENU_PEACE_TEXT: &str = "求和";
pub const GAME_MENU_PROMPT_TEXT: &str = "提示";
pub const GAME_MENU_ADMIT_DEFEAT_TEXT: &str = "认输";
pub const GAME_MENU_SWAP_TEXT: &str = "换边";
pub const GAME_MENU_ROLL_TEXT: &str = "翻转";

// 游戏菜单配置
pub const GAME_MENU_NORMAL_BUTTON_COLOR: Color = Color::NONE;
pub const GAME_MENU_HOVERED_BUTTON_COLOR: Color = Color::rgb(0.30, 0.30, 0.30);
pub const GAME_MENU_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

pub fn event_listen(
    mut events: EventReader<GameChangeEvent>,
    mut commands: Commands,
    fonts: Res<public::asset::Fonts>,
    mut botton_q: Query<(Entity, &mut Visibility), With<ChessButtonGroup>>,
) {
    for event in events.iter() {
        match event.0 {
            EventAction::Spawn => {
                // 创建组件
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(20_f32),
                                bottom: Val::Percent(3_f32),
                                width: Val::Percent(60_f32),
                                height: Val::Px(50_f32),
                                justify_content: JustifyContent::Center,
                                justify_items: JustifyItems::Center,
                                justify_self: JustifySelf::Center,
                                align_content: AlignContent::Center,
                                align_items: AlignItems::Center,
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ChessButtonGroup,
                    ))
                    .with_children(|parent| {
                        make_chess_buttons(parent, fonts.xiaoli.clone());
                    });
            }
            EventAction::Hidden => {
                // 隐藏组件
                info!("隐藏游戏按钮");
                let (_, mut visibie) = botton_q.single_mut();
                *visibie = Visibility::Hidden;
            }
            EventAction::Despawn => {
                // 销毁组件
                let (entity, _) = botton_q.single_mut();
                commands.entity(entity).despawn_recursive();
            }
            EventAction::Visibie => {
                // 显示组件
                info!("显示游戏按钮");
                let (_, mut visibie) = botton_q.single_mut();
                *visibie = Visibility::Inherited;
            }
        }
    }
}

pub fn make_chess_buttons(parent: &mut ChildBuilder, font: Handle<Font>) {
    make_text_bundle(parent, font.clone(), GAME_MENU_NEW_GAME_TEXT, ChessButton::NewGame);
    make_text_bundle(parent, font.clone(), GAME_MENU_RETRACT_TEXT, ChessButton::Retract);
    make_text_bundle(parent, font.clone(), GAME_MENU_PEACE_TEXT, ChessButton::Peact);
    make_text_bundle(parent, font.clone(), GAME_MENU_PROMPT_TEXT, ChessButton::Prompt);
    make_text_bundle(parent, font.clone(), GAME_MENU_ADMIT_DEFEAT_TEXT, ChessButton::AdmitDefeat);
    make_text_bundle(parent, font.clone(), GAME_MENU_SWAP_TEXT, ChessButton::Swap);
    make_text_bundle(parent, font.clone(), GAME_MENU_ROLL_TEXT, ChessButton::Roll);
}

pub fn make_text_bundle(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    menu: ChessButton,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(100_f32),
                    height: Val::Px(50_f32),
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    justify_self: JustifySelf::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                background_color: GAME_MENU_NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            menu,
        ))
        .with_children(|text_parent| {
            text_parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size: 22_f32,
                        color: Color::ANTIQUE_WHITE,
                    },
                )
                .with_style(Style {
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    justify_self: JustifySelf::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                }),
            );
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
                        info!("todo GameMenu NewGame");
                    }
                    ChessButton::Retract => {
                        info!("todo GameMenu Retract");
                    }
                    ChessButton::Peact => {
                        info!("todo GameMenu Peact");
                    }
                    ChessButton::Prompt => {
                        info!("todo GameMenu Prompt");
                    }
                    ChessButton::AdmitDefeat => {
                        info!("todo GameMenu AdmitDefeat");
                    }
                    ChessButton::Swap => {
                        info!("todo GameMenu Swap");
                    }
                    ChessButton::Roll => {
                        info!("todo GameMenu Roll");
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
