package engine

import (
	"chessai/book"
	"fmt"
	"math"
	"math/rand"
	"strings"
)

type Engine struct {
	SdPlayer    int
	ZobRistKey  int
	ZobRistLock int
	VlWhite     int
	Distance    int
	VlBlack     int
	MvList      []int
	PcList      []int
	KeyList     []int
	ChkList     []bool
	Squares     [256]int
}

func NewEngine() *Engine {
	return new(Engine)
}

func (p *Engine) ClearBoard() {
	p.SdPlayer = 0
	p.ZobRistKey = 0
	p.ZobRistLock = 0
	p.VlWhite = 0
	p.VlBlack = 0
	p.Squares = [256]int{}

}

func (p *Engine) SetIrrev() {
	p.Distance = 0
	p.MvList = []int{0}
	p.PcList = []int{0}
	p.KeyList = []int{0}
	p.ChkList = []bool{p.Checked()}
	fmt.Println("SetIrrev", p.Checked())
}

func (p *Engine) Checked() bool {

	pcSelfSide := SIDE_TAG(p.SdPlayer)
	pcOppSide := OPP_SIDE_TAG(p.SdPlayer)
	fmt.Printf("checked 1 self_side %d opp_side %d player %d\n", pcSelfSide, pcOppSide, p.SdPlayer)
	for sqSrc := 0; sqSrc < 256; sqSrc++ {
		// self_side 8, opp_side 16, player 0
		if p.Squares[sqSrc] != (pcSelfSide + PIECE_KING) {
			continue
		}

		//fmt.Printf("checked 2 sq_src %d sd_player %d oppside %d PIECE_PAWN: %d\n", sqSrc, p.SdPlayer, pcOppSide, PIECE_PAWN)
		if p.Squares[SQUARE_FORWARD(sqSrc, p.SdPlayer)] == (pcOppSide + PIECE_PAWN) {
			return true
		}

		if p.Squares[sqSrc-1] == (pcOppSide + PIECE_PAWN) {
			fmt.Println("checked 3")
			return true
		}

		if p.Squares[sqSrc+1] == (pcOppSide + PIECE_PAWN) {
			fmt.Println("checked 4")
			return true
		}

		for i := 0; i < 4; i++ {
			if p.Squares[sqSrc+ADVISOR_DELTA[i]] != 0 {
				continue
			}

			for j := 0; j < 2; j++ {
				pcDst := p.Squares[sqSrc+KNIGHT_CHECK_DELTA[i][j]]
				if pcDst == (pcOppSide + PIECE_KNIGHT) {
					fmt.Println("checked 5")
					return true
				}
			}
		}

		for i := 0; i < 4; i++ {
			delta := KING_DELTA[i]
			sqDst := sqSrc + delta
			for IN_BOARD(sqDst) {
				pcDst := p.Squares[sqDst]
				if pcDst > 0 {
					if pcDst == (pcOppSide+PIECE_ROOK) || pcDst == (pcOppSide+PIECE_KING) {
						fmt.Println("checked 6")
						return true
					}
					break
				}
				sqDst += delta
			}
			sqDst += delta
			for IN_BOARD(sqDst) {
				pcDst := p.Squares[sqDst]
				if pcDst > 0 {
					if pcDst == (pcOppSide + PIECE_CANNON) {
						fmt.Println("checked 7")
						return true
					}
					break
				}
				sqDst += delta
			}
		}
		return false
	}
	return false
}

func (p *Engine) MateValue() int {
	return p.Distance - MATE_VALUE
}

func (p *Engine) BanValue() int {
	return p.Distance - BAN_VALUE
}

func (p *Engine) DrawValue() int {
	if p.Distance&1 == 0 {
		return -DRAW_VALUE
	}
	return DRAW_VALUE
}

func (p *Engine) Evaluate() int {
	var vl int
	if p.SdPlayer == 0 {
		vl = p.VlWhite - p.VlBlack
	} else {
		vl = p.VlBlack - p.VlWhite
	}
	vl += ADVANCED_VALUE
	if vl == p.DrawValue() {
		return vl - 1
	}
	return vl
}

