use bevy::asset::Handle;
use bevy::prelude::{AudioSource, Font, Image, Resource, TextureAtlas};

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
    pub broad_w: Handle<Image>,
    pub broad_b: Handle<Image>,
    pub cover: Handle<Image>,
    pub checkmate: Handle<TextureAtlas>,
    pub piece: Handle<Image>,
    pub avatar: Handle<Image>,

    pub piece_rook_b: Handle<Image>,
    pub piece_knight_b: Handle<Image>,
    pub piece_bishop_b: Handle<Image>,
    pub piece_advisor_b: Handle<Image>,
    pub piece_cannon_b: Handle<Image>,
    pub piece_pawn_b: Handle<Image>,
    pub piece_king_b: Handle<Image>,

    pub piece_rook_w: Handle<Image>,
    pub piece_knight_w: Handle<Image>,
    pub piece_bishop_w: Handle<Image>,
    pub piece_advisor_w: Handle<Image>,
    pub piece_cannon_w: Handle<Image>,
    pub piece_pawn_w: Handle<Image>,
    pub piece_king_w: Handle<Image>,
}
