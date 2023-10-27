use bevy::prelude::Component;
use crate::component::PieceColor;

#[derive(Component, Clone, Debug)]
pub struct Record {
    pub serial: usize,
    pub code: String,
    pub value: String,
}

/// PlayerState 玩家状态
#[derive(Component, Clone, Copy, Debug, Default)]
pub enum PlayerState {
    // 空闲
    #[default]
    None,
    // 思考中
    Thinking,
    // 已选棋
    Selected,
}

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub color: PieceColor,
    pub records: Vec<String>,
}
