use std::time::{Duration, Instant};

use rand::Rng;

use super::{
    pregen::{self, MATE_VALUE, WIN_VALUE},
    util,
};

#[derive(Clone, Copy, Default)]
struct Hash {
    depth: isize,
    flag: isize,
    vl: isize,
    mv: isize,
    zobrist_lock: isize,
}

pub struct Search {
    engine: super::core::Engine,
    mask: isize,
    hash_table: Vec<Hash>,
    history: Vec<isize>,
    killer_table: Vec<[isize; 2]>,
    result: isize,
    all_nodes: isize,
}

impl Search {
    pub fn new(eng: super::core::Engine, level: isize) -> Self {
        return Self {
            engine: eng,
            mask: (1 << level) - 1,
            hash_table: vec![],
            history: vec![],
            killer_table: vec![],
            result: 0,
            all_nodes: 0,
        };
    }

    pub fn probe_hash(
        &self,
        vl_alpha: isize,
        vl_beta: isize,
        depth: isize,
        mvs: &mut Vec<isize>,
    ) -> isize {
        let hash_idx = (self.engine.zobrist_key & self.mask) as usize;
        let mut hash = self.hash_table[hash_idx]; // todo set hash???
        if hash.zobrist_lock != self.engine.zobrist_key {
            mvs[0] = 0;
            return -pregen::MATE_VALUE;
        };
        mvs[0] = hash.mv;

        let mut mate = false;

        if hash.vl > pregen::WIN_VALUE {
            if hash.vl <= pregen::BAN_VALUE {
                return -pregen::MATE_VALUE;
            }
            hash.vl = hash.vl.saturating_sub_unsigned(self.engine.distance);
            mate = true;
        } else if hash.vl < -pregen::WIN_VALUE {
            if hash.vl > -pregen::BAN_VALUE {
                return -pregen::MATE_VALUE;
            };
            hash.vl = hash.vl.saturating_add_unsigned(self.engine.distance);
            mate = true;
        } else if hash.vl == self.engine.draw_value() {
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

    pub fn record_hash(&self, flag: isize, vl: isize, depth: isize, mv: isize) {
        let mut hash = self.hash_table[(self.engine.zobrist_key & self.mask) as usize];
        if hash.depth > depth {
            return;
        }

        hash.flag = flag;
        hash.depth = depth;
        if vl > pregen::WIN_VALUE {
            if mv == 0 && vl <= pregen::BAN_VALUE {
                return;
            };

            hash.vl = hash.vl.saturating_add_unsigned(self.engine.distance);
        } else if vl < -pregen::WIN_VALUE {
            if mv == 0 && vl <= pregen::BAN_VALUE {
                return;
            }
            hash.vl = hash.vl.saturating_sub_unsigned(self.engine.distance);
        } else if vl == self.engine.draw_value() && mv == 0 {
            return;
        } else {
            hash.vl = vl;
        };
        hash.mv = mv;
        hash.zobrist_lock = self.engine.zobrist_lock;
        // todo set hash???
    }

    pub fn set_best_move(&mut self, mv: isize, depth: isize) {
        self.history[self.engine.history_index(mv)] += (depth * depth);
        let killer = self.killer_table[self.engine.distance as usize];
        if killer[0] != mv {
            self.killer_table[self.engine.distance as usize] = [mv, killer[0]];
        };
    }

    pub fn search_pruning(&mut self, vl_alpha: isize, vl_beta: isize) -> isize {
        let mut vl_alpha = vl_alpha.to_owned();
        let mut vl_beta = vl_beta.to_owned();
        self.all_nodes += 1;
        let mut vl = self.engine.mate_value() as isize;
        if vl >= vl_beta {
            return vl;
        };
        let vl_rep = self.engine.rep_status(1);
        if vl_rep > 0 {
            return self.engine.rep_value(vl_rep);
        };
        if self.engine.distance == pregen::LIMIT_DEPTH {
            return self.engine.evalute();
        };
        let mut vl_best = -pregen::MATE_VALUE;
        let mut mvs = vec![];
        let mut vls = vec![];

        if self.engine.in_check() {
            mvs = self.engine.generate_mvs(&mut None);
            for mv in mvs.iter() {
                vls.push(self.history[self.engine.history_index(*mv)]);
            }
            util::shell_sort(&mut mvs, &mut vls);
        } else {
            vl = self.engine.evalute();
            if vl > vl_best {
                if vl >= vl_beta {
                    return vl;
                };
                vl_best = vl;
                if vl > vl_alpha {
                    vl_alpha = vl;
                }
            };
            mvs = self.engine.generate_mvs(&mut Some(vls.clone()));
            util::shell_sort(&mut mvs, &mut vls);
            for i in 0..mvs.len() {
                if vls[i] < 10
                    || (vls[i] < 20
                        && pregen::HOME_HALF(util::dst(mvs[i]) as usize, self.engine.sd_player))
                {
                    mvs = mvs[0..i].to_vec();
                    break;
                }
            }
        };
        for i in 0..mvs.len() {
            if !self.engine.make_move(mvs[i]) {
                continue;
            }
            vl = -self.search_pruning(-vl_beta, -vl_alpha);
            self.engine.undo_make_move();
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
            return self.engine.mate_value() as isize;
        }
        return vl_best;
    }

    pub fn search_full(
        &mut self,
        vl_alpha: isize,
        vl_beta: isize,
        depth: isize,
        not_null: bool,
    ) -> isize {
        if depth <= 0 {
            return self.search_pruning(vl_alpha, vl_beta);
        };
        self.all_nodes += 1;
        let mut vl = self.engine.mate_value() as isize;
        if vl > vl_beta {
            return vl;
        };
        let vl_rep = self.engine.rep_status(1);
        if vl_rep > 0 {
            return self.engine.rep_value(vl_rep);
        };
        let mut mv_hash = vec![0];
        vl = self.probe_hash(vl_alpha, vl_beta, depth, &mut mv_hash);
        if vl > -pregen::MATE_VALUE {
            return vl;
        };
        if self.engine.distance == pregen::LIMIT_DEPTH {
            return self.engine.evalute();
        };

        if !not_null && !self.engine.in_check() && self.engine.null_okey() {
            self.engine.null_move();
            vl = -self.search_full(-vl_beta, 1 - vl_beta, depth - pregen::NULL_DEPTH - 1, true);
            self.engine.undo_null_move();
            if vl >= vl_beta
                && (self.engine.null_safe()
                    || self.search_full(vl_alpha, vl_beta, depth - pregen::NULL_DEPTH, true)
                        >= vl_beta)
            {
                return vl;
            }
        };

        let mut hash_flag = pregen::HASH_ALPHA;
        let mut vl_best = -pregen::MATE_VALUE;
        let mut mv_best = 0;
        let mut state = super::state::MoveState::new(
            self.engine.clone(),
            mv_hash[0],
            self.killer_table.clone(),
            self.history.clone(),
        );
        let mut vl_alpha = vl_alpha;
        loop {
            let mv = state.next();
            if mv <= 0 {
                break;
            };
            if !self.engine.make_move(mv) {
                continue;
            };

            let new_depth = match self.engine.in_check() || state.signle {
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
            self.engine.undo_make_move();
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
            return self.engine.mate_value() as isize;
        };
        self.record_hash(hash_flag, vl_best, depth, mv_best);
        if mv_best > 0 {
            self.set_best_move(mv_best, depth);
        };
        return vl_best;
    }

    pub fn search_root(&mut self, depth: isize) -> isize {
        let mut vl_best = -pregen::MATE_VALUE;
        let mut state = super::state::MoveState::new(
            self.engine.clone(),
            self.result,
            self.killer_table.clone(),
            self.history.clone(),
        );
        loop {
            let mv = state.next();
            if mv < 0 {
                break;
            };
            if !self.engine.make_move(mv) {
                continue;
            };
            let new_depth = match self.engine.in_check() {
                true => depth,
                false => depth - 1,
            };
            let mut vl = 0;
            if vl_best == -pregen::MATE_VALUE {
                vl = -self.search_full(-pregen::MATE_VALUE, pregen::MATE_VALUE, new_depth, true);
            } else {
                vl = -self.search_full(-vl_best - 1, -vl_best, new_depth, false);
                if vl > vl_best {
                    vl = -self.search_full(-MATE_VALUE, -vl_best, new_depth, false);
                };
            };
            self.engine.undo_make_move();
            if vl > vl_best {
                vl_best = vl;
                self.result = mv;
                if vl_best > -pregen::WIN_VALUE && vl_best < pregen::WIN_VALUE {
                    let mut rng = rand::thread_rng();
                    let n1: f64 = rng.gen_range(0.0..1.0);
                    let n2: f64 = rng.gen_range(0.0..1.0);
                    vl_best += (n1.floor() * pregen::RANDOMNESS as f64
                        - n2.floor() * pregen::RANDOMNESS as f64)
                        as isize;
                    if vl_best == self.engine.draw_value() {
                        vl_best -= 1;
                    }
                }
            }
        }
        self.set_best_move(self.result, depth);
        return vl_best;
    }

    pub fn search_unique(&mut self, vl_beta: isize, depth: isize) -> bool {
        let mut state = super::state::MoveState::new(
            self.engine.clone(),
            self.result,
            self.killer_table.clone(),
            self.history.clone(),
        );
        state.next();

        loop {
            let mv = state.next();
            if mv <= 0 {
                break;
            };
            if !self.engine.make_move(mv) {
                continue;
            }
            let mut new_depth = depth;
            if !self.engine.in_check() {
                new_depth -= 1;
            };
            let vl = -self.search_full(-vl_beta, 1 - vl_beta, new_depth, false);
            self.engine.undo_make_move();
            if vl >= vl_beta {
                return false;
            }
        }
        return true;
    }

    pub fn search_main(&mut self, depth: isize, millis: u64) -> isize {
        self.result = self.engine.book_move();
        if self.result > 0 {
            self.engine.make_move(self.result);
            if self.engine.rep_status(3) == 0 {
                self.engine.undo_make_move();
                return self.result;
            };
            self.engine.undo_make_move();
        };

        self.hash_table = vec![];
        for _ in 0..self.mask {
            self.hash_table.push(Hash::default());
        }
        self.killer_table = vec![[0, 0]; pregen::LIMIT_DEPTH];
        self.history = vec![0; 4096];
        self.result = 0;
        self.all_nodes = 0;
        self.engine.distance = 0;

        let start = Instant::now();
        let millis = Duration::from_millis(millis);

        for i in 0..depth + 1 {
            let vl = self.search_root(i);
            if Instant::now() - start >= millis {
                break;
            }
            if vl > WIN_VALUE || vl <= pregen::WIN_VALUE {
                break;
            };
            if self.search_unique(1 - pregen::WIN_VALUE, i) {
                break;
            };
        }
        return self.result;
    }
}
