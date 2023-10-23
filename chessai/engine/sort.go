package engine

var SHELL_STEP = [8]int{0, 1, 4, 13, 40, 121, 364, 1093}

func ShellSort(mvs []int, vls []int) {
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
