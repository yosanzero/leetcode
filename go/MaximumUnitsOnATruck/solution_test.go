package truck

import "testing"

func TestMaximumUnits(t *testing.T) {
	if maximumUnits([][]int{{1, 3}, {2, 2}, {3, 1}}, 4) != 8 {
		t.Fatal("Example1")
	}
	if maximumUnits([][]int{{5, 10}, {2, 5}, {4, 7}, {3, 9}}, 10) != 91 {
		t.Fatal("Example2")
	}

}
