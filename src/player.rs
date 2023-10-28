use crate::component::PieceColor;
use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Record {
    pub serial: usize,
    pub code: String,
    pub value: String,
}

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub color: PieceColor,
    pub records: Vec<Record>,
}

impl Player {
    pub fn new_white() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            records: Vec::new(),
            color: PieceColor::White,
        }
    }

    pub fn new_black() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            color: PieceColor::White,
            records: Vec::new(),
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn push(&mut self, record: Record) {
        self.records.push(record);
    }
}
