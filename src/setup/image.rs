use bevy::{prelude::*, window::WindowResized};

use crate::public;

#[derive(Component)]
pub struct Background;

pub fn loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("loading asset images");

    // 图片
    let images = public::asset::Images {
        background: asset_server.load(public::path::IMAGE_BACKGROUND),
        broad: asset_server.load(public::path::IMAGE_BROAD),
        cover: asset_server.load(public::path::IMAGE_COVER),
        player_frame: asset_server.load(public::path::IMAGE_PLAYER_FRAME),
        player_focus: asset_server.load(public::path::IMAGE_PLAYER_FOCUS),
        popup: asset_server.load(public::path::IMAGE_POPUP),
        select_shadow: asset_server.load(public::path::IMAGE_SELECT_SHADOW),
        start_pos: asset_server.load(public::path::IMAGE_START_POS),
        play_vs: asset_server.load(public::path::IMAGE_PLAY_VS),
        black_avatar: asset_server.load(public::path::IMAGE_BLACK_AVATAR),
        white_avatar: asset_server.load(public::path::IMAGE_WHITE_AVATAR),
        flag_draw: asset_server.load(public::path::IMAGE_FLAG_DRAW),
        flag_loss: asset_server.load(public::path::IMAGE_FLAG_LOSS),
        flag_win: asset_server.load(public::path::IMAGE_FLAG_WIN),
    };

    // 背景
    commands.spawn((
        SpriteBundle {
            texture: images.background.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: public::WIN_SIZE.w,
                    y: public::WIN_SIZE.h,
                }),
                ..Default::default()
            },
            ..Default::default()
        },
        Background,
    ));

    commands.insert_resource(images);

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

pub fn on_window_resize(
    mut query: Query<&mut Sprite, With<Background>>,
    mut resize_events: EventReader<WindowResized>,
) {
    let mut bg = query.single_mut();
    for e in resize_events.iter() {
        bg.custom_size = Some(Vec2 {
            x: e.width,
            y: e.height,
        })
    }
}
