package main

import (
	"chessai/engine"
	"fmt"
)

func main() {
	eng := engine.NewEngine()
	search := engine.NewSearch(eng, 16)
	fen := "9/2Cca4/3k1C3/4P1p2/4N1b2/4R1r2/4c1n2/3p1n3/2rNK4/9 w"
	eng.FromFen(fen)
	mv := search.SearchMain(64, 1000)
	fmt.Println(engine.Move2Iccs(mv))
}
