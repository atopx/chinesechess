package book

import (
	"fmt"
	"math"
	"math/rand"
	"time"
)

type Search struct {
	HashMask     int
	Pos          *Position
	HashTable    []*HashTableObject
	HistoryTable []int
	KillerTable  [][2]int
	MvResult     int
	AllNodes     int
	AllMillis    int64
}

func NewSearch(pos *Position, hashLevel int) *Search {
	return &Search{
		HashMask:     (1 << hashLevel) - 1,
		Pos:          pos,
		HashTable:    []*HashTableObject{},
		HistoryTable: []int{},
		KillerTable:  [][2]int{},
	}
}

func (s *Search) GetHashItem() *HashTableObject {
	return s.HashTable[s.Pos.ZobristKey&s.HashMask]
}

func (s *Search) ProbeHash(vlAlpha, vlBeta, depth int, mvs []int) int {
	hash := s.GetHashItem()
	if hash.ZobristLock != s.Pos.ZobristLock {
		mvs[0] = 0
		return -MATE_VALUE
	}
	mvs[0] = hash.Mv

	mate := false
	if hash.Vl > WIN_VALUE {
		if hash.Vl <= BAN_VALUE {
			return -MATE_VALUE
		}
		hash.Vl -= s.Pos.Distance
		mate = true
	} else if hash.Vl < -WIN_VALUE {
		if hash.Vl > -BAN_VALUE {
			return -MATE_VALUE
		}
		hash.Vl += s.Pos.Distance
		mate = true
	} else if hash.Vl == s.Pos.DrawValue() {
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
		hash.Vl = vl + s.Pos.Distance
	} else if vl < -WIN_VALUE {
		if mv == 0 && vl >= -BAN_VALUE {
			return
		}
		hash.Vl = vl - s.Pos.Distance
	} else if vl == s.Pos.DrawValue() && mv == 0 {
		return
	} else {
		hash.Vl = vl
	}
	hash.Mv = mv
	hash.ZobristLock = s.Pos.ZobristLock
}

func (s *Search) SetBestMove(mv, depth int) {
	s.HistoryTable[s.Pos.HistoryIndex(mv)] += depth * depth
	// mvsKiller := s.KillerTable[s.Pos.distance]
	// if mvsKiller[0] != mv {
	// 	mvsKiller[1] = mvsKiller[0]
	// 	mvsKiller[0] = mv
	// }

}

func (s *Search) SearchQuiesc(vlAlpha, vlBeta int) int {
	s.AllNodes += 1

	vl := s.Pos.MateValue()
	if vl >= vlBeta {
		return vl
	}
	vlRep := s.Pos.RepStatus(1)
	if vlRep > 0 {
		return s.Pos.RepValue(vlRep)
	}

	if s.Pos.Distance == LIMIT_DEPTH {
		return s.Pos.Evaluate()
	}

	vlBest := -MATE_VALUE
	mvs := []int{}
	vls := []int{}

	if s.Pos.InCheck() {
		mvs = s.Pos.GenerateMoves(nil)
		for _, mv := range mvs {
			vls = append(vls, s.HistoryTable[s.Pos.HistoryIndex(mv)])
		}
		ShellSort(mvs, vls)
	} else {
		vl = s.Pos.Evaluate()
		if vl > vlBest {
			if vl >= vlBest {
				return vl
			}
			vlBest = vl
			if vl > vlAlpha {
				vlAlpha = vl
			}
		}
		mvs = s.Pos.GenerateMoves(vls)
		ShellSort(mvs, vls)
		for i := 0; i < len(mvs); i++ {
			if vls[i] < 10 || (vls[i] < 20 && HOME_HALF(DST(mvs[i]), s.Pos.SdPlayer)) {
				mvs = mvs[:i]
				break
			}
		}
	}
	for i := 0; i < len(mvs); i++ {
		if !s.Pos.MakeMove(mvs[i]) {
			continue
		}
		vl = -s.SearchQuiesc(-vlBest, -vlAlpha)
		s.Pos.UndoMakeMove()
		if vl > vlBest {
			if vl >= vlBest {
				return vl
			}
			vlBest = vl
			if vl > vlAlpha {
				vlAlpha = vl
			}
		}
	}
	if vlBest == -MATE_VALUE {
		s.Pos.MateValue()
	}
	return vlBest
}

