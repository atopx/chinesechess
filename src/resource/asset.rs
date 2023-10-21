use bevy::asset::Handle;
use bevy::prelude::{AudioSource, Font, Image, Resource};

#[derive(Resource)]
pub struct Fonts {
    pub wenkai: Handle<Font>,
}

#[derive(Resource)]
pub struct Sounds {
    pub bgm: Handle<AudioSource>,
    pub eat: Handle<AudioSource>,
    pub go: Handle<AudioSource>,
    pub invalid: Handle<AudioSource>,
    pub select: Handle<AudioSource>,
    pub check: Handle<AudioSource>,
    pub lose: Handle<AudioSource>,
    pub win: Handle<AudioSource>,
    pub alarm: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct Images {
    pub background: Handle<Image>,
    pub broad: Handle<Image>,
    pub cover: Handle<Image>,
    pub popup: Handle<Image>,
    pub select_shadow: Handle<Image>,
    pub start_pos: Handle<Image>,
    pub start_posflag: Handle<Image>,
}

#[derive(Resource)]
pub struct Animates {
    pub check: Vec<Handle<Image>>,
    pub checkmate: Vec<Handle<Image>>,
    pub endposflag: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct Pieces {
    pub black_advisor: Handle<Image>,
    pub black_advisor_select: Handle<Image>,
    pub black_bishop: Handle<Image>,
    pub black_bishop_select: Handle<Image>,
    pub black_cannon: Handle<Image>,
    pub black_cannon_select: Handle<Image>,
    pub black_king: Handle<Image>,
    pub black_king_select: Handle<Image>,
    pub black_knight: Handle<Image>,
    pub black_knight_select: Handle<Image>,
    pub black_pawn: Handle<Image>,
    pub black_pawn_select: Handle<Image>,
    pub black_rook: Handle<Image>,
    pub black_rook_select: Handle<Image>,
    pub white_advisor: Handle<Image>,
    pub white_advisor_select: Handle<Image>,
    pub white_bishop: Handle<Image>,
    pub white_bishop_select: Handle<Image>,
    pub white_cannon: Handle<Image>,
    pub white_cannon_select: Handle<Image>,
    pub white_king: Handle<Image>,
    pub white_king_select: Handle<Image>,
    pub white_knight: Handle<Image>,
    pub white_knight_select: Handle<Image>,
    pub white_pawn: Handle<Image>,
    pub white_pawn_select: Handle<Image>,
    pub white_rook: Handle<Image>,
    pub white_rook_select: Handle<Image>,
}