use super::{pregen, util};

pub struct Engine {
    sd_player: usize,
    zobrist_key: isize,
    zobrist_lock: isize,
    vl_white: isize,
    vl_black: isize,
    distance: isize,
    mv_list: Vec<isize>,
    pc_list: Vec<isize>,
    key_list: Vec<isize>,
    chk_list: Vec<bool>,
    squares: [isize; 256],
}

impl Engine {
    pub fn new() -> Self {
        Self {
            sd_player: 0,
            zobrist_key: 0,
            zobrist_lock: 0,
            vl_white: 0,
            vl_black: 0,
            distance: 0,
            mv_list: vec![],
            pc_list: vec![],
            key_list: vec![],
            chk_list: vec![],
            squares: [0; 256],
        }
    }

    pub fn clearborad(&mut self) {
        self.sd_player = 0;
        self.zobrist_key = 0;
        self.zobrist_lock = 0;
        self.vl_black = 0;
        self.vl_white = 0;
        self.squares = [0; 256];
    }

    pub fn mate_valie(&self) -> isize {
        self.distance - pregen::MATE_VALUE
    }

    pub fn ban_value(&self) -> isize {
        self.distance - pregen::BAN_VALUE
    }

    pub fn draw_value(&self) -> isize {
        match self.distance & 1 {
            0 => -pregen::DRAW_VALUE,
            _ => pregen::DRAW_VALUE,
        }
    }

    pub fn evalute(&self) -> isize {
        let mut vl = match self.sd_player {
            0 => self.vl_white - self.vl_black,
            _ => self.vl_black - self.vl_white,
        };
        vl += pregen::ADVANCED_VALUE;
        match self.draw_value() {
            vl => vl - 1,
            _ => vl,
        }
    }

    pub fn null_okey(&self) -> bool {
        let mut vl = match self.sd_player {
            0 => self.vl_white,
            _ => self.vl_black,
        };
        vl > pregen::NULL_OKAY_MARGIN
    }

    pub fn null_safe(&self) -> bool {
        let mut vl = match self.sd_player {
            0 => self.vl_white,
            _ => self.vl_black,
        };
        vl > pregen::NULL_OKAY_MARGIN
    }

    pub fn null_move(&mut self) {
        self.mv_list.push(0);
        self.pc_list.push(0);
        self.key_list.push(self.zobrist_key);
        self.change_side();
        self.chk_list.push(false);
        self.distance += 1
    }

    pub fn undo_null_move(&mut self) {
        self.distance -= 1;
        self.chk_list.pop();
        self.change_side();
        self.key_list.pop();
        self.pc_list.pop();
        self.mv_list.pop();
    }

    pub fn is_check(&self) -> bool {
        *self.chk_list.last().unwrap()
    }

    pub fn captured(&self) -> bool {
        *self.pc_list.last().unwrap() > 0
    }

    pub fn rep_value(&self, vl_rep: isize) -> isize {
        let mut vl: isize = 0;
        if vl_rep & 2 != 0 {
            vl = self.ban_value();
        };
        if vl_rep & 4 != 0 {
            vl -= self.ban_value();
        };
        match vl {
            0 => self.draw_value(),
            _ => vl,
        }
    }

    pub fn rep_status(&self, mut recur: usize) -> usize {
        let mut status = 0;
        let mut side = false;
        let mut perp_check = true;
        let mut opp_perp_check = true;
        let mut index = self.mv_list.len() - 1;
        while self.mv_list[index] > 0 && self.pc_list[index] == 0 {
            if side {
                perp_check = perp_check && self.chk_list[index];
                if self.key_list[index] == self.zobrist_key {
                    recur -= 1;
                    if recur == 0 {
                        if perp_check {
                            status += 2;
                        }
                        if opp_perp_check {
                            status += 4;
                        }
                        return status + 1;
                    }
                }
            } else {
                opp_perp_check = opp_perp_check && self.chk_list[index];
            }
            side = !side;
            index -= 1;
        }
        status
    }

    pub fn change_side(&mut self) {
        self.sd_player = 1 - self.sd_player;
        self.zobrist_key ^= pregen::PRE_GEN_ZOB_RIST_KEY_PLAYER;
        self.zobrist_lock ^= pregen::PRE_GEN_ZOB_RIST_LOCK_PLAYER;
    }

    pub fn history_index(&self, mv: isize) -> isize {
        (self.squares[util::src(mv) as usize] - 8) << 8 + util::dst(mv)
    }

    pub fn checked(&self) -> bool {
        let self_side = util::side_tag(self.sd_player as isize);
        let opp_side = util::opp_side_tag(self.sd_player as isize);
        let mut sq_src: usize = 0;

        for sql_src in 0..256 as usize {
            if self.squares[sq_src] != (self_side + opp_side) {
                continue;
            }
            
            let side_pawn = opp_side + pregen::PIECE_TYPE::PAWN as isize;
            
            if self.squares[util::square_forward(sq_src, self.sd_player as usize)] == side_pawn {
                return true;
            }

            if self.squares[sq_src - 1] == side_pawn || self.squares[sq_src + 1] == side_pawn {
                return true;
            }

            for i in 0..4 as usize {
                if self.squares[sq_src.saturating_add_signed(pregen::ADVISOR_DELTA[i])] != 0 {
                    continue;
                };
                let side_knght = opp_side + pregen::PIECE_TYPE::KNIGHT.int();
                for n in 0..2 as usize {
                    if self.squares[sq_src.saturating_add_signed(pregen::KNIGHT_CHECK_DELTA[i][n])] == side_knght {
                        return true;
                    }
                }
            }

            for i in 0..4 as usize {
                let delta = pregen::KING_DELTA[i] as usize;
                let mut sq_dst = sq_src+delta;
                while pregen::IN_BROAD(sq_dst) {
                    let pc_dst = self.squares[sq_dst];
                    if pc_dst > 0 {
                        if pc_dst == opp_side+pregen::PIECE_TYPE::ROOK.int() || pc_dst == opp_side+pregen::PIECE_TYPE::KING.int() {
                            return true;
                        }
                        break;
                    }
                    sq_dst+=delta;
                }
                sq_dst+= delta;
                while pregen::IN_BROAD(sq_dst) {
                    let pc_dst = self.squares[sq_dst];
                    if pc_dst > 0 {
                        if pc_dst == opp_side + pregen::PIECE_TYPE::CANNON.int() {
                            return true;
                        }
                        break;
                    }
                    sq_dst+= delta;
                }
            }
            return false;
        }
        return false;
    }
}