func (p *Engine) NullOkay() bool {
	var vl int
	if p.SdPlayer == 0 {
		vl = p.VlWhite
	} else {
		vl = p.VlBlack
	}
	return vl > NULL_OKAY_MARGIN
}

func (p *Engine) NullSafe() bool {
	var vl int
	if p.SdPlayer == 0 {
		vl = p.VlWhite
	} else {
		vl = p.VlBlack
	}
	return vl > NULL_SAFE_MARGIN
}

func (p *Engine) InCheck() bool {
	return p.ChkList[len(p.ChkList)-1]
}

func (p *Engine) Captured() bool {
	return p.PcList[len(p.PcList)-1] > 0
}

func (p *Engine) RepValue(vlRep int) int {
	var vl int
	if vlRep&2 != 0 {
		vl = p.BanValue()
	}
	if vlRep&4 != 0 {
		vl -= p.BanValue()
	}
	if vl == 0 {
		return p.DrawValue()
	}
	return vl
}

func (p *Engine) RepStatus(recur int) (status int) {
	selfSide := false
	perpCheck := true
	oppPerpCheck := true
	index := len(p.MvList) - 1
	for p.MvList[index] > 0 && p.PcList[index] == 0 {
		if selfSide {
			perpCheck = perpCheck && p.ChkList[index]
			if p.KeyList[index] == p.ZobRistKey {
				recur--
				if recur == 0 {
					if perpCheck {
						status += 2
					}
					if oppPerpCheck {
						status += 4
					}
					return status + 1
				}
			}
		} else {
			oppPerpCheck = oppPerpCheck && p.ChkList[index]
		}
		selfSide = !selfSide
		index--
	}
	return status
}

func (p *Engine) Mirror() *Engine {
	mirror := NewEngine()
	mirror.ClearBoard()
	for i := 0; i < len(mirror.Squares); i++ {
		pc := p.Squares[i]
		if pc > 0 {
			mirror.AddPiece(MIRROR_SQUARE(i), pc, ADD_PIECE)
		}
	}
	if p.SdPlayer == 1 {
		mirror.ChangeSide()
	}
	return mirror
}

func (p *Engine) BookMove() int {
	var mirror bool
	lock := UnsignedRightShift(p.ZobRistLock, 1)
	index := book.BinarySearch(lock)
	if index < 0 {
		mirror = true
		lock = UnsignedRightShift(p.Mirror().ZobRistLock, 1)
		index = book.BinarySearch(lock)
		if index < 0 {
			return 0
		}
	}
	index--
	for index >= 0 && book.DATA[index][0] == lock {
		index--
	}
	mvs := make([]int, 0)
	vls := make([]int, 0)
	var value int
	index += 1
	for index < len(book.DATA) && book.DATA[index][0] == lock {
		mv := book.DATA[index][1]
		if mirror {
			mv = MIRROR_MOVE(mv)
		}
		if p.LegalMove(mv) {
			mvs = append(mvs, mv)
			vl := book.DATA[index][2]
			vls = append(vls, vl)
			value += vl
		}
		index += 1
	}
	if value == 0 {
		return 0
	}
	value = int(math.Floor(rand.Float64() * float64(value)))
	for index = 0; index < len(mvs); index++ {
		value -= vls[index]
		if value < 0 {
			break
		}
	}
	return mvs[index]
}

func (p *Engine) NullMove() {
	p.MvList = append(p.MvList, 0)
	p.PcList = append(p.PcList, 0)
	p.KeyList = append(p.KeyList, p.ZobRistKey)
	p.ChangeSide()
	p.ChkList = append(p.ChkList, false)
	p.Distance++
}

func (p *Engine) UndoNullMove() {
	p.Distance--
	p.ChkList = p.ChkList[:len(p.ChkList)-1]
	p.ChangeSide()
	p.KeyList = p.KeyList[:len(p.KeyList)-1]
	p.PcList = p.PcList[:len(p.PcList)-1]
	p.MvList = p.MvList[:len(p.MvList)-1]
}

func (p *Engine) HistoryIndex(mv int) int {
	return ((p.Squares[SRC(mv)] - 8) << 8) + DST(mv)
}

