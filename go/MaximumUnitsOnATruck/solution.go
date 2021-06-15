package truck

import "sort"

func maximumUnits(boxTypes [][]int, truckSize int) int {
	sort.SliceStable(boxTypes, func(i, j int) bool {
		return boxTypes[i][1] > boxTypes[j][1]
	})
	ans := 0
	for _, box := range boxTypes {
		if truckSize > box[0] {
			ans += box[0] * box[1]
			truckSize -= box[0]
		} else {
			ans += truckSize * box[1]
			break
		}
	}

	return ans
}
