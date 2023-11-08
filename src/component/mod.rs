use bevy::prelude::{Component, Entity};

use crate::public::Pos;
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

#[derive(Component)]
pub struct ChineseBroadCamera;

// 玩家信息框
#[derive(Component, Debug, Default, Clone)]
pub struct PlayerInfo;

// 游戏对局按钮组件
#[derive(Component)]
pub struct ChessButtonGroup;

// 选择棋子动画
#[derive(Component)]
pub struct PieceSelected(Pos);

#[derive(Component)]
pub struct PieceCanceled(Pos);

#[derive(Component)]
pub struct PieceMoved(Pos, Pos);
