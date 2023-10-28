use bevy::prelude::*;

use crate::public;

pub fn loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("loading asset images");

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
                x: public::WIN_SIZE.w,
                y: public::WIN_SIZE.h,
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
