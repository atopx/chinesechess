package main

import (
	"chessai/book"
	"fmt"
)

func main() {
	// fmt.Println("Iccs2move('a2-h3')47811 :", book.Iccs2Move("A2-H3"))
	// fmt.Println("Iccs2move('a0-i9')23523 :", book.Iccs2Move("A0-I9"))
	// fmt.Println("Iccs2move('a9-i0')60243 :", book.Iccs2Move("A9-I0"))

	// fmt.Println("Iccs2move('H9-I2'):", book.Move2Iccs(43834))
	// fmt.Println("Iccs2move('H0-G2') :", book.Move2Iccs(43466))
	// fmt.Println("Iccs2move('a9-i0'):", book.Move2Iccs(60243))
	// mis := 5000

	pos := book.NewPosition()
	pos.FromFen("rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1")
	fmt.Println(pos.Squares[53:210])
	search := book.NewSearch(pos, 32)
	mv := search.SearchMain(200, 5000)
	println(book.Move2Iccs(mv))
}
