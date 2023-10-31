use bevy::{
    prelude::{Component, Entity, Vec2, With},
    text::TextSection,
};
pub mod piece;

#[derive(Component, Debug, Clone)]
pub struct Broad;

#[derive(Component, Debug, Clone)]
pub struct Background(Entity);


#[derive(Component, Debug, Default)]
pub struct ActivePiece;

#[derive(Component, Debug, Default)]
pub struct SourcePiece;

#[derive(Component, Debug, Default)]
pub struct SelectedPiece;

type WithPiece = (With<ActivePiece>, With<SourcePiece>, With<SelectedPiece>);
