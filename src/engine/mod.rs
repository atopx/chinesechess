use std::time::{Duration, Instant};

use self::state::{MoveState, Status};

mod book;
mod pregen;
mod state;
mod util;

#[derive(Clone, Copy, Default)]
struct Hash {
    depth: isize,
    flag: isize,
    vl: isize,
    mv: isize,
    zobrist_lock: isize,
}

pub struct ChessAi {
    sd_player: isize,
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
    mask: isize,
    hash_table: Vec<Hash>,
    history: Vec<isize>,
    killer_table: Vec<[isize; 2]>,
    result: isize,
    all_nodes: isize,
}

impl ChessAi {
    pub fn new() -> Self {
        ChessAi {
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
            mask: 65535,
            hash_table: vec![],
            history: vec![],
            killer_table: vec![],
            result: 0,
            all_nodes: 0,
        }
    }

    pub fn from_fen(&mut self, fen: &str) {
        self.clearboard();
        let mut x = pregen::FILE_LEFT;
        let mut y = pregen::RANK_TOP;
        let mut index = 0;

        if fen.len() == index {
            self.set_irrev();
            return;
        }

        let mut chars = fen.chars();
        let mut c = chars.next().unwrap();
        while c != ' ' {
            if c == '/' {
                x = pregen::FILE_LEFT;
                y += 1;
                if y > pregen::RANK_BOTTOM {
                    break;
                }
            } else if c >= '1' && c <= '9' {
                x += (c as u8 - b'0') as isize;
            } else if c >= 'A' && c <= 'Z' {
                if x <= pregen::FILE_RIGHT {
                    if let Some(pt) = pregen::from_char(c) {
                        self.add_piece(util::coord_xy(x, y), pt + 8, pregen::PieceAction::ADD);
                    };
                    x += 1;
                }
            } else if c >= 'a' && c <= 'z' {
                if x <= pregen::FILE_RIGHT {
                    if let Some(pt) = pregen::from_char((c as u8 + b'A' - b'a') as char) {
                        self.add_piece(util::coord_xy(x, y), pt + 16, pregen::PieceAction::ADD);
                    }
                    x += 1;
                }
            }
            index += 1;
            if index == fen.len() {
                self.set_irrev();
                return;
            }
            c = chars.next().unwrap();
        }
        index += 1;
        if index == fen.len() {
            self.set_irrev();
            return;
        }
        let player = if fen.chars().nth(index).unwrap() == 'b' {
            0
        } else {
            1
        };
        if self.sd_player == player {
            self.change_side();
        }
        self.set_irrev();
    }

    pub fn to_fen(&self) -> String {
        let mut chars: Vec<String> = Vec::new();
        for y in pregen::RANK_TOP..pregen::RANK_BOTTOM + 1 {
            let mut k = 0;
            let mut row = String::new();
            for x in pregen::FILE_LEFT..pregen::FILE_RIGHT + 1 {
                let pc = self.squares[util::coord_xy(x, y) as usize];
                if pc > 0 {
                    if k > 0 {
                        row.push((k as u8 + b'0') as char);
                        k = 0;
                    }
                    row.push(pregen::FEN_PIECE[pc as usize]);
                } else {
                    k += 1;
                }
            }
            if k > 0 {
                row.push((k as u8 + b'0') as char);
            }
            chars.push(row);
        }
        let mut fen = chars.join("/");
        if self.sd_player == 0 {
            fen.push_str(" w");
        } else {
            fen.push_str(" b");
        }
        fen
    }

    pub fn clearboard(&mut self) {
        self.sd_player = 0;
        self.zobrist_key = 0;
        self.zobrist_lock = 0;
        self.vl_black = 0;
        self.vl_white = 0;
        self.squares = [0; 256];
    }

    pub fn set_irrev(&mut self) {
        self.distance = 0;
        self.mv_list = vec![0];
        self.pc_list = vec![0];
        self.key_list = vec![0];
        self.chk_list = vec![self.checked()];
    }

