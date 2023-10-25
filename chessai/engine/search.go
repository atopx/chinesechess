package engine

import (
	"math"
	"math/rand"
	"time"
)

type HashObject struct{ Depth, Flag, Vl, Mv, ZobRistLock int }

type Search struct {
	engine       *Engine
	HashMask     int
	HashTable    []*HashObject
	HistoryTable []int
	KillerTable  [][2]int
	MvResult     int
	AllNodes     int
	AllMillis    int64
}

func NewSearch(pos *Engine, hashLevel int) *Search {
	return &Search{
		HashMask:     (1 << hashLevel) - 1,
		engine:       pos,
		HashTable:    []*HashObject{},
		HistoryTable: []int{},
		KillerTable:  [][2]int{},
	}
}

func (s *Search) GetHashItem() *HashObject {
	return s.HashTable[s.engine.ZobRistKey&s.HashMask]
}

func (s *Search) ProbeHash(vlAlpha, vlBeta, depth int, mvs []int) int {
	hash := s.GetHashItem()
	if hash.ZobRistLock != s.engine.ZobRistLock {
		mvs[0] = 0
		return -MATE_VALUE
	}
	mvs[0] = hash.Mv

	mate := false
	if hash.Vl > WIN_VALUE {
		if hash.Vl <= BAN_VALUE {
			return -MATE_VALUE
		}
		hash.Vl -= s.engine.Distance
		mate = true
	} else if hash.Vl < -WIN_VALUE {
		if hash.Vl > -BAN_VALUE {
			return -MATE_VALUE
		}
		hash.Vl += s.engine.Distance
		mate = true
	} else if hash.Vl == s.engine.DrawValue() {
		return -MATE_VALUE
	}

	if hash.Depth < depth && !mate {
		return -MATE_VALUE
	}

	if hash.Flag == HASH_BETA {
		if hash.Vl >= vlBeta {
			return hash.Vl
		}
		return -MATE_VALUE
	}

	if hash.Flag == HASH_ALPHA {
		if hash.Vl <= vlAlpha {
			return hash.Vl
		}
		return -MATE_VALUE
	}

	return hash.Vl
}

func (s *Search) RecordHash(flag, vl, depth, mv int) {
	hash := s.GetHashItem()
	if hash.Depth > depth {
		return
	}
	hash.Flag = flag
	hash.Depth = depth
	if vl > WIN_VALUE {
		if mv == 0 && vl <= BAN_VALUE {
			return
		}
		hash.Vl = vl + s.engine.Distance
	} else if vl < -WIN_VALUE {
		if mv == 0 && vl >= -BAN_VALUE {
			return
		}
		hash.Vl = vl - s.engine.Distance
	} else if vl == s.engine.DrawValue() && mv == 0 {
		return
	} else {
		hash.Vl = vl
	}
	hash.Mv = mv
	hash.ZobRistLock = s.engine.ZobRistLock
}

func (s *Search) SetBestMove(mv, depth int) {
	s.HistoryTable[s.engine.HistoryIndex(mv)] += depth * depth
	mvsKiller := s.KillerTable[s.engine.Distance]
	if mvsKiller[0] != mv {
		s.KillerTable[s.engine.Distance] = [2]int{mv, mvsKiller[0]}
	}
}

func (s *Search) SearchPruning(vlAlpha, vlBeta int) int {
	s.AllNodes += 1
	vl := s.engine.MateValue()
	if vl >= vlBeta {
		return vl
	}
	vlRep := s.engine.RepStatus(1)
	if vlRep > 0 {
		return s.engine.RepValue(vlRep)
	}
	if s.engine.Distance == LIMIT_DEPTH {
		return s.engine.Evaluate()
	}
	vlBest := -MATE_VALUE
	var mvs, vls []int

	if s.engine.InCheck() {
		mvs = s.engine.GenerateMoves(nil)
		for _, mv := range mvs {
			vls = append(vls, s.HistoryTable[s.engine.HistoryIndex(mv)])
		}
		s.engine.ShellSort(mvs, vls)
	} else {
		vl = s.engine.Evaluate()
		if vl > vlBest {
			if vl >= vlBeta {
				return vl
			}
			vlBest = vl
			if vl > vlAlpha {
				vlAlpha = vl
			}
		}
		mvs = s.engine.GenerateMoves(&vls)
		s.engine.ShellSort(mvs, vls)
		for i := 0; i < len(mvs); i++ {
			if vls[i] < 10 || (vls[i] < 20 && HOME_HALF(DST(mvs[i]), s.engine.SdPlayer)) {
				mvs = mvs[:i]
				break
			}
		}
	}
	for i := 0; i < len(mvs); i++ {
		if !s.engine.MakeMove(mvs[i]) {
			continue
		}
		vl = -s.SearchPruning(-vlBeta, -vlAlpha)
		s.engine.UndoMakeMove()
		if vl > vlBest {
			if vl >= vlBeta {
				return vl
			}
			vlBest = vl
			if vl > vlAlpha {
				vlAlpha = vl
			}
		}
	}
	if vlBest == -MATE_VALUE {
		return s.engine.MateValue()
	}
	return vlBest
}

