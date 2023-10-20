use bevy::prelude::*;
use bevy::window::WindowMode;

use crate::public::{IMAGE_CHECKMATE_POINT, IMAGE_CHECKMATE_SIZE};

mod component;
mod game;
mod plugin;
mod public;
mod resource;
mod system;

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut data: ResMut<game::Data>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: Query<&mut Window>,
) {
    // 创建默认镜头
    commands.spawn(Camera2dBundle::default());

    // 获取当前窗口
    let window = windows.single_mut();

    let win_size = resource::Size {
        width: window.height(),
        height: window.height(),
    };

    commands.insert_resource(win_size);

    // 加载绝杀动画
    let texture_handle = asset_server.load(public::assets::IMAGE_CHECKMATE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::from(IMAGE_CHECKMATE_SIZE),
        IMAGE_CHECKMATE_POINT.0,
        IMAGE_CHECKMATE_POINT.1,
        None,
        None,
    );

    let checkmate: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    // 字体
    let fonts = resource::asset::Fonts {
        wenkai: asset_server.load(public::assets::FONT_WENKAI),
    };
    commands.insert_resource(fonts);

    // 图片资源
    let images = resource::asset::Images {
        background: asset_server.load(public::assets::IMAGE_BACKGROUND),
        broad: asset_server.load(public::assets::IMAGE_BROAD),
        broad_w: asset_server.load(public::assets::IMAGE_BROAD_W),
        broad_b: asset_server.load(public::assets::IMAGE_BROAD_B),
        cover: asset_server.load(public::assets::IMAGE_COVER),
        piece: asset_server.load(public::assets::IMAGE_PIECE),
        avatar: asset_server.load(public::assets::IMAGE_AVATAR),
        piece_rook_b: asset_server.load(public::assets::PIECE_ROOK_B),
        piece_knight_b: asset_server.load(public::assets::PIECE_KNIGHT_B),
        piece_bishop_b: asset_server.load(public::assets::PIECE_BISHOP_B),
        piece_advisor_b: asset_server.load(public::assets::PIECE_ADVISOR_B),
        piece_cannon_b: asset_server.load(public::assets::PIECE_CANNON_B),
        piece_pawn_b: asset_server.load(public::assets::PIECE_PAWN_B),
        piece_king_b: asset_server.load(public::assets::PIECE_KING_B),
        piece_rook_w: asset_server.load(public::assets::PIECE_ROOK_W),
        piece_knight_w: asset_server.load(public::assets::PIECE_KNIGHT_W),
        piece_bishop_w: asset_server.load(public::assets::PIECE_BISHOP_W),
        piece_advisor_w: asset_server.load(public::assets::PIECE_ADVISOR_W),
        piece_cannon_w: asset_server.load(public::assets::PIECE_CANNON_W),
        piece_pawn_w: asset_server.load(public::assets::PIECE_PAWN_W),
        piece_king_w: asset_server.load(public::assets::PIECE_KING_W),
        checkmate,
    };

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

    // 棋盘
    commands.spawn(SpriteBundle {
        texture: images.broad_w.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 767., y: 842. }),
            ..Default::default()
        },
        ..Default::default()
    });
    let t = data.parse_route("a1e4".to_string());
    println!("{:?}", t);
    println!("{}", data.white_player.name);
}

fn main() {
    App::new()
        // 游戏状态
        .add_state::<game::Status>()
        .insert_resource(game::Data::new())
        // 背景色
        .insert_resource(ClearColor(Color::rgb(255., 255., 255.)))
        // 窗口
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
        .add_systems(Startup, setup_system)
        // 启动 esc 键退出程序
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
