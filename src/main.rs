use bevy::app::AppExit;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowMode};
use bevy::winit::WinitWindows;
use game::Status;
use std::io::Cursor;
use winit::window::Icon;

// use crate::chessbroad::Status;
use public::WIN_SIZE;
mod chess;
mod component;
mod game;
mod menu;
mod player;
mod public;
mod setup;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        // 初始状态
        .add_state::<Status>()
        // 插件系统
        .add_plugins((
            // 初始化系统资源
            setup::AssetLoading,
            // 下棋插件
            chess::ChessPlugin,
        ))
        // 初始化数据
        .insert_resource(game::Data::new())
        .insert_resource(game::BroadEntitys::default())
        // 窗口图标
        .add_systems(Startup, set_window_icon)
        // 加载退出游戏系统
        .add_systems(OnEnter(Status::EXIT), exit_system)
        // 进入PENDING状态
        .add_systems(OnEnter(Status::PENDING), menu::setup_pending)
        // 全局菜单系统
        .add_systems(Update, menu::pending_state_system.run_if(in_state(Status::PENDING)))
        // 退出PENDING状态
        .add_systems(OnExit(Status::PENDING), menu::cleanup_menu)
        // ESC事件
        .add_systems(Update, game::esc_event_system)
        // 初始化窗口
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: public::WIN_TITLE.to_string(),
                        resolution: (WIN_SIZE.w, WIN_SIZE.h).into(),
                        canvas: Some("#bevy".to_owned()),
                        mode: WindowMode::Windowed,
                        prevent_default_event_handling: false,
                        resizable: false,
                        ..default()
                    }),
                    ..WindowPlugin::default()
                })
                .set(
                    // debug
                    LogPlugin {
                        level: bevy::log::Level::TRACE,
                        ..LogPlugin::default()
                    },
                ),
        )
        .run()
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/image/logo.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

#[cfg(test)]
mod tests {
    use chessai::Engine;

    #[test]
    fn test_engine() {
        let mut engine = Engine::new();
        engine.from_fen("rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RNBAKABNR b");
        let mv = engine.search_main(64, 1000);
        assert_eq!(mv, 26215);
    }
}
