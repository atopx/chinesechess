package engine

import (
	"encoding/binary"
	"fmt"
)

func IN_BOARD(sq int) bool {
	return IN_BOARD_[sq] != 0
}

func IN_FORT(sq int) bool {
	return IN_FORT_[sq] != 0
}

func RANK_Y(sq int) int {
	return sq >> 4
}

func FILE_X(sq int) int {
	return sq & 15
}

func COORD_XY(x, y int) int {
	return x + (y << 4)
}

func SQUARE_FLIP(sq int) int {
	return 254 - sq

}
func FILE_FLIP(x int) int {
	return 14 - x
}

func MIRROR_SQUARE(sq int) int {
	return COORD_XY(FILE_FLIP(FILE_X(sq)), RANK_Y(sq))
}

func SQUARE_FORWARD(sq, sd int) int {
	return sq - 16 + (sd << 5)
}

func KING_SPAN(sqSrc, sqDst int) bool {
	return LEGAL_SPAN[sqDst-sqSrc+256] == 1
}

func ADVISOR_SPAN(sqSrc, sqDst int) bool {
	return LEGAL_SPAN[sqDst-sqSrc+256] == 2
}

func BISHOP_SPAN(sqSrc, sqDst int) bool {
	return LEGAL_SPAN[sqDst-sqSrc+256] == 3
}

func BISHOP_PIN(sqSrc, sqDst int) int {
	return (sqSrc + sqDst) >> 1
}

func KNIGHT_PIN(sqSrc, sqDst int) int {
	return sqSrc + KNIGHT_SPIN[sqDst-sqSrc+256]
}

func HOME_HALF(sq, sd int) bool {
	return (sq & 0x80) != (sd << 7)
}

func AWAY_HALF(sq, sd int) bool {
	return (sq & 0x80) == (sd << 7)
}

func SAME_HALF(sqSrc, sqDst int) bool {
	return ((sqSrc ^ sqDst) & 0x80) == 0
}

func SAME_RANK(sqSrc, sqDst int) bool {
	return ((sqSrc ^ sqDst) & 0xf0) == 0
}

func SAME_FILE(sqSrc, sqDst int) bool {
	return ((sqSrc ^ sqDst) & 0x0f) == 0
}

func SIDE_TAG(sd int) int {
	return 8 + (sd << 3)
}

func OPP_SIDE_TAG(sd int) int {
	return 16 - (sd << 3)
}

func SRC(mv int) int {
	return mv & 255
}

func DST(mv int) int {
	return mv >> 8

}
func MOVE(sqSrc, sqDst int) int {
	return sqSrc + (sqDst << 8)
}

func MIRROR_MOVE(mv int) int {
	return MOVE(MIRROR_SQUARE(SRC(mv)), MIRROR_SQUARE(DST(mv)))
}
func MVV_LVA(pc, lva int) int {
	return MVV_VALUE[pc&7] - lva
}

func CHR(n int) string {
	return string(rune(n))
}

func ASC(c rune) int {
	return int(c)
}

const FEN_PIECE = "        KABNRCP kabnrcp "

func CHAR_TO_PIECE(c byte) int {
	switch c {
	case 'K':
		return PIECE_KING
	case 'A':
		return PIECE_ADVISOR
	case 'B':
		return PIECE_BISHOP
	case 'E':
		return PIECE_BISHOP
	case 'H':
		return PIECE_KNIGHT
	case 'N':
		return PIECE_KNIGHT
	case 'R':
		return PIECE_ROOK
	case 'C':
		return PIECE_CANNON
	case 'P':
		return PIECE_PAWN
	default:
		return -1
	}
}

func UnsignedRightShift(x int, y int) int {
	x = x & 0xffffffff
	b := make([]byte, 4)
	binary.BigEndian.PutUint32(b, uint32(x))
	x = int(binary.BigEndian.Uint32(b))
	return x >> (y & 0xf)
}

func Cord2uint8(cord string) int {
	alphabet := cord[0] - 'A' + FILE_LEFT
	numeric := '9' - cord[1] + RANK_TOP
	return int(numeric<<4 + alphabet)
}

func Iccs2Move(iccs string) int {
	src := Cord2uint8(iccs[:2])
	dst := Cord2uint8(iccs[3:])
	return (dst << 8) + src
}

func Move2Iccs(mv int) string {
	src := SRC(mv)
	dst := DST(mv)
	return fmt.Sprintf("%s%s-%s%s",
		string(rune('A'+FILE_X(src)-FILE_LEFT)),
		string(rune('9'-RANK_Y(src)+RANK_TOP)),
		string(rune('A'+FILE_X(dst)-FILE_LEFT)),
		string(rune('9'-RANK_Y(dst)+RANK_TOP)),
	)
}