func (p *Engine) Winner() int {
	if p.IsMate() {
		return 1 - p.SdPlayer
	}
	pc := SIDE_TAG(p.SdPlayer) + PIECE_KING
	sqMate := 0
	for i := 0; i < 256; i++ {
		if p.Squares[i] == pc {
			sqMate = i
			break
		}
	}
	if sqMate == 0 {
		return 1 - p.SdPlayer
	}
	vlRep := p.RepStatus(3)
	if vlRep > 0 {
		vlRep = p.RepValue(vlRep)
		if -WIN_VALUE < vlRep && vlRep < WIN_VALUE {
			return 2
		} else {
			return p.SdPlayer
		}
	}
	hasMaterial := false
	for i := 0; i < 256; i++ {
		if IN_BOARD(i) && (p.Squares[i]&7) > 2 {
			hasMaterial = true
			break
		}
	}
	if !hasMaterial {
		// 无进攻子力做和
		return 2
	}
	return 0
}

func (p *Engine) LegalMove(mv int) bool {
	sqSrc := SRC(mv)
	pcSrc := p.Squares[sqSrc]

	pcSelfSide := SIDE_TAG(p.SdPlayer)
	if (pcSrc & pcSelfSide) == 0 {
		return false
	}
	sqDst := DST(mv)
	pcDst := p.Squares[sqDst]
	if (pcDst & pcSelfSide) != 0 {
		return false
	}

	switch pcSrc - pcSelfSide {
	case PIECE_KING:
		return IN_FORT(sqDst) && KING_SPAN(sqSrc, sqDst)
	case PIECE_ADVISOR:
		return IN_FORT(sqDst) && ADVISOR_SPAN(sqSrc, sqDst)
	case PIECE_BISHOP:
		return SAME_HALF(sqSrc, sqDst) && BISHOP_SPAN(sqSrc, sqDst) && (p.Squares[BISHOP_PIN(sqSrc, sqDst)] == 0)
	case PIECE_KNIGHT:
		sqPin := KNIGHT_PIN(sqSrc, sqDst)
		return sqPin != sqSrc && p.Squares[sqPin] == 0
	case PIECE_ROOK, PIECE_CANNON:
		var delta int
		if SAME_RANK(sqSrc, sqDst) {
			if sqDst < sqSrc {
				delta = -1
			} else {
				delta = 1
			}
		} else if SAME_FILE(sqSrc, sqDst) {
			if sqDst < sqSrc {
				delta = -16
			} else {
				delta = 16
			}
		} else {
			return false
		}
		sqPin := sqSrc + delta
		for sqPin != sqDst && p.Squares[sqPin] == 0 {
			sqPin += delta
		}
		if sqPin == sqDst {
			return pcDst == 0 || (pcSrc-pcSelfSide == PIECE_ROOK)
		}
		if pcDst == 0 || (pcSrc-pcSelfSide != PIECE_CANNON) {
			return false
		}
		sqPin += delta
		for sqPin != sqDst && p.Squares[sqPin] == 0 {
			sqPin += delta
		}
		return sqPin == sqDst

	case PIECE_PAWN:
		if AWAY_HALF(sqDst, p.SdPlayer) && (sqDst == sqSrc-1 || sqDst == sqSrc+1) {
			return true
		}
		return sqDst == SQUARE_FORWARD(sqSrc, p.SdPlayer)
	}
	return false
}

func (p *Engine) IsMate() bool {
	mvs := p.GenerateMoves(nil)
	for _, v := range mvs {
		if p.MakeMove(v) {
			p.UndoMakeMove()
			return false
		}
	}
	return true
}

func (p *Engine) UndoMakeMove() {
	p.Distance -= 1
	p.ChkList = p.ChkList[:len(p.ChkList)-1]
	p.ChangeSide()
	p.KeyList = p.KeyList[:len(p.KeyList)-1]
	p.UndoMovePiece()
}

