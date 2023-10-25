package engine

type MoveSort struct {
	mvs, vls, historyTable       []int
	mvHash, mvKiller1, mvKiller2 int
	eng                          *Engine
	index, phase                 int
	signleReply                  bool
}

func NewMoveSort(
	mvHash int, eng *Engine,
	killerTable [][2]int,
	historyTable []int,
) *MoveSort {

	ms := &MoveSort{
		eng:          eng,
		historyTable: historyTable,
		phase:        PHASE_HASH,
	}

	if eng.InCheck() {
		ms.phase = PHASE_REST
		mvsAll := eng.GenerateMoves(nil)
		for _, mv := range mvsAll {
			if !eng.MakeMove(mv) {
				continue
			}
			eng.UndoMakeMove()
			ms.mvs = append(ms.mvs, mv)
			if mv == mvHash {
				ms.vls = append(ms.vls, 0x7fffffff)
			} else {
				ms.vls = append(ms.vls, historyTable[eng.HistoryIndex(mv)])
			}
			eng.ShellSort(ms.mvs, ms.vls)
			ms.signleReply = len(ms.mvs) == 1
		}
		ms.mvHash = mvHash
		ms.mvKiller1 = killerTable[eng.Distance][0]
		ms.mvKiller2 = killerTable[eng.Distance][1]
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
		if m.mvKiller1 != m.mvHash && m.mvKiller1 > 0 && m.eng.LegalMove(m.mvKiller1) {
			return m.mvKiller1
		}
	}

	if m.phase == PHASE_KILLER_2 {
		m.phase = PHASE_GEN_MOVES
		if m.mvKiller2 != m.mvHash && m.mvKiller2 > 0 && m.eng.LegalMove(m.mvKiller2) {
			return m.mvKiller2
		}
	}

	if m.phase == PHASE_GEN_MOVES {
		m.phase = PHASE_REST
		m.mvs = m.eng.GenerateMoves(nil)
		m.vls = []int{}
		for _, mv := range m.mvs {
			m.vls = append(m.vls, m.historyTable[m.eng.HistoryIndex(mv)])
		}
		m.eng.ShellSort(m.mvs, m.vls)
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
