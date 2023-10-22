package main

import (
	"chessai/book"
	"fmt"
	"math/rand"
	"testing"
)

func Test_Search(t *testing.T) {
	pos := book.NewPosition()
	pos.FromFen("9/2Cca4/3k1C3/4P1p2/4N1b2/4R1r2/4c1n2/3p1n3/2rNK4/9 w")
	search := book.NewSearch(pos, 16)
	mov := search.SearchMain(64, 1000)
	t.Logf("excep: 26215: get %d", mov)
}

func Test_ShellSort(t *testing.T) {
	d := []int{}
	c := []int{}
	for i := 0; i < 1000; i++ {
		v := rand.Intn(1000)
		d = append(d, v)
		c = append(c, v)
	}
	book.ShellSort(d, c)
	for i := 0; i < 999; i++ {
		if !(c[i+1] <= c[i]) {
			fmt.Println(c[i], c[i+1])
			panic(1)
		}
		if c[i] != d[i] {
			panic(2)
		}
	}
}
