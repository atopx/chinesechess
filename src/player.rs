use crate::component::PieceColor;
use bevy::{prelude::*, time::Stopwatch};

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
    pub total_timer: Stopwatch,
    pub current_timer: Stopwatch,
}

impl Player {
    pub fn new_white() -> Self {
        let mut total_timer = Stopwatch::new();
        total_timer.pause();
        total_timer.reset();
        let mut current_timer = Stopwatch::new();
        current_timer.pause();
        current_timer.reset();
        Self {
            id: String::new(),
            name: String::new(),
            records: Vec::new(),
            color: PieceColor::White,
            total_timer,
            current_timer,
        }
    }

    pub fn new_black() -> Self {
        let mut total_timer = Stopwatch::new();
        total_timer.pause();
        total_timer.reset();
        let mut current_timer = Stopwatch::new();
        current_timer.pause();
        current_timer.reset();
        Self {
            id: String::new(),
            name: String::new(),
            color: PieceColor::Black,
            records: Vec::new(),
            total_timer,
            current_timer,
        }
    }

    pub fn get_action(&self) -> String {
        if self.records.len() > 0 {
            self.records.last().unwrap().value.clone()
        } else if self.color == PieceColor::White {
            "思考中".to_string()
        } else {
            "空闲中".to_string()
        }
    }

    pub fn start_timer(&mut self) {
        self.total_timer.unpause();
        self.current_timer.reset();
        self.current_timer.unpause();
    }

    pub fn stop_timer(&mut self) {
        self.total_timer.pause();
        self.current_timer.pause();
    }

    pub fn get_total_timer(&self) -> String {
        let secs = self.total_timer.elapsed_secs();
        let minutes = (secs / 60.0).floor() as u32;
        let seconds = (secs % 60.0).round() as u32;
        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn get_current_timer(&self) -> String {
        let secs = self.current_timer.elapsed_secs();
        let minutes = (secs / 60.0).floor() as u32;
        let seconds = (secs % 60.0).round() as u32;
        format!("{:02}:{:02}", minutes, seconds)
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

#[derive(Component)]
pub struct PlayerFocus;
