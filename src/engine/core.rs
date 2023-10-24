use rand::Rng;

use super::{
    book::Book,
    pregen::{self, IN_BROAD},
    util,
};

#[derive(Clone)] // TODO 优化为指针
pub struct Engine {
    pub sd_player: usize,
    pub zobrist_key: isize,
    pub zobrist_lock: isize,
    vl_white: isize,
    vl_black: isize,
    pub distance: usize,
    mv_list: Vec<isize>,
    pc_list: Vec<usize>,
    key_list: Vec<isize>,
    chk_list: Vec<bool>,
    pub squares: [usize; 256],
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
                        self.add_piece(
                            util::coord_xy(x, y) as usize,
                            pt + 8,
                            pregen::PieceAction::ADD,
                        );
                    };
                    x += 1;
                }
            } else if c >= 'a' && c <= 'z' {
                if x <= pregen::FILE_RIGHT {
                    let pt = pregen::from_char((c as u8 + b'A' - b'a') as char);
                    if let Some(pt) = pregen::from_char((c as u8 + b'A' - b'a') as char) {
                        self.add_piece(
                            util::coord_xy(x, y) as usize,
                            pt + 16,
                            pregen::PieceAction::ADD,
                        );
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
                    row.push(pregen::FEN_PIECE[pc]);
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

    pub fn mate_value(&self) -> usize {
        self.distance.saturating_add_signed(pregen::MATE_VALUE)
    }

    pub fn ban_value(&self) -> usize {
        self.distance.saturating_add_signed(pregen::BAN_VALUE)
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

    pub fn in_check(&self) -> bool {
        self.chk_list[self.chk_list.len() - 1]
    }

    pub fn captured(&self) -> bool {
        self.pc_list[self.pc_list.len() - 1] > 0
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

    pub fn history_index(&self, mv: isize) -> usize {
        (self.squares[util::src(mv) as usize] - 8) << 8 + util::dst(mv)
    }

    pub fn book_move(&self) -> isize {
        let mut mirror_opt: bool = false;
        let mut lock = util::unsigned_right_shift(self.zobrist_lock, 1);
        let mut index_opt = Book::get().search(lock);
        let book = Book::get();
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
        // todo
        let mut value = 0;
        let book = Book::get();
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
        let mut rng = rand::thread_rng();
        let num: f64 = rng.gen_range(0.0..1.0);
        value = (num.floor() * (value as f64)) as isize;
        for index in 0..mvs.len() {
            value -= vls[index];
            if value < 0 {
                break;
            }
        }
        return mvs[index];
    }

    pub fn legal_move(&self, mv: isize) -> bool {
        let sq_src = util::src(mv) as usize;
        let pc_src = self.squares[sq_src];

        let self_side = util::side_tag(self.sd_player as isize) as usize;
        if pc_src & self_side == 0 {
            return false;
        }
        let sq_dst = util::dst(mv) as usize;
        let pc_dst = self.squares[sq_dst];
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
                pin != sq_src && self.squares[pin] == 0
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

                let mut pin = sq_src.saturating_add_signed(delta);

                while pin != sq_dst && self.squares[pin] == 0 {
                    pin = pin.saturating_add_signed(delta);
                }

                if pin == sq_dst {
                    return pc_dst == 0 || pc_src - self_side == pregen::PIECE_ROOK;
                }

                if pc_dst == 0 || pc_src - self_side != pregen::PIECE_CANNON {
                    return false;
                }

                pin = pin.saturating_add_signed(delta);
                while pin != sq_dst && self.squares[pin] == 0 {
                    pin = pin.saturating_add_signed(delta);
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
                    util::mirror_square(i as isize) as usize,
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
        let sq_src = util::src(mv) as usize;
        let sq_dst = util::dst(mv) as usize;
        let pc_dst = self.squares[sq_dst];
        self.pc_list.push(pc_dst);
        if pc_dst > 0 {
            self.add_piece(sq_src, pc_dst, pregen::PieceAction::DEL);
        }
        let pc_src = self.squares[sq_src];
        self.add_piece(sq_src, pc_src, pregen::PieceAction::DEL);
        self.add_piece(sq_dst, pc_src, pregen::PieceAction::ADD);
        self.mv_list.push(mv);
    }

    pub fn make_move(&mut self, mv: isize) -> bool {
        let zobrist_key = self.zobrist_key;
        self.move_piece(mv);
        if self.checked() {
            self.undo_move_piece();
            return false;
        }
        self.key_list.push(zobrist_key);
        self.change_side();
        self.chk_list.push(self.checked());
        self.distance += 1;
        return true;
    }

    pub fn undo_make_move(&mut self) {
        self.distance -= 1;
        self.chk_list.pop();
        self.change_side();
        self.key_list.pop();
        self.undo_move_piece();
    }

    pub fn undo_move_piece(&mut self) {
        let mv = self.mv_list.pop().unwrap();
        let sq_src = util::src(mv) as usize;
        let sq_dst = util::dst(mv) as usize;
        let pc_dst = self.squares[sq_dst];
        self.add_piece(sq_dst, pc_dst, pregen::PieceAction::DEL);
        self.add_piece(sq_src, pc_dst, pregen::PieceAction::ADD);
        let pc_src = self.pc_list.pop().unwrap();
        if pc_src > 0 {
            self.add_piece(sq_dst, pc_src, pregen::PieceAction::ADD)
        }
    }

    pub fn add_piece(&mut self, sq: usize, pc: usize, action: pregen::PieceAction) {
        let mut adjust = 0;
        if pc < 16 {
            adjust = pc - 8;
            let score = pregen::PIECE_VALUE[adjust][sq];
            self.squares[sq] = match action {
                pregen::PieceAction::ADD => {
                    self.vl_white += score;
                    pc
                }
                pregen::PieceAction::DEL => {
                    self.vl_white -= score;
                    0
                }
            }
        } else {
            adjust = pc - 16;
            let score = pregen::PIECE_VALUE[adjust][util::square_fltp(sq)];
            adjust += 7;
            self.squares[sq] = match action {
                pregen::PieceAction::ADD => {
                    self.vl_black += score;
                    pc
                }
                pregen::PieceAction::DEL => {
                    self.vl_black -= score;
                    0
                }
            }
        }
        self.zobrist_key ^= pregen::PRE_GEN_ZOB_RIST_KEY_TABLE[adjust][sq];
        self.zobrist_lock ^= pregen::PRE_GEN_ZOB_RIST_LOCK_TABLE[adjust][sq];
    }

    pub fn checked(&self) -> bool {
        let self_side = util::side_tag(self.sd_player as isize);
        let opp_side = util::opp_side_tag(self.sd_player as isize);
        let mut sq_src: usize = 0;

        for sql_src in 0..256 as usize {
            if self.squares[sq_src] as isize != self_side + opp_side {
                continue;
            }

            let side_pawn = pregen::PIECE_PAWN.saturating_add_signed(opp_side);

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
                let side_knght = pregen::PIECE_KNIGHT.saturating_add_signed(opp_side);

                for n in 0..2 as usize {
                    if self.squares[sq_src.saturating_add_signed(pregen::KNIGHT_CHECK_DELTA[i][n])]
                        == side_knght
                    {
                        return true;
                    }
                }
            }

            for i in 0..4 as usize {
                let delta = pregen::KING_DELTA[i] as usize;
                let mut sq_dst = sq_src + delta;
                while pregen::IN_BROAD(sq_dst) {
                    let pc_dst = self.squares[sq_dst];
                    if pc_dst > 0 {
                        if pc_dst == pregen::PIECE_ROOK.saturating_add_signed(opp_side)
                            || pc_dst == pregen::PIECE_KING.saturating_add_signed(opp_side)
                        {
                            return true;
                        }
                        break;
                    }
                    sq_dst += delta;
                }
                sq_dst += delta;
                while pregen::IN_BROAD(sq_dst) {
                    let pc_dst = self.squares[sq_dst];
                    if pc_dst > 0 {
                        if pc_dst == pregen::PIECE_CANNON.saturating_add_signed(opp_side) {
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

    pub fn generate_mvs(&self, vls_opt: &mut Option<Vec<isize>>) -> Vec<isize> {
        let self_side = util::side_tag(self.sd_player as isize) as usize;
        let opp_side = util::opp_side_tag(self.sd_player as isize) as usize;
        let mut mvs = vec![];
        for sq_src in 0..self.squares.len() {
            let pc_src = self.squares[sq_src];
            if pc_src & self_side == 0 {
                continue;
            }

            match pc_src - self_side {
                pregen::PIECE_KING => {
                    for i in 0..4 as usize {
                        let sq_dst = sq_src.saturating_add_signed(pregen::KING_DELTA[i]);
                        if !pregen::IN_FORT(sq_dst) {
                            continue;
                        }
                        let pc_dst = self.squares[sq_dst];

                        match vls_opt {
                            Some(vls) => {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                    vls.push(pregen::MVV_LVA(pc_dst, 5) as isize);
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
                pregen::PIECE_ADVISOR => {
                    for i in 0..4 as usize {
                        let sq_dst = sq_src.saturating_add_signed(pregen::ADVISOR_DELTA[i]);
                        if !pregen::IN_FORT(sq_dst) {
                            continue;
                        }
                        let pc_dst = self.squares[sq_dst];

                        match vls_opt {
                            Some(vls) => {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                    vls.push(pregen::MVV_LVA(pc_dst, 1) as isize);
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
                pregen::PIECE_BISHOP => {
                    for i in 0..4 as usize {
                        let mut sq_dst = sq_src.saturating_add_signed(pregen::ADVISOR_DELTA[i]);
                        if !(pregen::IN_BROAD(sq_dst)
                            && pregen::HOME_HALF(sq_dst, self.sd_player)
                            && self.squares[sq_dst] == 0)
                        {
                            continue;
                        }
                        sq_dst = sq_dst.saturating_add_signed(pregen::ADVISOR_DELTA[i]);
                        let pc_dst = self.squares[sq_dst];

                        match vls_opt {
                            Some(vls) => {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                    vls.push(pregen::MVV_LVA(pc_dst, 1) as isize);
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
                pregen::PIECE_KNIGHT => {
                    for i in 0..4 as usize {
                        let mut sq_dst = sq_src.saturating_add_signed(pregen::KING_DELTA[i]);
                        if self.squares[sq_dst] > 0 {
                            continue;
                        }
                        for j in 0..2 as usize {
                            sq_dst = sq_src.saturating_add_signed(pregen::KNIGHT_DELTA[i][j]);
                            if !IN_BROAD(sq_dst) {
                                continue;
                            }
                            let pc_dst = self.squares[sq_dst];
                            match vls_opt {
                                Some(vls) => {
                                    if pc_dst & opp_side != 0 {
                                        mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                        vls.push(pregen::MVV_LVA(pc_dst, 1) as isize);
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
                    for i in 0..4 as usize {
                        let delta = pregen::KING_DELTA[i];
                        let sq_dst = sq_src.saturating_add_signed(delta);
                        while pregen::IN_BROAD(sq_dst) {
                            let pc_dst = self.squares[sq_dst];
                            if pc_dst == 0 {
                                if vls_opt.is_none() {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                }
                            } else {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));

                                    if let Some(vls) = vls_opt {
                                        vls.push(pregen::MVV_LVA(pc_dst, 4) as isize);
                                    };
                                };
                            };
                        }
                    }
                }
                pregen::PIECE_CANNON => {
                    // todo
                    for i in 0..4 as usize {
                        let delta = pregen::KING_DELTA[i];
                        let mut sq_dst = sq_src.saturating_add_signed(delta);
                        while pregen::IN_BROAD(sq_dst) {
                            let pc_dst = self.squares[sq_dst];
                            if pc_dst == 0 {
                                if vls_opt.is_none() {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                }
                            } else {
                                break;
                            };
                            sq_dst = sq_dst.saturating_add_signed(delta);
                        }
                        sq_dst = sq_dst.saturating_add_signed(delta);

                        while pregen::IN_BROAD(sq_dst) {
                            let pc_dst = self.squares[sq_dst];
                            if pc_dst > 0 {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                    if let Some(vls) = vls_opt {
                                        vls.push(pregen::MVV_LVA(pc_dst, 4) as isize);
                                    };
                                }
                                break;
                            }
                            sq_dst = sq_dst.saturating_add_signed(delta);
                        }
                    }
                }
                pregen::PIECE_PAWN => {
                    let mut sq_dst = util::square_forward(sq_src, self.sd_player);
                    if pregen::IN_BROAD(sq_dst) {
                        let pc_dst = self.squares[sq_dst];
                        match vls_opt {
                            Some(vls) => {
                                if pc_dst & opp_side != 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                    vls.push(pregen::MVV_LVA(pc_dst, 4) as isize);
                                }
                            }
                            None => {
                                if pc_dst & self_side == 0 {
                                    mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                }
                            }
                        }
                    }
                    if pregen::AWAY_HALF(sq_src, self.sd_player) {
                        for delta in [-1, 1] {
                            sq_dst = sq_src.saturating_add_signed(delta);
                            if pregen::IN_BROAD(sq_dst) {
                                let pc_dst = self.squares[sq_dst];
                                match vls_opt {
                                    Some(vls) => {
                                        if pc_dst & opp_side != 0 {
                                            mvs.push(util::merge(sq_src as isize, sq_dst as isize));
                                            vls.push(pregen::MVV_LVA(pc_dst, 4) as isize);
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
                }
                _ => continue,
            };
        }
        mvs
    }

    pub fn is_mate(&mut self) -> bool {
        let mvs = self.generate_mvs(&mut None);
        for mv in mvs {
            if self.make_move(mv) {
                self.undo_make_move();
                return false;
            }
        }
        true
    }

    pub fn winner(&mut self) -> usize {
        if self.is_mate() {
            return 1 - self.sd_player;
        };
        let pc = pregen::PIECE_KING.saturating_add_signed(util::side_tag(self.sd_player as isize));
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
            if IN_BROAD(i) && self.squares[i] & 7 > 2 {
                has_material = true;
                break;
            }
        }
        if !has_material {
            return 2;
        }
        0
    }
}