    pub fn mate_value(&self) -> isize {
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

    pub fn evaluate(&self) -> isize {
        let mut vl = match self.sd_player {
            0 => self.vl_white - self.vl_black,
            _ => self.vl_black - self.vl_white,
        };
        vl += pregen::ADVANCED_VALUE;
        if vl == self.draw_value() {
            vl - 1
        } else {
            vl
        }
    }

    pub fn null_okay(&self) -> bool {
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
        self.chk_list.pop().unwrap();
        self.change_side();
        self.key_list.pop().unwrap();
        self.pc_list.pop().unwrap();
        self.mv_list.pop().unwrap();
    }

    pub fn in_check(&self) -> bool {
        *self.chk_list.last().unwrap()
    }

    pub fn captured(&self) -> bool {
        *self.pc_list.last().unwrap() > 0
    }

    pub fn rep_value(&self, vl_rep: isize) -> isize {
        let mut vl: isize = 0;
        if vl_rep & 2 != 0 {
            vl = self.ban_value() as isize;
        };
        if vl_rep & 4 != 0 {
            vl -= self.ban_value() as isize;
        };
        match vl {
            0 => self.draw_value(),
            _ => vl,
        }
    }

    pub fn rep_status(&self, mut recur: isize) -> isize {
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
        ((self.squares[util::src(mv) as usize] - 8) << 8) + util::dst(mv)
    }

    pub fn book_move(&self) -> isize {
        let mut mirror_opt: bool = false;
        let mut lock = util::unsigned_right_shift(self.zobrist_lock, 1);
        let mut index_opt = book::Book::get().search(lock);
        let book = book::Book::get();
        if index_opt.is_none() {
            mirror_opt = true;
            lock = util::unsigned_right_shift(self.mirror().zobrist_lock, 1);
            index_opt = book.search(lock);
            if index_opt.is_none() {
                return 0;
            }
        };
        let mut index = index_opt.unwrap() - 1;
        while index >= 0 && book.data[index][0] == lock {
            index -= 1;
        }
        let mut mvs = vec![];
        let mut vls = vec![];
        let mut value = 0;
        let book = book::Book::get();
        while index < book.data.len() && book.data[index][0] == lock {
            let mv = book.data[index][1];
            if mirror_opt {
                let mv = util::mirror_move(mv);
            }
            if self.legal_move(mv) {
                mvs.push(mv);
                let vl = book.data[index][2];
                vls.push(vl);
                value += vl;
            }
            index += 1;
        }
        if value == 0 {
            return 0;
        };
        value = util::randf64(value) as isize;
        for index in 0..mvs.len() {
            value -= vls[index];
            if value < 0 {
                break;
            }
        }
        return mvs[index];
    }

    pub fn legal_move(&self, mv: isize) -> bool {
        let sq_src = util::src(mv) as isize;
        let pc_src = self.squares[sq_src as usize];

        let self_side = util::side_tag(self.sd_player as isize) as isize;
        if pc_src & self_side == 0 {
            return false;
        }
        let sq_dst = util::dst(mv) as isize;
        let pc_dst = self.squares[sq_dst as usize];
        if pc_dst & self_side != 0 {
            return false;
        }

        match pc_src - self_side {
            pregen::PIECE_KING => pregen::IN_FORT(sq_dst) && pregen::KING_SPAN(sq_src, sq_dst),
            pregen::PIECE_ADVISOR => {
                pregen::IN_FORT(sq_dst) && pregen::ADVISOR_SPAN(sq_src, sq_dst)
            }
            pregen::PIECE_BISHOP => {
                pregen::SAME_HALF(sq_src, sq_dst)
                    && pregen::BISHOP_SPAN(sq_src, sq_dst)
                    && self.squares[pregen::BISHOP_PIN(sq_src, sq_dst)] == 0
            }
            pregen::PIECE_KNIGHT => {
                let pin = pregen::KNIGHT_PIN(sq_src, sq_dst);
                pin != sq_src && self.squares[pin as usize] == 0
            }
            pregen::PIECE_PAWN => {
                if pregen::AWAY_HALF(sq_dst, self.sd_player)
                    && (sq_dst == sq_src - 1 || sq_dst == sq_src + 1)
                {
                    true
                } else {
                    sq_dst == util::square_forward(sq_src, self.sd_player)
                }
            }
            pregen::PIECE_ROOK | pregen::PIECE_CANNON => {
                let delta = if pregen::SAME_RANK(sq_src, sq_dst) {
                    if sq_src > sq_dst {
                        -1
                    } else {
                        1
                    }
                } else if pregen::SAME_FILE(sq_src, sq_dst) {
                    if sq_src > sq_dst {
                        -16
                    } else {
                        16
                    }
                } else {
                    return false;
                };

                let mut pin = sq_src + delta;

                while pin != sq_dst && self.squares[pin as usize] == 0 {
                    pin = pin + delta;
                }

                if pin == sq_dst {
                    return pc_dst == 0 || pc_src - self_side == pregen::PIECE_ROOK;
                }

                if pc_dst == 0 || pc_src - self_side != pregen::PIECE_CANNON {
                    return false;
                }
                pin = pin + delta;
                while pin != sq_dst && self.squares[pin as usize] == 0 {
                    pin = pin + delta;
                }
                return pin == sq_dst;
            }
            _ => false,
        }
    }

    pub fn mirror(&self) -> Self {
        let mut mirror = Self::new();
        mirror.clearboard();
        for i in 0..mirror.squares.len() {
            let pc = self.squares[i];
            if pc > 0 {
                mirror.add_piece(
                    util::mirror_square(i as isize) as isize,
                    pc,
                    pregen::PieceAction::ADD,
                )
            }
        }

        if self.sd_player == 1 {
            mirror.change_side();
        }
        mirror
    }

    pub fn move_piece(&mut self, mv: isize) {
        let sq_src = util::src(mv);
        let sq_dst = util::dst(mv);
        let pc_dst = self.squares[sq_dst as usize];
        self.pc_list.push(pc_dst);
        if pc_dst > 0 {
            self.add_piece(sq_dst, pc_dst, pregen::PieceAction::DEL);
        }
        let pc_src = self.squares[sq_src as usize];

        self.add_piece(sq_src, pc_src, pregen::PieceAction::DEL);
        self.add_piece(sq_dst, pc_src, pregen::PieceAction::ADD);
        self.mv_list.push(mv);
    }

    pub fn make_move(&mut self, mv: isize) -> bool {
        self.move_piece(mv);

        if self.checked() {
            self.undo_move_piece();
            return false;
        }

        self.key_list.push(self.zobrist_key);
        self.change_side();
        self.chk_list.push(self.checked());
        self.distance += 1;

        return true;
    }

    pub fn undo_make_move(&mut self) {
        self.distance -= 1;
        self.chk_list.pop().unwrap();
        self.change_side();
        self.key_list.pop().unwrap();
        self.undo_move_piece();
    }

    pub fn undo_move_piece(&mut self) {
        let mv = self.mv_list.pop().unwrap();
        let sq_src = util::src(mv);
        let sq_dst = util::dst(mv);
        let pc_dst = self.squares[sq_dst as usize];

        self.add_piece(sq_dst, pc_dst, pregen::PieceAction::DEL);
        self.add_piece(sq_src, pc_dst, pregen::PieceAction::ADD);
        let pc_src = self.pc_list.pop().unwrap();
        if pc_src > 0 {
            self.add_piece(sq_dst, pc_src, pregen::PieceAction::ADD)
        }
    }

    pub fn add_piece(&mut self, sq: isize, pc: isize, action: pregen::PieceAction) {
        self.squares[sq as usize] = match action {
            pregen::PieceAction::DEL => 0,
            pregen::PieceAction::ADD => pc,
        };
        let mut adjust = 0;
        if pc < 16 {
            adjust = pc - 8;
            let score = pregen::PIECE_VALUE[adjust as usize][sq as usize];
            match action {
                pregen::PieceAction::DEL => {
                    self.vl_white -= score;
                }
                pregen::PieceAction::ADD => {
                    self.vl_white += score;
                }
            }
        } else {
            adjust = pc - 16;
            let score = pregen::PIECE_VALUE[adjust as usize][util::square_fltp(sq)];
            match action {
                pregen::PieceAction::DEL => {
                    self.vl_black -= score;
                }
                pregen::PieceAction::ADD => {
                    self.vl_black += score;
                }
            }
            adjust += 7;
        }
        self.zobrist_key ^= pregen::PRE_GEN_ZOB_RIST_KEY_TABLE[adjust as usize][sq as usize];
        self.zobrist_lock ^= pregen::PRE_GEN_ZOB_RIST_LOCK_TABLE[adjust as usize][sq as usize];
    }

    pub fn checked(&self) -> bool {
        let self_side = util::side_tag(self.sd_player);
        let opp_side = util::opp_side_tag(self.sd_player);

        for sq_src in 0..256 {
            if self.squares[sq_src as usize] != self_side + pregen::PIECE_KING {
                continue;
            }

            let side_pawn = pregen::PIECE_PAWN + opp_side;
            if self.squares[util::square_forward(sq_src, self.sd_player) as usize] == side_pawn {
                return true;
            }

            if self.squares[(sq_src - 1) as usize] == side_pawn {
                return true;
            }
            // self_side 16 opp_side 8 player 1
            if self.squares[(sq_src + 1) as usize] == side_pawn {
                return true;
            }

            for i in 0..4usize {
                if self.squares[(sq_src + pregen::ADVISOR_DELTA[i]) as usize] != 0 {
                    continue;
                };

                let side_knight = pregen::PIECE_KNIGHT + opp_side;

                for n in 0..2usize {
                    if self.squares[(sq_src + pregen::KNIGHT_CHECK_DELTA[i][n]) as usize]
                        == side_knight
                    {
                        return true;
                    }
                }
            }

            for i in 0..4usize {
                let delta = pregen::KING_DELTA[i] as isize;
                let mut sq_dst = sq_src + delta;
                while pregen::IN_BROAD(sq_dst) {
                    let pc_dst = self.squares[sq_dst as usize];
                    if pc_dst > 0 {
                        if pc_dst == pregen::PIECE_ROOK + opp_side
                            || pc_dst == pregen::PIECE_KING + opp_side
                        {
                            return true;
                        }
                        break;
                    }
                    sq_dst += delta;
                }
                sq_dst += delta;
                while pregen::IN_BROAD(sq_dst) {
                    let pc_dst = self.squares[sq_dst as usize];
                    if pc_dst > 0 {
                        if pc_dst == pregen::PIECE_CANNON + opp_side {
                            return true;
                        }
                        break;
                    }
                    sq_dst += delta;
                }
            }
            return false;
        }
        return false;
    }

    pub fn generate_mvs(&self, vls_opt: Option<Vec<isize>>) -> (Vec<isize>, Vec<isize>) {
        let self_side = util::side_tag(self.sd_player);
        let opp_side = util::opp_side_tag(self.sd_player);
        let mut mvs = vec![];
        let mut vls = vec![];
        // let mut vls_is_none = true;
        if vls_opt.is_some() {
            // for v in vls_opt.clone().unwrap().iter() {
            // vls.push(*v);
            vls = vls_opt.clone().unwrap().to_vec();
            // vls_is_none = false;
            // }
        }

        for sq_src in 0..self.squares.len() {
            let pc_src = self.squares[sq_src];
            if pc_src & self_side == 0 {
                continue;
            }

            match pc_src - self_side {
                pregen::PIECE_KING => {
                    for i in 0..4usize {
                        let sq_dst = sq_src as isize + pregen::KING_DELTA[i];

                        if !pregen::IN_FORT(sq_dst) {
                            continue;
                        }
                        let pc_dst = self.squares[sq_dst as usize];

                        match vls_opt {
                            Some(_) => {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                    vls.push(pregen::MVV_LVA(pc_dst, 5));
                                }
                            }
                            None => {
                                if pc_dst & self_side == 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                }
                            }
                        }
                    }
                }
                pregen::PIECE_ADVISOR => {
                    for i in 0..4usize {
                        let sq_dst = sq_src as isize + pregen::ADVISOR_DELTA[i];

                        if !pregen::IN_FORT(sq_dst) {
                            continue;
                        }
                        let pc_dst = self.squares[sq_dst as usize];

                        match vls_opt {
                            Some(_) => {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                    vls.push(pregen::MVV_LVA(pc_dst, 1));
                                }
                            }
                            None => {
                                if pc_dst & self_side == 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                }
                            }
                        }
                    }
                }
                pregen::PIECE_BISHOP => {
                    for i in 0..4usize {
                        let mut sq_dst = sq_src as isize + pregen::ADVISOR_DELTA[i];

                        if !(pregen::IN_BROAD(sq_dst)
                            && pregen::HOME_HALF(sq_dst, self.sd_player)
                            && self.squares[sq_dst as usize] == 0)
                        {
                            continue;
                        }
                        sq_dst = sq_dst + pregen::ADVISOR_DELTA[i];
                        let pc_dst = self.squares[sq_dst as usize];

                        match vls_opt {
                            Some(_) => {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                    vls.push(pregen::MVV_LVA(pc_dst, 1));
                                }
                            }
                            None => {
                                if pc_dst & self_side == 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                }
                            }
                        }
                    }
                }
                pregen::PIECE_KNIGHT => {
                    for i in 0..4usize {
                        let mut sq_dst = sq_src.saturating_add_signed(pregen::KING_DELTA[i]);

                        if self.squares[sq_dst] > 0 {
                            continue;
                        }
                        for j in 0..2usize {
                            sq_dst = sq_src.saturating_add_signed(pregen::KNIGHT_DELTA[i][j]);
                            if !pregen::IN_BROAD(sq_dst as isize) {
                                continue;
                            }
                            let pc_dst = self.squares[sq_dst];
                            match vls_opt {
                                Some(_) => {
                                    if pc_dst & opp_side != 0 {
                                        mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                        vls.push(pregen::MVV_LVA(pc_dst, 1));
                                    }
                                }
                                None => {
                                    if pc_dst & self_side == 0 {
                                        mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                    }
                                }
                            }
                        }
                    }
                }
                pregen::PIECE_ROOK => {
                    for i in 0..4usize {
                        let delta = pregen::KING_DELTA[i];
                        let mut sq_dst = sq_src as isize + delta;

                        while pregen::IN_BROAD(sq_dst) {
                            let pc_dst = self.squares[sq_dst as usize];
                            if pc_dst == 0 {
                                if vls_opt.is_none() {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                }
                            } else {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));

                                    if let Some(_) = vls_opt {
                                        vls.push(pregen::MVV_LVA(pc_dst, 4));
                                    };
                                };
                                break;
                            };
                            sq_dst += delta;
                        }
                    }
                }
                pregen::PIECE_CANNON => {
                    for i in 0..4usize {
                        let delta = pregen::KING_DELTA[i];
                        let mut sq_dst = sq_src as isize + delta;
                        // i=1 delta= -1 sq_dst= 52 sq_src= 53

                        while pregen::IN_BROAD(sq_dst) {
                            let pc_dst = self.squares[sq_dst as usize];
                            if pc_dst == 0 {
                                if vls_opt.is_none() {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                }
                            } else {
                                break;
                            };
                            sq_dst += delta;
                        }
                        sq_dst = sq_dst + delta;

                        while pregen::IN_BROAD(sq_dst) {
                            let pc_dst = self.squares[sq_dst as usize];
                            if pc_dst > 0 {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));

                                    if let Some(_) = vls_opt {
                                        vls.push(pregen::MVV_LVA(pc_dst, 4));
                                    };
                                }
                                break;
                            }
                            sq_dst += delta;
                        }
                    }
                }
                pregen::PIECE_PAWN => {
                    let mut sq_dst = util::square_forward(sq_src as isize, self.sd_player);

                    if pregen::IN_BROAD(sq_dst) {
                        let pc_dst = self.squares[sq_dst as usize];

                        if vls_opt.is_none() {
                            if pc_dst & self_side == 0 {
                                mvs.push(util::merge(sq_src as isize, sq_dst));
                            }
                        } else if pc_dst & opp_side != 0 {
                            mvs.push(util::merge(sq_src as isize, sq_dst));
                            vls.push(pregen::MVV_LVA(pc_dst, 4));
                        };
                    }

                    if pregen::AWAY_HALF(sq_src as isize, self.sd_player) {
                        for delta in [-1, 1] {
                            sq_dst = sq_src as isize + delta;
                            if pregen::IN_BROAD(sq_dst) {
                                let pc_dst = self.squares[sq_dst as usize];
                                if vls_opt.is_none() {
                                    if pc_dst & self_side == 0 {
                                        mvs.push(util::merge(sq_src as isize, sq_dst));
                                    }
                                } else {
                                    mvs.push(util::merge(sq_src as isize, sq_dst));
                                    vls.push(pregen::MVV_LVA(pc_dst, 4));
                                }
                            }
                        }
                    }
                }
                _ => continue,
            };
        }

        (mvs, vls)
    }

    pub fn has_mate(&mut self) -> bool {
        let (mvs, _) = self.generate_mvs(None);
        for mv in mvs {
            if self.make_move(mv) {
                self.undo_make_move();
                return false;
            }
        }
        true
    }

    pub fn winner(&mut self) -> isize {
        if self.has_mate() {
            return 1 - self.sd_player;
        };
        let pc = pregen::PIECE_KING + util::side_tag(self.sd_player);
        let mut mate = 0;
        for i in 0..self.squares.len() {
            if self.squares[i] == pc {
                mate = i;
                break;
            }
        }
        if mate == 0 {
            return 1 - self.sd_player;
        }
        let mut vl_rep = self.rep_status(3);
        if vl_rep > 0 {
            vl_rep = self.rep_value(vl_rep);
            if -pregen::WIN_VALUE < vl_rep && vl_rep < pregen::WIN_VALUE as isize {
                return 2;
            }
            return self.sd_player;
        }
        let mut has_material = false;
        for i in 0..self.squares.len() {
            if pregen::IN_BROAD(i as isize) && self.squares[i] & 7 > 2 {
                has_material = true;
                break;
            }
        }
        if !has_material {
            return 2;
        }
        0
    }

    pub fn new_state(&mut self, hash: isize) -> MoveState {
        let mut state = MoveState::new(self.history.clone(), hash);
        if self.in_check() {
            state.phase = Status::REST;
            let (all_mvs, _) = self.generate_mvs(None);
            for mv in all_mvs {
                if !self.make_move(mv) {
                    continue;
                }
                self.undo_make_move();
                state.mvs.push(mv);
                if mv == state.hash {
                    state.vls.push(0x7fffffff);
                } else {
                    state
                        .vls
                        .push(self.history[self.history_index(mv) as usize])
                };
                util::shell_sort(&mut state.mvs, &mut state.vls);
                state.signle = state.mvs.len() == 1
            }
            state.hash = hash;
            state.killer_first = self.killer_table[self.distance as usize][0];
            state.killer_second = self.killer_table[self.distance as usize][1];
        }
        state
    }

    pub fn next_state(&mut self, state: &mut MoveState) -> isize {
        if state.phase == Status::HASH {
            state.phase = Status::KILLER_FIRST;
            if state.hash > 0 {
                return state.hash;
            }
        };

        if state.phase == Status::KILLER_FIRST {
            state.phase = Status::KILLER_SECOND;
            if state.killer_first != state.hash
                && state.killer_first > 0
                && self.legal_move(state.killer_first)
            {
                return state.killer_first;
            }
        };

        if state.phase == Status::KILLER_SECOND {
            state.phase = Status::GEN_MOVES;
            if state.killer_second != state.hash
                && state.killer_second > 0
                && self.legal_move(state.killer_second)
            {
                return state.killer_second;
            }
        };

        if state.phase == Status::GEN_MOVES {
            state.phase = Status::REST;

            let (mvs, _) = self.generate_mvs(None);
            state.mvs = mvs;
            state.vls = vec![];
            for mv in state.mvs.iter() {
                state
                    .vls
                    .push(self.history[self.history_index(*mv) as usize]);
            }
            util::shell_sort(&mut state.mvs, &mut state.vls);
            state.index = 0;
        };

        while state.index < state.mvs.len() {
            let mv = state.mvs[state.index];
            state.index += 1;
            if mv != state.hash && mv != state.killer_first && mv != state.killer_second {
                return mv;
            }
        }
        0
    }

    pub fn probe_hash(
        &self,
        vl_alpha: isize,
        vl_beta: isize,
        depth: isize,
        mvs: &mut Vec<isize>,
    ) -> isize {
        let hash_idx = (self.zobrist_key & self.mask) as usize;
        let mut hash = self.hash_table[hash_idx]; // todo set hash???
        if hash.zobrist_lock != self.zobrist_key {
            mvs[0] = 0;
            return -pregen::MATE_VALUE;
        };
        mvs[0] = hash.mv;

        let mut mate = false;

        if hash.vl > pregen::WIN_VALUE {
            if hash.vl <= pregen::BAN_VALUE {
                return -pregen::MATE_VALUE;
            }
            hash.vl = hash.vl - self.distance;
            mate = true;
        } else if hash.vl < -pregen::WIN_VALUE {
            if hash.vl > -pregen::BAN_VALUE {
                return -pregen::MATE_VALUE;
            };
            hash.vl = hash.vl + self.distance;
            mate = true;
        } else if hash.vl == self.draw_value() {
            return -pregen::MATE_VALUE;
        };

        if hash.depth < depth && !mate {
            return -pregen::MATE_VALUE;
        };

        if hash.flag == pregen::HASH_BETA {
            if hash.vl >= vl_beta {
                return hash.vl;
            };
            return -pregen::MATE_VALUE;
        };

        if hash.flag == pregen::HASH_ALPHA {
            if hash.vl <= vl_alpha {
                return hash.vl;
            }
            return -pregen::MATE_VALUE;
        }
        return hash.vl;
    }

    pub fn record_hash(&mut self, flag: isize, vl: isize, depth: isize, mv: isize) {
        let hash_idx = self.zobrist_key & self.mask;
        let mut hash = self.hash_table[hash_idx as usize];
        if hash.depth > depth {
            return;
        }

        hash.flag = flag;
        hash.depth = depth;
        if vl > pregen::WIN_VALUE {
            if mv == 0 && vl <= pregen::BAN_VALUE {
                return;
            };

            hash.vl = hash.vl + self.distance;
        } else if vl < -pregen::WIN_VALUE {
            if mv == 0 && vl <= pregen::BAN_VALUE {
                return;
            }
            hash.vl = hash.vl - self.distance;
        } else if vl == self.draw_value() && mv == 0 {
            return;
        } else {
            hash.vl = vl;
        };
        hash.mv = mv;
        hash.zobrist_lock = self.zobrist_lock;
        self.hash_table[hash_idx as usize] = hash;
    }

    pub fn set_best_move(&mut self, mv: isize, depth: isize) {
        let idx = self.history_index(mv) as usize;

        self.history[idx] += depth * depth;
        let killer = self.killer_table[self.distance as usize];
        if killer[0] != mv {
            self.killer_table[self.distance as usize] = [mv, killer[0]];
        };
    }

    pub fn search_pruning(&mut self, mut vl_alpha: isize, vl_beta: isize) -> isize {
        self.all_nodes += 1;

        let mut vl = self.mate_value();
        if vl >= vl_beta {
            return vl;
        };

        let vl_rep = self.rep_status(1);
        if vl_rep > 0 {
            return self.rep_value(vl_rep);
        };

        if self.distance == pregen::LIMIT_DEPTH as isize {
            return self.evaluate();
        };

        let mut vl_best = -pregen::MATE_VALUE;
        let mut mvs = vec![];
        let mut vls = vec![];

        if self.in_check() {
            (mvs, _) = self.generate_mvs(None);
            for mv in mvs.iter() {
                vls.push(self.history[self.history_index(*mv) as usize]);
            }
            util::shell_sort(&mut mvs, &mut vls);
        } else {
            vl = self.evaluate();

            if vl > vl_best {
                if vl >= vl_beta {
                    return vl;
                };
                vl_best = vl;
                if vl > vl_alpha {
                    vl_alpha = vl;
                }
            };

            let (mt, vt) = self.generate_mvs(Some(vls));
            mvs = mt;
            vls = vt;
            util::shell_sort(&mut mvs, &mut vls);
            for i in 0..mvs.len() {
                if vls[i] < 10
                    || (vls[i] < 20 && pregen::HOME_HALF(util::dst(mvs[i]), self.sd_player))
                {
                    mvs = mvs[0..i].to_vec();
                    break;
                }
            }
        };

        for i in 0..mvs.len() {
            if !self.make_move(mvs[i]) {
                continue;
            }
            vl = -self.search_pruning(-vl_beta, -vl_alpha);
            self.undo_make_move();
            if vl > vl_best {
                if vl >= vl_beta {
                    return vl;
                }
                vl_best = vl;
                if vl > vl_alpha {
                    vl_alpha = vl;
                };
            }
        }

        if vl_best == -pregen::MATE_VALUE {
            return self.mate_value();
        }

        return vl_best;
    }

    pub fn search_full(
        &mut self,
        mut vl_alpha: isize,
        vl_beta: isize,
        depth: isize,
        not_null: bool,
    ) -> isize {
        if depth <= 0 {
            return self.search_pruning(vl_alpha, vl_beta);
        };

        self.all_nodes += 1;
        let mut vl = self.mate_value() as isize;
        if vl > vl_beta {
            return vl;
        };

        let vl_rep = self.rep_status(1);
        if vl_rep > 0 {
            return self.rep_value(vl_rep);
        };

        let mut mv_hash = vec![0];
        vl = self.probe_hash(vl_alpha, vl_beta, depth, &mut mv_hash);
        if vl > -pregen::MATE_VALUE {
            return vl;
        };

        if self.distance == pregen::LIMIT_DEPTH as isize {
            return self.evaluate();
        };

        if !not_null && !self.in_check() && self.null_okay() {
            self.null_move();
            vl = -self.search_full(-vl_beta, 1 - vl_beta, depth - pregen::NULL_DEPTH - 1, true);
            self.undo_null_move();
            if vl >= vl_beta
                && (self.null_safe()
                    || self.search_full(vl_alpha, vl_beta, depth - pregen::NULL_DEPTH, true)
                        >= vl_beta)
            {
                return vl;
            }
        };

        let mut hash_flag = pregen::HASH_ALPHA;
        let mut vl_best = -pregen::MATE_VALUE;
        let mut mv_best = 0;

        let mut state = self.new_state(mv_hash[0]);
        loop {
            let mv = self.next_state(&mut state);
            if mv <= 0 {
                break;
            };
            if !self.make_move(mv) {
                continue;
            };

            let new_depth = match self.in_check() || state.signle {
                true => depth,
                false => depth - 1,
            };

            if vl_best == -pregen::MATE_VALUE {
                vl = -self.search_full(-vl_best, -vl_alpha, new_depth, false);
            } else {
                vl = -self.search_full(-vl_alpha - 1, -vl_alpha, new_depth, false);
                if vl_alpha < vl && vl < vl_beta {
                    vl = -self.search_full(-vl_beta, -vl_alpha, new_depth, false);
                };
            };
            self.undo_make_move();
            if vl > vl_best {
                vl_best = vl;
                if vl >= vl_beta {
                    hash_flag = pregen::HASH_BETA;
                    mv_best = mv;
                    break;
                };
                if vl > vl_alpha {
                    vl_alpha = vl;
                    hash_flag = pregen::HASH_PV;
                    mv_best = mv;
                }
            };
        }

        if vl_best == -pregen::MATE_VALUE {
            return self.mate_value();
        };

        self.record_hash(hash_flag, vl_best, depth, mv_best);
        if mv_best > 0 {
            self.set_best_move(mv_best, depth);
        };
        return vl_best;
    }

    pub fn search_root(&mut self, depth: isize) -> isize {
        let mut vl_best: isize = -pregen::MATE_VALUE;

        let mut state = self.new_state(self.result);
        let mut i = 0;
        loop {
            i += 1;
            let mv = self.next_state(&mut state);
            if mv <= 0 {
                break;
            };

            if !self.make_move(mv) {
                continue;
            };

            let new_depth: isize = match self.in_check() {
                true => depth,
                false => depth - 1,
            };

            let mut vl = 0;
            if vl_best == -pregen::MATE_VALUE {
                vl = -self.search_full(-pregen::MATE_VALUE, pregen::MATE_VALUE, new_depth, true);
            } else {
                vl = -self.search_full(-vl_best - 1, -vl_best, new_depth, false);
                if vl > vl_best {
                    vl = -self.search_full(-pregen::MATE_VALUE, -vl_best, new_depth, false);
                };
            };
            self.undo_make_move();
            if vl > vl_best {
                vl_best = vl;
                self.result = mv;
                if vl_best > -pregen::WIN_VALUE && vl_best < pregen::WIN_VALUE {
                    vl_best += (util::randf64(pregen::RANDOMNESS)
                        - util::randf64(pregen::RANDOMNESS))
                        as isize;
                    if vl_best == self.draw_value() {
                        vl_best -= 1;
                    }
                }
            }
        }
        self.set_best_move(self.result, depth);
        return vl_best;
    }

    pub fn search_unique(&mut self, vl_beta: isize, depth: isize) -> bool {
        let mut state = self.new_state(self.result);
        self.next_state(&mut state);

        loop {
            let mv = self.next_state(&mut state);
            if mv <= 0 {
                break;
            };
            if !self.make_move(mv) {
                continue;
            }
            let mut new_depth = depth;
            if !self.in_check() {
                new_depth -= 1;
            };
            let vl = -self.search_full(-vl_beta, 1 - vl_beta, new_depth, false);
            self.undo_make_move();
            if vl >= vl_beta {
                return false;
            }
        }
        return true;
    }

    pub fn search_main(&mut self, depth: isize, millis: u64) -> isize {
        self.result = self.book_move();
        if self.result > 0 {
            self.make_move(self.result);
            if self.rep_status(3) == 0 {
                self.undo_make_move();
                return self.result;
            };
            self.undo_make_move();
        };

        self.hash_table = vec![Hash::default(); self.mask as usize + 1];
        self.killer_table = vec![[0, 0]; pregen::LIMIT_DEPTH];
        self.history = vec![0; 4096];
        self.result = 0;
        self.all_nodes = 0;
        self.distance = 0;

        let start = Instant::now();
        let millis = Duration::from_millis(millis);
        for i in 1..depth + 1 {
            let vl = self.search_root(i);
            if Instant::now() - start >= millis {
                break;
            }
            if vl > pregen::WIN_VALUE || vl < -pregen::WIN_VALUE {
                break;
            };
            if self.search_unique(1 - pregen::WIN_VALUE, i) {
                break;
            };
        }
        return self.result;
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::ChessAi;

    #[test]
    fn test_fen() {
        let fen = "9/2Cca4/3k1C3/4P1p2/4N1b2/4R1r2/4c1n2/3p1n3/2rNK4/9 w";
        let mut engine = ChessAi::new();
        engine.from_fen(fen);
        assert_eq!(fen, engine.to_fen());
    }

    #[test]
    fn test_engine_26215() {
        let fen: &str = "9/2Cca4/3k1C3/4P1p2/4N1b2/4R1r2/4c1n2/3p1n3/2rNK4/9 w";
        let mut engine = ChessAi::new();
        engine.from_fen(fen);
        let mv = engine.search_main(64, 1000);
        assert_eq!(mv, 26215);
    }

    #[test]
    fn test_engine_22326() {
        let fen: &str = "C1nNk4/9/9/9/9/9/n1pp5/B3C4/9/3A1K3 w - - 0 1";
        let mut engine = ChessAi::new();
        engine.from_fen(fen);
        let mv = engine.search_main(64, 1000);
        assert_eq!(mv, 22326);
    }
    
    #[test]
    fn test_engine_22985() {
        let fen: &str = "4kab2/4a4/8b/9/9/9/9/9/9/4K1R2 w - - 0 1";
        let mut engine = ChessAi::new();
        engine.from_fen(fen);
        let mv = engine.search_main(64, 1000);
        assert_eq!(mv, 22985);
    }
}
