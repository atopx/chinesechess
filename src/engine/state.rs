#[derive(PartialEq, Debug)]
pub enum Status {
    HASH = 0,
    KILLER_FIRST = 1,
    KILLER_SECOND = 2,
    GEN_MOVES = 3,
    REST = 4,
}

pub struct MoveState {
    pub mvs: Vec<isize>,
    pub vls: Vec<isize>,
    pub history: Vec<isize>,
    pub index: usize,
    pub hash: isize,
    pub killer_first: isize,
    pub killer_second: isize,
    pub phase: Status,
    pub signle: bool,
}

impl MoveState {
    pub fn new(hisoty: Vec<isize>, hash: isize) -> Self {
        Self {
            mvs: vec![],
            vls: vec![],
            history: hisoty,
            index: 0,
            hash,
            killer_first: 0,
            killer_second: 0,
            phase: Status::HASH,
            signle: false,
        }
    }
}