func (s *Search) SearchFull(vlAlpha, vlBeta, depth int, noNull bool) int {
	if depth <= 0 {
		return s.SearchQuiesc(vlAlpha, vlBeta)
	}

	s.AllNodes++
	vl := s.Pos.MateValue()
	if vl >= vlBeta {
		return vl
	}

	vlRep := s.Pos.RepStatus(1)
	if vlRep > 0 {
		return s.Pos.RepValue(vlRep)
	}

	mvHash := []int{0}
	vl = s.ProbeHash(vlAlpha, vlBeta, depth, mvHash)
	if vl > -MATE_VALUE {
		return vl
	}

	if s.Pos.Distance == LIMIT_DEPTH {
		return s.Pos.Evaluate()
	}

	if !noNull && !s.Pos.InCheck() && s.Pos.NullOkay() {
		s.Pos.NullMove()
		vl = -s.SearchFull(-vlBeta, 1-vlBeta, depth-NULL_DEPTH-1, true)
		s.Pos.UndoNullMove()
		if vl >= vlBeta && (s.Pos.NullSafe() || s.SearchFull(vlAlpha, vlBeta, depth-NULL_DEPTH, true) >= vlBeta) {
			return vl
		}
	}
	hashFlag := HASH_ALPHA
	vlBest := -MATE_VALUE
	mvBest := 0
	sort := NewMoveSort(mvHash[0], s.Pos, s.KillerTable, s.HistoryTable)

	for {
		mv := sort.Next()
		if mv <= 0 {
			break
		}

		if !s.Pos.MakeMove(mv) {
			continue
		}

		var newDepth int
		if s.Pos.InCheck() || sort.signleReply {
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
		s.Pos.UndoMakeMove()
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
		return s.Pos.MateValue()
	}
	s.RecordHash(hashFlag, vlBest, depth, mvBest)
	if mvBest > 0 {
		s.SetBestMove(mvBest, depth)
	}
	return vlBest
}

func (s *Search) SearchRoot(depth int) int {
	vlBest := -MATE_VALUE
	sort := NewMoveSort(s.MvResult, s.Pos, s.KillerTable, s.HistoryTable)

	for {
		mv := sort.Next()

		if mv <= 0 {
			break
		}
		if !s.Pos.MakeMove(mv) {
			continue
		}

		newDepth := 0
		if s.Pos.InCheck() {
			newDepth = depth
		} else {
			newDepth = depth - 1
		}
		var vl int
		if vlBest == -MATE_VALUE {
			vl = -s.SearchFull(-MATE_VALUE, MATE_VALUE, newDepth, true)
		} else {
			fmt.Println(vlBest, newDepth)
			vl = -s.SearchFull(-vlBest-1, -vlBest, newDepth, false)
			if vl > vlBest {
				vl = -s.SearchFull(-MATE_VALUE, -vlBest, newDepth, false)
			}
		}
		s.Pos.UndoMakeMove()
		if vl > vlBest {
			vlBest = vl
			s.MvResult = mv
			if -WIN_VALUE < vlBest && vlBest < WIN_VALUE {
				// todo 可能有问题
				vlBest += int(math.Floor(rand.Float64()*RANDOMNESS) - math.Floor(rand.Float64()*RANDOMNESS))
				if vlBest == s.Pos.DrawValue() {
					vlBest--
				}
			}
		}

	}
	s.SetBestMove(s.MvResult, depth)
	return vlBest
}

func (s *Search) SearchUnique(vlBeta, depth int) bool {
	sort := NewMoveSort(s.MvResult, s.Pos, s.KillerTable, s.HistoryTable)
	sort.Next()

	for {
		mv := sort.Next()
		if mv <= 0 {
			break
		}
		if !s.Pos.MakeMove(mv) {
			continue
		}

		newDepth := depth

		if !s.Pos.InCheck() {
			newDepth--
		}

		vl := -s.SearchFull(-vlBeta, 1-vlBeta, newDepth, false)

		s.Pos.UndoMakeMove()
		if vl >= vlBeta {
			return false
		}
	}

	return true
}

func (s *Search) SearchMain(depth, millis int) int {
	s.MvResult = s.Pos.BookMove()
	if s.MvResult > 0 {
		s.Pos.MakeMove(s.MvResult)
		if s.Pos.RepStatus(3) == 0 {
			s.Pos.UndoMakeMove()
			return s.MvResult
		}
		s.Pos.UndoMakeMove()
	}

	s.HashTable = []*HashTableObject{}
	for i := 0; i < s.HashMask+1; i++ {
		s.HashTable = append(s.HashTable, &HashTableObject{
			Depth: 0, Flag: 0, Vl: 0, Mv: 0, ZobristLock: 0,
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
	s.Pos.Distance = 0
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