func (p *Engine) GenerateMoves(vls *[]int) (mvs []int) {
	fmt.Println("generate_mvs 1")
	pcSelfSide := SIDE_TAG(p.SdPlayer)
	pcOppSide := OPP_SIDE_TAG(p.SdPlayer)
	fmt.Println("generate_mvs 2")
	for sqSrc := 0; sqSrc < 256; sqSrc++ {
		pcSrc := p.Squares[sqSrc]
		if (pcSrc & pcSelfSide) == 0 {
			fmt.Println("generate_mvs 2 -- 1")
			continue
		}
		fmt.Println("generate_mvs 2 -- 2")
		switch pcSrc - pcSelfSide {
		case PIECE_KING:
			fmt.Println("generate_mvs 2 -- 3")
			for i := 0; i < 4; i++ {
				sqDst := sqSrc + KING_DELTA[i]
				fmt.Printf("generate_mvs 2 -- 3 -- %d sq_dst=%d sq_src=%d\n", i, sqDst, sqSrc)
				if !IN_FORT(sqDst) {
					continue
				}

				pcDst := p.Squares[sqDst]

				if vls == nil {
					if (pcDst & pcSelfSide) == 0 {
						mvs = append(mvs, MOVE(sqSrc, sqDst))
						fmt.Println("push PIECE_KING 1 mv", mvs[len(mvs)-1])
					}
				} else if (pcDst & pcOppSide) != 0 {
					mvs = append(mvs, MOVE(sqSrc, sqDst))
					fmt.Println("push PIECE_KING 2 mv", MOVE(sqSrc, sqDst))
					*vls = append(*vls, MVV_LVA(pcDst, 5))
					fmt.Println("push PIECE_KING 2 vls", (*vls)[len(*vls)-1])
				}
			}

		case PIECE_ADVISOR:
			fmt.Println("generate_mvs 2 -- 4")
			for i := 0; i < 4; i++ {
				sqDst := sqSrc + ADVISOR_DELTA[i]
				fmt.Printf("generate_mvs 2 -- 4 -- %d sq_dst=%d sq_src=%d\n", i, sqDst, sqSrc)
				if !IN_FORT(sqDst) {
					continue
				}
				pcDst := p.Squares[sqDst]
				if vls == nil {
					if (pcDst & pcSelfSide) == 0 {
						mvs = append(mvs, MOVE(sqSrc, sqDst))
						fmt.Println("push PIECE_ADVISOR 1 mv", mvs[len(mvs)-1])
					}
				} else if (pcDst & pcOppSide) != 0 {
					mvs = append(mvs, MOVE(sqSrc, sqDst))
					fmt.Println("push PIECE_ADVISOR 2 mv", mvs[len(mvs)-1])
					*vls = append(*vls, MVV_LVA(pcDst, 1))
					fmt.Println("push PIECE_ADVISOR 2 vls", (*vls)[len(*vls)-1])
				}
			}

		case PIECE_BISHOP:
			fmt.Println("generate_mvs 2 -- 5")
			for i := 0; i < 4; i++ {
				sqDst := sqSrc + ADVISOR_DELTA[i]
				fmt.Printf("generate_mvs 2 -- 5 -- %d sq_dst=%d sq_src=%d\n", i, sqDst, sqSrc)
				if !(IN_BOARD(sqDst) && HOME_HALF(sqDst, p.SdPlayer) && p.Squares[sqDst] == 0) {
					continue
				}
				sqDst += ADVISOR_DELTA[i]
				pcDst := p.Squares[sqDst]
				if vls == nil {
					if (pcDst & pcSelfSide) == 0 {
						mvs = append(mvs, MOVE(sqSrc, sqDst))
						fmt.Println("push PIECE_BISHOP 1 mv", mvs[len(mvs)-1])
					}
				} else if (pcDst & pcOppSide) != 0 {
					mvs = append(mvs, MOVE(sqSrc, sqDst))
					fmt.Println("push PIECE_BISHOP 2 mv", mvs[len(mvs)-1])
					*vls = append(*vls, MVV_LVA(pcDst, 1))
					fmt.Println("push PIECE_BISHOP 2 vls", (*vls)[len(*vls)-1])
				}
			}

		case PIECE_KNIGHT:
			fmt.Println("generate_mvs 2 -- 6")
			for i := 0; i < 4; i++ {
				sqDst := sqSrc + KING_DELTA[i]
				fmt.Printf("generate_mvs 2 -- 6 -- %d sq_dst=%d sq_src=%d\n", i, sqDst, sqSrc)
				if p.Squares[sqDst] > 0 {
					continue
				}
				for j := 0; j < 2; j++ {
					sqDst := sqSrc + KNIGHT_DELTA[i][j]
					if !IN_BOARD(sqDst) {
						continue
					}
					pcDst := p.Squares[sqDst]
					if vls == nil {
						if (pcDst & pcSelfSide) == 0 {
							mvs = append(mvs, MOVE(sqSrc, sqDst))
							fmt.Println("push PIECE_KNIGHT 1 mv", mvs[len(mvs)-1])
						}
					} else if (pcDst & pcOppSide) != 0 {
						mvs = append(mvs, MOVE(sqSrc, sqDst))
						fmt.Println("push PIECE_KNIGHT 2 mv", mvs[len(mvs)-1])
						*vls = append(*vls, MVV_LVA(pcDst, 1))
						fmt.Println("push PIECE_KNIGHT 2 vls", (*vls)[len(*vls)-1])
					}
				}
			}

		case PIECE_ROOK:
			fmt.Println("generate_mvs 2 -- 7")
			for i := 0; i < 4; i++ {
				delta := KING_DELTA[i]
				sqDst := sqSrc + delta
				fmt.Printf("generate_mvs 2 -- 7 -- %d delta=%d sq_dst=%d sq_src=%d\n", i, delta, sqDst, sqSrc)
				for IN_BOARD(sqDst) {
					pcDst := p.Squares[sqDst]
					if pcDst == 0 {
						if vls == nil {
							mvs = append(mvs, MOVE(sqSrc, sqDst))
							fmt.Println("push PIECE_ROOK 1 mv", mvs[len(mvs)-1])
						}
					} else {
						if (pcDst & pcOppSide) != 0 {
							mvs = append(mvs, MOVE(sqSrc, sqDst))
							fmt.Println("push PIECE_ROOK 2 mv", mvs[len(mvs)-1])
							if vls != nil {
								*vls = append(*vls, MVV_LVA(pcDst, 4))
								fmt.Println("push PIECE_ROOK 2 vls", (*vls)[len(*vls)-1])
							}
						}
						break
					}
					sqDst += delta
				}
			}

		case PIECE_CANNON:
			fmt.Println("generate_mvs 2 -- 8")
			for i := 0; i < 4; i++ {
				delta := KING_DELTA[i]
				sqDst := sqSrc + delta
				fmt.Printf("generate_mvs 2 -- 8 -- %d delta=%d sq_dst=%d sq_src=%d\n", i, delta, sqDst, sqSrc)
				for IN_BOARD(sqDst) {
					pcDst := p.Squares[sqDst]
					//if delta == -1 && sqDst == 52 && sqSrc == 53 {
					//	fmt.Println(p.Squares)
					//	fmt.Println(pcDst)
					//	os.Exit(0)
					//}
					if pcDst == 0 {
						if vls == nil {
							mvs = append(mvs, MOVE(sqSrc, sqDst))
							fmt.Println("push PIECE_CANNON 1 mv", mvs[len(mvs)-1])
						}
					} else {
						break
					}
					sqDst += delta
				}
				sqDst += delta

				for IN_BOARD(sqDst) {
					pcDst := p.Squares[sqDst]
					if pcDst > 0 {
						if (pcDst & pcOppSide) != 0 {
							mvs = append(mvs, MOVE(sqSrc, sqDst))
							fmt.Println("push PIECE_CANNON 2 mv", mvs[len(mvs)-1])
							if vls != nil {
								*vls = append(*vls, MVV_LVA(pcDst, 4))
								fmt.Println("push PIECE_CANNON 2 vls", (*vls)[len(*vls)-1])
							}
						}
						break
					}
					sqDst += delta
				}
			}

		case PIECE_PAWN:
			fmt.Println("generate_mvs 2 -- 9")
			sqDst := SQUARE_FORWARD(sqSrc, p.SdPlayer)
			if IN_BOARD(sqDst) {
				pcDst := p.Squares[sqDst]
				fmt.Printf("generate_mvs 2 -- 9 -- sq_dst=%d sq_src=%d\n", sqDst, sqSrc)
				if vls == nil {
					if (pcDst & pcSelfSide) == 0 {
						mvs = append(mvs, MOVE(sqSrc, sqDst))
						fmt.Println("push PIECE_PAWN 1 mv", mvs[len(mvs)-1])
					}
				} else if (pcDst & pcOppSide) != 0 {
					mvs = append(mvs, MOVE(sqSrc, sqDst))
					fmt.Println("push PIECE_PAWN 2 mv", mvs[len(mvs)-1])
					*vls = append(*vls, MVV_LVA(pcDst, 4))
					fmt.Println("push PIECE_PAWN 2 vls", (*vls)[len(*vls)-1])
				}
			}
			if AWAY_HALF(sqSrc, p.SdPlayer) {
				for _, delta := range []int{-1, 1} {
					sqDst = sqSrc + delta
					if IN_BOARD(sqDst) {
						pcDst := p.Squares[sqDst]
						if vls == nil {
							if (pcDst & pcSelfSide) == 0 {
								mvs = append(mvs, MOVE(sqSrc, sqDst))
								fmt.Println("push PIECE_PAWN 1 mv", mvs[len(mvs)-1])
							}
						} else {
							mvs = append(mvs, MOVE(sqSrc, sqDst))
							fmt.Println("push PIECE_PAWN 2 mv", mvs[len(mvs)-1])
							*vls = append(*vls, MVV_LVA(pcDst, 4))
							fmt.Println("push PIECE_PAWN 2 vls", (*vls)[len(*vls)-1])
						}
					}
				}
			}

		}
	}
	fmt.Println("generate_mvs 3")
	return mvs
}

