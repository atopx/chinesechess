use crate::component::{Piece, PieceCate, PieceColor};
use bevy::asset::Handle;
use bevy::prelude::{AudioSource, Font, Image, Resource};

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
    pub play_vs: Handle<Image>,
    pub black_avatar: Handle<Image>,
    pub white_avatar: Handle<Image>,
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

impl Pieces {
    pub fn get_handle(&self, piece: &Piece, selected: bool) -> Option<Handle<Image>> {
        match piece.color {
            PieceColor::White => match piece.cate {
                PieceCate::Rook => {
                    if selected {
                        Some(self.white_rook_select.clone())
                    } else {
                        Some(self.white_rook.clone())
                    }
                }
                PieceCate::Knight => {
                    if selected {
                        Some(self.white_knight_select.clone())
                    } else {
                        Some(self.white_knight.clone())
                    }
                }
                PieceCate::Bishop => {
                    if selected {
                        Some(self.white_bishop_select.clone())
                    } else {
                        Some(self.white_bishop.clone())
                    }
                }
                PieceCate::Advisor => {
                    if selected {
                        Some(self.white_advisor_select.clone())
                    } else {
                        Some(self.white_advisor.clone())
                    }
                }
                PieceCate::Cannon => {
                    if selected {
                        Some(self.white_cannon_select.clone())
                    } else {
                        Some(self.white_cannon.clone())
                    }
                }
                PieceCate::Pawn => {
                    if selected {
                        Some(self.white_pawn_select.clone())
                    } else {
                        Some(self.white_pawn.clone())
                    }
                }
                PieceCate::King => {
                    if selected {
                        Some(self.white_king_select.clone())
                    } else {
                        Some(self.white_king.clone())
                    }
                }
            },
            PieceColor::Black => match piece.cate {
                PieceCate::Rook => {
                    if selected {
                        Some(self.black_rook_select.clone())
                    } else {
                        Some(self.black_rook.clone())
                    }
                }
                PieceCate::Knight => {
                    if selected {
                        Some(self.black_knight_select.clone())
                    } else {
                        Some(self.black_knight.clone())
                    }
                }
                PieceCate::Bishop => {
                    if selected {
                        Some(self.black_bishop_select.clone())
                    } else {
                        Some(self.black_bishop.clone())
                    }
                }
                PieceCate::Advisor => {
                    if selected {
                        Some(self.black_advisor_select.clone())
                    } else {
                        Some(self.black_advisor.clone())
                    }
                }
                PieceCate::Cannon => {
                    if selected {
                        Some(self.black_cannon_select.clone())
                    } else {
                        Some(self.black_cannon.clone())
                    }
                }
                PieceCate::Pawn => {
                    if selected {
                        Some(self.black_pawn_select.clone())
                    } else {
                        Some(self.black_pawn.clone())
                    }
                }
                PieceCate::King => {
                    if selected {
                        Some(self.black_king_select.clone())
                    } else {
                        Some(self.black_king.clone())
                    }
                }
            },
        }
    }
}
