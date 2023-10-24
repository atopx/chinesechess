use super::util;

pub struct MoveState {
    mvs: Vec<isize>,
    vls: Vec<isize>,
    history: Vec<isize>,
    eng: super::core::Engine,
    index: usize,
    hash: isize,
    killer_first: isize,
    killer_second: isize,
    phase: super::pregen::Phase,
    pub signle: bool,
}

impl MoveState {
    pub fn new(
        eng: super::core::Engine,
        hash: isize,
        killer_table: Vec<[isize; 2]>,
        history: Vec<isize>,
    ) -> Self {
        let mut state = Self {
            mvs: vec![],
            vls: vec![],
            history,
            eng,
            index: 0,
            hash: 0,
            killer_first: 0,
            killer_second: 0,
            phase: super::pregen::Phase::HASH,
            signle: false,
        };

        if state.eng.in_check() {
            state.phase = super::pregen::Phase::REST;
            let all_mvs: Vec<isize> = state.eng.generate_mvs(&mut None);
            for mv in all_mvs {
                if state.eng.make_move(mv) {
                    continue;
                }
                state.eng.undo_make_move();
                state.mvs.push(mv);
                if mv == hash {
                    state.vls.push(0x7fffffff);
                } else {
                    state.vls.push(state.history[state.eng.history_index(mv)])
                };
                util::shell_sort(&mut state.mvs, &mut state.vls);
                state.signle = state.mvs.len() == 1
            }
            state.hash = hash;
            state.killer_first = killer_table[state.eng.distance as usize][0];
            state.killer_second = killer_table[state.eng.distance as usize][1];
        };
        state
    }

    pub fn next(&mut self) -> isize {
        if self.phase == super::pregen::Phase::HASH {
            self.phase = super::pregen::Phase::KILLER_FIRST;
            if self.hash > 0 {
                return self.hash;
            }
        };

        if self.phase == super::pregen::Phase::KILLER_FIRST {
            self.phase = super::pregen::Phase::KILLER_SECOND;
            if self.killer_first != self.hash && self.killer_first > 0 && self.eng.legal_move(self.killer_first) {
                return self.killer_first;
            }
        };

        if self.phase == super::pregen::Phase::KILLER_SECOND {
            self.phase = super::pregen::Phase::GEN_MOVES;
            if self.killer_second != self.hash && self.killer_second > 0 && self.eng.legal_move(self.killer_second) {
                return self.killer_second;
            }
        };

        if self.phase == super::pregen::Phase::GEN_MOVES {
            self.phase = super::pregen::Phase::REST;
            self.mvs = self.eng.generate_mvs(&mut None);
            self.vls = vec![];
            for mv in self.mvs.iter() {
                self.vls.push(self.history[self.eng.history_index(*mv)]);
            };
            util::shell_sort(&mut self.mvs, &mut self.vls);
            self.index = 0;
        };

        while self.index < self.mvs.len() {
            let mv =   self.mvs[self.index];
            self.index+= 1;
            if mv != self.hash && mv != self.killer_first && mv != self.killer_second {
                return mv;
            }
        };
        0
    }
}