func (p *Engine) MovePiece(mv int) {
	sqSrc := SRC(mv)
	sqDst := DST(mv)
	pc := p.Squares[sqDst]
	p.PcList = append(p.PcList, pc)
	if pc > 0 {
		p.AddPiece(sqDst, pc, DEL_PIECE)
	}
	pc = p.Squares[sqSrc]
	p.AddPiece(sqSrc, pc, DEL_PIECE)
	p.AddPiece(sqDst, pc, ADD_PIECE)
	p.MvList = append(p.MvList, mv)
}

func (p *Engine) UndoMovePiece() {
	fmt.Println("undo_move_piece 1")
	mv := p.MvList[len(p.MvList)-1]
	p.MvList = p.MvList[:len(p.MvList)-1]
	sqSrc := SRC(mv)
	sqDst := DST(mv)
	pc := p.Squares[sqDst]
	fmt.Printf("undo_move_piece 2 -- mv %d sq_src %d sq_dst %d pc_dst %d\n", mv, sqSrc, sqDst, pc)
	p.AddPiece(sqDst, pc, DEL_PIECE)
	p.AddPiece(sqSrc, pc, ADD_PIECE)
	pc = p.PcList[len(p.PcList)-1]
	p.PcList = p.PcList[:len(p.PcList)-1]
	if pc > 0 {
		p.AddPiece(sqDst, pc, ADD_PIECE)
	}
}

