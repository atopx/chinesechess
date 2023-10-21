use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::game::Status;

mod component;
mod game;
mod plugin;
mod public;
mod resource;
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
        // RUNNING
        .add_systems(Update, menu::running_state_system.run_if(in_state(Status::RUNNING)))


        // 初始化窗口
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: public::WIN_TITLE.to_string(),
                resolution: (public::WIN_SIZE).into(),
                mode: WindowMode::Windowed,
                resizable: false,
                ..Window::default()
            }),
            ..WindowPlugin::default()
        }))
        // 初始化游戏

        // 启动 esc 键退出程序
        // .add_systems(Update, bevy::window::close_on_esc)
        .run()
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    info!("init system");
    // 创建默认镜头
    commands.spawn(Camera2dBundle::default());

    // 获取当前窗口
    let window = windows.single_mut();

    let win_size = resource::Size {
        width: window.height(),
        height: window.height(),
    };

    commands.insert_resource(win_size);

    // 字体
    let fonts = resource::asset::Fonts {
        wenkai: asset_server.load(public::assets::FONT_WENKAI),
    };
    commands.insert_resource(fonts);

    // 声音
    let sounds = resource::asset::Sounds {
        bgm: asset_server.load(public::assets::SOUND_BGM),
        eat: asset_server.load(public::assets::SOUND_EAT),
        go: asset_server.load(public::assets::SOUND_GO),
        invalid: asset_server.load(public::assets::SOUND_INVALID),
        select: asset_server.load(public::assets::SOUND_SELECT),
        check: asset_server.load(public::assets::SOUND_CHECK),
        lose: asset_server.load(public::assets::SOUND_LOSE),
        win: asset_server.load(public::assets::SOUND_WIN),
        alarm: asset_server.load(public::assets::SOUND_ALARM),
    };
    commands.insert_resource(sounds);

    // 图片
    let images = resource::asset::Images {
        background: asset_server.load(public::assets::IMAGE_BACKGROUND),
        broad: asset_server.load(public::assets::IMAGE_BROAD),
        cover: asset_server.load(public::assets::IMAGE_COVER),
        popup: asset_server.load(public::assets::IMAGE_POPUP),
        select_shadow: asset_server.load(public::assets::IMAGE_SELECT_SHADOW),
        start_pos: asset_server.load(public::assets::IMAGE_START_POS),
        start_posflag: asset_server.load(public::assets::IMAGE_START_POSFLAG),
    };

    // 背景
    commands.spawn(SpriteBundle {
        texture: images.background.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: public::WIN_SIZE.0,
                y: public::WIN_SIZE.1,
            }),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.insert_resource(images);

    // 动画
    let animates = resource::asset::Animates {
        check: vec![
            asset_server.load(public::assets::ANIMATE_CHECK_0),
            asset_server.load(public::assets::ANIMATE_CHECK_1),
            asset_server.load(public::assets::ANIMATE_CHECK_2),
            asset_server.load(public::assets::ANIMATE_CHECK_3),
            asset_server.load(public::assets::ANIMATE_CHECK_4),
        ],
        checkmate: vec![
            asset_server.load(public::assets::ANIMATE_CHECKMATE_0),
            asset_server.load(public::assets::ANIMATE_CHECKMATE_1),
            asset_server.load(public::assets::ANIMATE_CHECKMATE_2),
            asset_server.load(public::assets::ANIMATE_CHECKMATE_3),
            asset_server.load(public::assets::ANIMATE_CHECKMATE_4),
        ],
        endposflag: vec![
            asset_server.load(public::assets::ANIMATE_ENDPOSFLAG_0),
            asset_server.load(public::assets::ANIMATE_ENDPOSFLAG_1),
            asset_server.load(public::assets::ANIMATE_ENDPOSFLAG_2),
            asset_server.load(public::assets::ANIMATE_ENDPOSFLAG_3),
            asset_server.load(public::assets::ANIMATE_ENDPOSFLAG_4),
            asset_server.load(public::assets::ANIMATE_ENDPOSFLAG_5),
            asset_server.load(public::assets::ANIMATE_ENDPOSFLAG_6),
            asset_server.load(public::assets::ANIMATE_ENDPOSFLAG_7),
        ],
    };
    commands.insert_resource(animates);

    // 棋子
    let pieces = resource::asset::Pieces {
        black_advisor: asset_server.load(public::assets::PIECE_BLACK_ADVISOR),
        black_advisor_select: asset_server.load(public::assets::PIECE_BLACK_ADVISOR_SELECT),
        black_bishop: asset_server.load(public::assets::PIECE_BLACK_BISHOP),
        black_bishop_select: asset_server.load(public::assets::PIECE_BLACK_BISHOP_SELECT),
        black_cannon: asset_server.load(public::assets::PIECE_BLACK_CANNON),
        black_cannon_select: asset_server.load(public::assets::PIECE_BLACK_CANNON_SELECT),
        black_king: asset_server.load(public::assets::PIECE_BLACK_KING),
        black_king_select: asset_server.load(public::assets::PIECE_BLACK_KING_SELECT),
        black_knight: asset_server.load(public::assets::PIECE_BLACK_KNIGHT),
        black_knight_select: asset_server.load(public::assets::PIECE_BLACK_KNIGHT_SELECT),
        black_pawn: asset_server.load(public::assets::PIECE_BLACK_PAWN),
        black_pawn_select: asset_server.load(public::assets::PIECE_BLACK_PAWN_SELECT),
        black_rook: asset_server.load(public::assets::PIECE_BLACK_ROOK),
        black_rook_select: asset_server.load(public::assets::PIECE_BLACK_ROOK_SELECT),
        white_advisor: asset_server.load(public::assets::PIECE_WHITE_ADVISOR),
        white_advisor_select: asset_server.load(public::assets::PIECE_WHITE_ADVISOR_SELECT),
        white_bishop: asset_server.load(public::assets::PIECE_WHITE_BISHOP),
        white_bishop_select: asset_server.load(public::assets::PIECE_WHITE_BISHOP_SELECT),
        white_cannon: asset_server.load(public::assets::PIECE_WHITE_CANNON),
        white_cannon_select: asset_server.load(public::assets::PIECE_WHITE_CANNON_SELECT),
        white_king: asset_server.load(public::assets::PIECE_WHITE_KING),
        white_king_select: asset_server.load(public::assets::PIECE_WHITE_KING_SELECT),
        white_knight: asset_server.load(public::assets::PIECE_WHITE_KNIGHT),
        white_knight_select: asset_server.load(public::assets::PIECE_WHITE_KNIGHT_SELECT),
        white_pawn: asset_server.load(public::assets::PIECE_WHITE_PAWN),
        white_pawn_select: asset_server.load(public::assets::PIECE_WHITE_PAWN_SELECT),
        white_rook: asset_server.load(public::assets::PIECE_WHITE_ROOK),
        white_rook_select: asset_server.load(public::assets::PIECE_WHITE_ROOK_SELECT),
    };
    commands.insert_resource(pieces);
}


fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}