func (s *Search) SearchFull(vlAlpha, vlBeta, depth int, noNull bool) int {
	if depth <= 0 {
		return s.SearchPruning(vlAlpha, vlBeta)
	}
	s.AllNodes++
	vl := s.engine.MateValue()
	if vl >= vlBeta {
		return vl
	}
	vlRep := s.engine.RepStatus(1)
	if vlRep > 0 {
		return s.engine.RepValue(vlRep)
	}
	mvHash := []int{0}
	vl = s.ProbeHash(vlAlpha, vlBeta, depth, mvHash)
	if vl > -MATE_VALUE {
		return vl
	}
	if s.engine.Distance == LIMIT_DEPTH {
		return s.engine.Evaluate()
	}
	if !noNull && !s.engine.InCheck() && s.engine.NullOkay() {
		s.engine.NullMove()
		vl = -s.SearchFull(-vlBeta, 1-vlBeta, depth-NULL_DEPTH-1, true)
		s.engine.UndoNullMove()
		if vl >= vlBeta && (s.engine.NullSafe() || s.SearchFull(vlAlpha, vlBeta, depth-NULL_DEPTH, true) >= vlBeta) {
			return vl
		}
	}
	hashFlag := HASH_ALPHA
	vlBest := -MATE_VALUE
	mvBest := 0
	sort := NewMoveSort(mvHash[0], s.engine, s.KillerTable, s.HistoryTable)

	for {
		mv := sort.Next()
		if mv <= 0 {
			break
		}

		if !s.engine.MakeMove(mv) {
			continue
		}

		var newDepth int
		if s.engine.InCheck() || sort.signleReply {
			newDepth = depth
		} else {
			newDepth = depth - 1
		}

		if vlBest == -MATE_VALUE {
			vl = -s.SearchFull(-vlBeta, -vlAlpha, newDepth, false)
		} else {
			vl = -s.SearchFull(-vlAlpha-1, -vlAlpha, newDepth, false)
			if vlAlpha < vl && vl < vlBeta {
				vl = -s.SearchFull(-vlBeta, -vlAlpha, newDepth, false)
			}
		}
		s.engine.UndoMakeMove()
		if vl > vlBest {
			vlBest = vl
			if vl >= vlBeta {
				hashFlag = HASH_BETA
				mvBest = mv
				break
			}
			if vl > vlAlpha {
				vlAlpha = vl
				hashFlag = HASH_PV
				mvBest = mv
			}
		}
	}
	if vlBest == -MATE_VALUE {
		return s.engine.MateValue()
	}
	s.RecordHash(hashFlag, vlBest, depth, mvBest)
	if mvBest > 0 {
		s.SetBestMove(mvBest, depth)
	}
	return vlBest
}

func (s *Search) SearchRoot(depth int) int {
	vlBest := -MATE_VALUE
	sort := NewMoveSort(s.MvResult, s.engine, s.KillerTable, s.HistoryTable)
	for {
		mv := sort.Next()
		if mv <= 0 {
			break
		}
		if !s.engine.MakeMove(mv) {
			continue
		}
		newDepth := 0
		if s.engine.InCheck() {
			newDepth = depth
		} else {
			newDepth = depth - 1
		}

		var vl int
		if vlBest == -MATE_VALUE {
			vl = -s.SearchFull(-MATE_VALUE, MATE_VALUE, newDepth, true)
		} else {
			vl = -s.SearchFull(-vlBest-1, -vlBest, newDepth, false)
			if vl > vlBest {
				vl = -s.SearchFull(-MATE_VALUE, -vlBest, newDepth, false)
			}
		}
		s.engine.UndoMakeMove()
		if vl > vlBest {
			vlBest = vl
			s.MvResult = mv
			if -WIN_VALUE < vlBest && vlBest < WIN_VALUE {
				vlBest += int(math.Floor(rand.Float64()*RANDOMNESS) - math.Floor(rand.Float64()*RANDOMNESS))
				if vlBest == s.engine.DrawValue() {
					vlBest--
				}
			}
		}
	}
	s.SetBestMove(s.MvResult, depth)
	return vlBest
}

func (s *Search) SearchUnique(vlBeta, depth int) bool {
	sort := NewMoveSort(s.MvResult, s.engine, s.KillerTable, s.HistoryTable)
	sort.Next()

	for {
		mv := sort.Next()
		if mv <= 0 {
			break
		}
		if !s.engine.MakeMove(mv) {
			continue
		}

		newDepth := depth

		if !s.engine.InCheck() {
			newDepth--
		}

		vl := -s.SearchFull(-vlBeta, 1-vlBeta, newDepth, false)

		s.engine.UndoMakeMove()
		if vl >= vlBeta {
			return false
		}
	}
	return true
}

func (s *Search) SearchMain(depth, millis int) int {
	s.MvResult = s.engine.BookMove()
	if s.MvResult > 0 {
		s.engine.MakeMove(s.MvResult)
		if s.engine.RepStatus(3) == 0 {
			s.engine.UndoMakeMove()
			return s.MvResult
		}
		s.engine.UndoMakeMove()
	}

	s.HashTable = make([]*HashObject, 0, len(s.HashTable))
	for i := 0; i < s.HashMask+1; i++ {
		s.HashTable = append(s.HashTable, &HashObject{
			Depth: 0, Flag: 0, Vl: 0, Mv: 0, ZobRistLock: 0,
		})
	}
	s.KillerTable = [][2]int{}
	for i := 0; i < LIMIT_DEPTH; i++ {
		s.KillerTable = append(s.KillerTable, [2]int{0, 0})
	}
	s.HistoryTable = []int{}
	for i := 0; i < 4096; i++ {
		s.HistoryTable = append(s.HistoryTable, 0)
	}
	s.MvResult = 0
	s.AllNodes = 0
	s.engine.Distance = 0
	t := time.Now().Unix()
	for i := 1; i < depth+1; i++ {
		vl := s.SearchRoot(i)
		s.AllMillis = (time.Now().Unix() - t) * 1000
		if s.AllMillis > int64(millis) {
			break
		}
		if vl > WIN_VALUE || vl < -WIN_VALUE {
			break
		}
		if s.SearchUnique(1-WIN_VALUE, i) {
			break
		}
	}
	return s.MvResult
}
