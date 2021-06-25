package number_match

func numMatchingSubseq(s string, words []string) int {
	m := make(map[rune][]int)
	for i, c := range s {
		if val, ok := m[c]; ok {
			m[c] = append(val, i)
		} else {
			m[c] = []int{i}
		}
	}

	ans := 0
	for _, word := range words {
		i := 0
		var c rune
		prevIdx := -1

		for i, c = range word {
			if idxs, ok := m[c]; ok {
				idxFound := false
				for _, idx := range idxs {
					if idx > prevIdx {
						prevIdx = idx
						idxFound = true
						break
					}
				}
				if !idxFound {
					i = 0
					break
				}
			} else {
				i = 0
				break
			}
		}

		if i == len(word)-1 {
			ans += 1
		}
	}
	return ans
}