func (p *Engine) MakeMove(mv int) bool {
	fmt.Println("make_move 1 mv", mv)
	p.MovePiece(mv)
	fmt.Println("make_move 2")
	if p.Checked() {
		fmt.Println("make_move 2 - 1 checked")
		p.UndoMovePiece()
		return false
	}
	fmt.Println("make_move 3")
	p.KeyList = append(p.KeyList, p.ZobRistKey)
	p.ChangeSide()
	p.ChkList = append(p.ChkList, p.Checked())
	p.Distance++
	fmt.Printf("make_move 4 distance %d, push check %v\n", p.Distance, p.Checked())
	return true
}

func (p *Engine) AddPiece(sq, pc int, deleted bool) {
	if deleted {
		fmt.Printf("add_piece 1 sq %d pc %d action DEL\n", sq, pc)
		p.Squares[sq] = 0
	} else {
		fmt.Printf("add_piece 1 sq %d pc %d action ADD\n", sq, pc)
		p.Squares[sq] = pc
	}

	var pcAdjust int
	if pc < 16 {
		pcAdjust = pc - 8
		score := PIECE_VALUE[pcAdjust][sq]
		if deleted {
			p.VlWhite -= score
		} else {
			p.VlWhite += score
		}
	} else {
		pcAdjust = pc - 16
		score := PIECE_VALUE[pcAdjust][SQUARE_FLIP(sq)]
		if deleted {
			p.VlBlack -= score
		} else {
			p.VlBlack += score
		}
		pcAdjust += 7
	}

	p.ZobRistKey ^= PreGenZobRistKeyTable[pcAdjust][sq]
	p.ZobRistLock ^= PreGenZobRistLockTable[pcAdjust][sq]
}

