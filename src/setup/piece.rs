use bevy::prelude::*;

use crate::public;

pub fn loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("loading asset pieces");
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
