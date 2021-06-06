func longestConsecutive(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	sort.Ints(nums)

	length := 1
	counter := nums[0]
	max_val := 1
	for _, val := range nums {
		if val == counter {
			continue
		}
		if val == (counter + 1) {
			length += 1
			counter += 1
		} else {
			counter = val
			max_val = max(max_val, length)
			length = 1
		}
	}
	return max(max_val, length)
}

func max(a int, b int) int {
	if a > b {
		return a
	}
	return b
}
