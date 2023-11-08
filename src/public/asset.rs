use bevy::asset::Handle;
use bevy::prelude::{AudioSource, Font, Image, Resource, TextureAtlas};

use crate::component::piece::{Kind, Piece, Side};

#[derive(Resource)]
pub struct Fonts {
    pub wenkai: Handle<Font>,
    pub xiaoli: Handle<Font>,
}

#[derive(Resource)]
pub struct Sounds {
    pub bgm: Handle<AudioSource>,
    pub eat: Handle<AudioSource>,
    pub go: Handle<AudioSource>,
    pub invalid: Handle<AudioSource>,
    pub select: Handle<AudioSource>,
    pub check: Handle<AudioSource>,
    pub loss: Handle<AudioSource>,
    pub win: Handle<AudioSource>,
    pub draw: Handle<AudioSource>,
    pub alarm: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct Images {
    pub background: Handle<Image>,
    pub broad: Handle<Image>,
    pub cover: Handle<Image>,
    pub popup: Handle<Image>,
    pub player_frame: Handle<Image>,
    pub player_focus: Handle<Image>,
    pub select_shadow: Handle<Image>,
    pub start_pos: Handle<Image>,
    pub play_vs: Handle<Image>,
    pub black_avatar: Handle<Image>,
    pub white_avatar: Handle<Image>,
    pub flag_draw: Handle<Image>,
    pub flag_loss: Handle<Image>,
    pub flag_win: Handle<Image>,
}

#[derive(Resource)]
pub struct Animates {
    pub endposflag: Handle<TextureAtlas>,
    pub check: Vec<Handle<Image>>,
    pub checkmate: Vec<Handle<Image>>,
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

impl Pieces {
    pub fn get_handle(&self, piece: &Piece, selected: bool) -> Handle<Image> {
        match piece.side {
            Side::White => match piece.kind {
                Kind::Rook => {
                    if selected {
                        self.white_rook_select.clone()
                    } else {
                        self.white_rook.clone()
                    }
                }
                Kind::Knight => {
                    if selected {
                        self.white_knight_select.clone()
                    } else {
                        self.white_knight.clone()
                    }
                }
                Kind::Bishop => {
                    if selected {
                        self.white_bishop_select.clone()
                    } else {
                        self.white_bishop.clone()
                    }
                }
                Kind::Advisor => {
                    if selected {
                        self.white_advisor_select.clone()
                    } else {
                        self.white_advisor.clone()
                    }
                }
                Kind::Cannon => {
                    if selected {
                        self.white_cannon_select.clone()
                    } else {
                        self.white_cannon.clone()
                    }
                }
                Kind::Pawn => {
                    if selected {
                        self.white_pawn_select.clone()
                    } else {
                        self.white_pawn.clone()
                    }
                }
                Kind::King => {
                    if selected {
                        self.white_king_select.clone()
                    } else {
                        self.white_king.clone()
                    }
                }
            },
            Side::Black => match piece.kind {
                Kind::Rook => {
                    if selected {
                        self.black_rook_select.clone()
                    } else {
                        self.black_rook.clone()
                    }
                }
                Kind::Knight => {
                    if selected {
                        self.black_knight_select.clone()
                    } else {
                        self.black_knight.clone()
                    }
                }
                Kind::Bishop => {
                    if selected {
                        self.black_bishop_select.clone()
                    } else {
                        self.black_bishop.clone()
                    }
                }
                Kind::Advisor => {
                    if selected {
                        self.black_advisor_select.clone()
                    } else {
                        self.black_advisor.clone()
                    }
                }
                Kind::Cannon => {
                    if selected {
                        self.black_cannon_select.clone()
                    } else {
                        self.black_cannon.clone()
                    }
                }
                Kind::Pawn => {
                    if selected {
                        self.black_pawn_select.clone()
                    } else {
                        self.black_pawn.clone()
                    }
                }
                Kind::King => {
                    if selected {
                        self.black_king_select.clone()
                    } else {
                        self.black_king.clone()
                    }
                }
            },
        }
    }
}
