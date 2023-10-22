package book

type MoveSort struct {
	mvs, vls, historyTable       []int
	mvHash, mvKiller1, mvKiller2 int
	pos                          *Position
	index, phase                 int
	signleReply                  bool
}

func NewMoveSort(
	mvHash int,
	pos *Position,
	killerTable [][2]int,
	historyTable []int,
) *MoveSort {

	ms := &MoveSort{
		pos:          pos,
		historyTable: historyTable,
		phase:        PHASE_HASH,
	}

	if pos.InCheck() {
		ms.phase = PHASE_REST
		mvsAll := pos.GenerateMoves(nil)
		for _, mv := range mvsAll {
			if !pos.MakeMove(mv) {
				continue
			}
			pos.UndoMakeMove()
			ms.mvs = append(ms.mvs, mv)
			if mv == mvHash {
				ms.vls = append(ms.vls, 0x7fffffff)
			} else {
				ms.vls = append(ms.vls, historyTable[pos.HistoryIndex(mv)])
			}
			ShellSort(ms.mvs, ms.vls)
			ms.signleReply = len(ms.mvs) == 1
		}
		ms.mvHash = mvHash
		ms.mvKiller1 = killerTable[pos.Distance][0]
		ms.mvKiller2 = killerTable[pos.Distance][1]
	}
	return ms
}

func (m *MoveSort) Next() int {

	if m.phase == PHASE_HASH {
		m.phase = PHASE_KILLER_1
		if m.mvHash > 0 {
			return m.mvHash
		}
	}

	if m.phase == PHASE_KILLER_1 {
		m.phase = PHASE_KILLER_2
		if m.mvKiller1 != m.mvHash && m.mvKiller1 > 0 && m.pos.LegalMove(m.mvKiller1) {
			return m.mvKiller1
		}
	}

	if m.phase == PHASE_KILLER_2 {
		m.phase = PHASE_GEN_MOVES
		if m.mvKiller2 != m.mvHash && m.mvKiller2 > 0 && m.pos.LegalMove(m.mvKiller2) {
			return m.mvKiller2
		}
	}

	if m.phase == PHASE_GEN_MOVES {
		m.phase = PHASE_REST
		m.mvs = m.pos.GenerateMoves(nil)
		m.vls = []int{}
		for _, mv := range m.mvs {
			m.vls = append(m.vls, m.historyTable[m.pos.HistoryIndex(mv)])
		}
		ShellSort(m.mvs, m.vls)
		m.index = 0
	}

	for m.index < len(m.mvs) {
		mv := m.mvs[m.index]
		m.index++
		if mv != m.mvHash && mv != m.mvKiller1 && mv != m.mvKiller2 {
			return mv
		}
	}
	return 0
}
