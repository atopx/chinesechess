package main

import (
	"chessai/engine"
	"testing"
)

func Test_Search(t *testing.T) {
	fen := "9/2Cca4/3k1C3/4P1p2/4N1b2/4R1r2/4c1n2/3p1n3/2rNK4/9 w"
	eng := engine.NewPosition()
	eng.FromFen(fen)
	search := engine.NewSearch(eng, 16)
	mv := search.SearchMain(64, 1000)

	if mv != 26215 {
		t.Fatalf("error, excep: 26215: get %d", mv)
	}
	t.Logf("fen: %s, move: %s", fen, engine.Move2Iccs(mv))
}

func Test_ShellSort(t *testing.T) {
	mvs := []int{22599, 34697, 30615, 34713, 46758, 34728, 46760, 13749, 46773}
	vls := []int{29, 36, 26, 39, 28, 39, 29, 26, 26}
	engine.ShellSort(mvs, vls)
	expMvs := []int{34728, 34713, 34697, 22599, 46760, 46758, 30615, 13749, 46773}
	expVls := []int{39, 39, 36, 29, 29, 28, 26, 26, 26}
	for i := 0; i < 9; i++ {
		if expMvs[i] != mvs[i] {
			t.Fatalf("ShellSort error, \nexp mvs: %+v, \nget mvs: %+v\n", expMvs, mvs)
		}
		if vls[i] != expVls[i] {
			t.Fatalf("ShellSort error, \nexp vls: %+v, \nget vvs: %+v\n", expVls, vls)
		}
	}
}
