package main

import (
	"chessai/engine"
	"fmt"
)

func main() {
	eng := engine.NewEngine()
	eng.FromFen("rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1")
	search := engine.NewSearch(eng, 32)
	mv := search.SearchMain(200, 5000)
	fmt.Println(engine.Move2Iccs(mv))
}
