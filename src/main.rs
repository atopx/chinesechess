use bevy::app::AppExit;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowMode, WindowResized};

use crate::game::Status;
use crate::public::WIN_SIZE;

mod component;
mod game;
mod plugin;
mod public;
mod system;
mod menu;
mod chessbroad;

fn main() {
    App::new()
        // 初始状态
        .add_state::<Status>()
        // 初始化数据
        .insert_resource(game::Data::new())
        // 初始化系统
        .add_systems(Startup, setup_system)
        // 加载退出游戏系统
        .add_systems(OnEnter(Status::EXIT), exit_system)
        // 进入PENDING状态
        .add_systems(OnEnter(Status::PENDING), menu::setup_pending)
        // PENDING
        .add_systems(Update, menu::pending_state_system.run_if(in_state(Status::PENDING)))
        // 退出PENDING状态
        .add_systems(OnExit(Status::PENDING), menu::cleanup_menu)
        // 进入RUNNING状态
        .add_systems(OnEnter(Status::RUNNING), menu::setup_running)
        // 退出RUNNING状态
        .add_systems(OnExit(Status::RUNNING), menu::cleanup_chessbroad)
        // ESC事件
        .add_systems(Update, menu::esc_event_system)

        // 初始化窗口
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: public::WIN_TITLE.to_string(),
                resolution: (WIN_SIZE.w, WIN_SIZE.h).into(),
                mode: WindowMode::Windowed,
                resizable: false,
                ..default()
            }),
            ..WindowPlugin::default()
        }).set(
            // debug
            LogPlugin {
                level: bevy::log::Level::TRACE,
                ..LogPlugin::default()
            })
        )
        .run()
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    trace!("init system");
    // 创建默认镜头
    commands.spawn(Camera2dBundle::default());

    // 字体
    let fonts = public::asset::Fonts {
        wenkai: asset_server.load(public::path::FONT_WENKAI),
    };
    commands.insert_resource(fonts);

    // 声音
    let sounds = public::asset::Sounds {
        bgm: asset_server.load(public::path::SOUND_BGM),
        eat: asset_server.load(public::path::SOUND_EAT),
        go: asset_server.load(public::path::SOUND_GO),
        invalid: asset_server.load(public::path::SOUND_INVALID),
        select: asset_server.load(public::path::SOUND_SELECT),
        check: asset_server.load(public::path::SOUND_CHECK),
        lose: asset_server.load(public::path::SOUND_LOSE),
        win: asset_server.load(public::path::SOUND_WIN),
        alarm: asset_server.load(public::path::SOUND_ALARM),
    };
    commands.insert_resource(sounds);

    // 图片
    let images = public::asset::Images {
        background: asset_server.load(public::path::IMAGE_BACKGROUND),
        broad: asset_server.load(public::path::IMAGE_BROAD),
        cover: asset_server.load(public::path::IMAGE_COVER),
        popup: asset_server.load(public::path::IMAGE_POPUP),
        select_shadow: asset_server.load(public::path::IMAGE_SELECT_SHADOW),
        start_pos: asset_server.load(public::path::IMAGE_START_POS),
        start_posflag: asset_server.load(public::path::IMAGE_START_POSFLAG),
        play_vs: asset_server.load(public::path::IMAGE_PLAY_VS),
        black_avatar: asset_server.load(public::path::IMAGE_BLACK_AVATAR),
        white_avatar: asset_server.load(public::path::IMAGE_WHITE_AVATAR),
    };

    // 背景
    commands.spawn(SpriteBundle {
        texture: images.background.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: WIN_SIZE.w,
                y: WIN_SIZE.h,
            }),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.insert_resource(images);

    // 动画
    let animates = public::asset::Animates {
        check: vec![
            asset_server.load(public::path::ANIMATE_CHECK_0),
            asset_server.load(public::path::ANIMATE_CHECK_1),
            asset_server.load(public::path::ANIMATE_CHECK_2),
            asset_server.load(public::path::ANIMATE_CHECK_3),
            asset_server.load(public::path::ANIMATE_CHECK_4),
        ],
        checkmate: vec![
            asset_server.load(public::path::ANIMATE_CHECKMATE_0),
            asset_server.load(public::path::ANIMATE_CHECKMATE_1),
            asset_server.load(public::path::ANIMATE_CHECKMATE_2),
            asset_server.load(public::path::ANIMATE_CHECKMATE_3),
            asset_server.load(public::path::ANIMATE_CHECKMATE_4),
        ],
        endposflag: vec![
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_0),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_1),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_2),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_3),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_4),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_5),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_6),
            asset_server.load(public::path::ANIMATE_ENDPOSFLAG_7),
        ],
    };
    commands.insert_resource(animates);

    // 棋子
    let pieces = public::asset::Pieces {
        black_advisor: asset_server.load(public::path::PIECE_BLACK_ADVISOR),
        black_advisor_select: asset_server.load(public::path::PIECE_BLACK_ADVISOR_SELECT),
        black_bishop: asset_server.load(public::path::PIECE_BLACK_BISHOP),
        black_bishop_select: asset_server.load(public::path::PIECE_BLACK_BISHOP_SELECT),
        black_cannon: asset_server.load(public::path::PIECE_BLACK_CANNON),
        black_cannon_select: asset_server.load(public::path::PIECE_BLACK_CANNON_SELECT),
        black_king: asset_server.load(public::path::PIECE_BLACK_KING),
        black_king_select: asset_server.load(public::path::PIECE_BLACK_KING_SELECT),
        black_knight: asset_server.load(public::path::PIECE_BLACK_KNIGHT),
        black_knight_select: asset_server.load(public::path::PIECE_BLACK_KNIGHT_SELECT),
        black_pawn: asset_server.load(public::path::PIECE_BLACK_PAWN),
        black_pawn_select: asset_server.load(public::path::PIECE_BLACK_PAWN_SELECT),
        black_rook: asset_server.load(public::path::PIECE_BLACK_ROOK),
        black_rook_select: asset_server.load(public::path::PIECE_BLACK_ROOK_SELECT),
        white_advisor: asset_server.load(public::path::PIECE_WHITE_ADVISOR),
        white_advisor_select: asset_server.load(public::path::PIECE_WHITE_ADVISOR_SELECT),
        white_bishop: asset_server.load(public::path::PIECE_WHITE_BISHOP),
        white_bishop_select: asset_server.load(public::path::PIECE_WHITE_BISHOP_SELECT),
        white_cannon: asset_server.load(public::path::PIECE_WHITE_CANNON),
        white_cannon_select: asset_server.load(public::path::PIECE_WHITE_CANNON_SELECT),
        white_king: asset_server.load(public::path::PIECE_WHITE_KING),
        white_king_select: asset_server.load(public::path::PIECE_WHITE_KING_SELECT),
        white_knight: asset_server.load(public::path::PIECE_WHITE_KNIGHT),
        white_knight_select: asset_server.load(public::path::PIECE_WHITE_KNIGHT_SELECT),
        white_pawn: asset_server.load(public::path::PIECE_WHITE_PAWN),
        white_pawn_select: asset_server.load(public::path::PIECE_WHITE_PAWN_SELECT),
        white_rook: asset_server.load(public::path::PIECE_WHITE_ROOK),
        white_rook_select: asset_server.load(public::path::PIECE_WHITE_ROOK_SELECT),
    };
    commands.insert_resource(pieces);
}


fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

fn resize_notificator(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        trace!("width = {} height = {}", e.width, e.height);
    }
}

#[cfg(test)]
mod tests {
    use chessai::Engine;

    #[test]
    fn test_engine() {
        let mut engine = Engine::new();
        engine.from_fen("9/2Cca4/3k1C3/4P1p2/4N1b2/4R1r2/4c1n2/3p1n3/2rNK4/9 w");
        let mv = engine.search_main(64, 1000);
        assert_eq!(mv, 26215);
    }
}