func (p *Engine) FromFen(fen string) {
	p.ClearBoard()
	x := FILE_LEFT
	y := RANK_TOP
	index := 0

	if len(fen) == index {
		p.SetIrrev()
		return
	}

	c := fen[index]
	for c != ' ' {
		if c == '/' {
			x = FILE_LEFT
			y += 1

			if y > RANK_BOTTOM {
				break
			}
		} else if c >= '1' && c <= '9' {
			x += ASC(rune(c)) - 48 // 48 = run('0')
		} else if c >= 'A' && c <= 'Z' {
			if x <= FILE_RIGHT {
				pt := CHAR_TO_PIECE(c)
				if pt >= 0 {
					p.AddPiece(COORD_XY(x, y), pt+8, ADD_PIECE) // # thistodo
				}
				x++
			}
		} else if c >= 'a' && c <= 'z' {
			if x <= FILE_RIGHT {
				pt := CHAR_TO_PIECE(c + 'A' - 'a')
				if pt >= 0 {
					p.AddPiece(COORD_XY(x, y), pt+16, ADD_PIECE)
				}
				x++
			}
		}
		index++
		if index == len(fen) {
			p.SetIrrev()
			return
		}
		c = fen[index]
	}
	index++
	if index == len(fen) {
		p.SetIrrev()
		return
	}
	var player int
	if fen[index] == 'b' {
		player = 0
	} else {
		player = 1
	}
	if p.SdPlayer == player {
		p.ChangeSide()
	}
	p.SetIrrev()
}

func (p *Engine) ToFen() (fen string) {
	var chars []string
	for y := RANK_TOP; y < RANK_BOTTOM+1; y++ {
		k := 0
		row := ""
		for x := FILE_LEFT; x < FILE_RIGHT+1; x++ {
			pc := p.Squares[COORD_XY(x, y)]
			if pc > 0 {
				if k > 0 {
					row += string(rune('0' + k))
					k = 0
				}
				row += string(FEN_PIECE[pc])
			} else {
				k++
			}
		}
		if k > 0 {
			row += string(rune('0' + k))
		}
		chars = append(chars, row)
	}
	fen = strings.Join(chars, "/")
	if p.SdPlayer == 0 {
		fen += " w"
	} else {
		fen += " b"
	}
	return fen
}

func (p *Engine) ChangeSide() {
	p.SdPlayer = 1 - p.SdPlayer
	p.ZobRistKey ^= PreGenZobRistKeyPlayer
	p.ZobRistLock ^= PreGenZobRistLockPlayer
}

func (p *Engine) ShellSort(mvs []int, vls []int) {
	stepLevel := 1
	for SHELL_STEP[stepLevel] < len(mvs) {
		stepLevel++
	}
	stepLevel--
	for stepLevel > 0 {
		step := SHELL_STEP[stepLevel]
		for i := 0; i < len(mvs); i++ {
			mvBest := mvs[i]
			vlBest := vls[i]
			j := i - step
			for j >= 0 && vlBest > vls[j] {
				mvs[j+step] = mvs[j]
				vls[j+step] = vls[j]
				j -= step
			}
			mvs[j+step] = mvBest
			vls[j+step] = vlBest
		}
		stepLevel--
	}
}
