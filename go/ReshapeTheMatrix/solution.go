package matrixreshape

func matrixReshape(mat [][]int, r int, c int) [][]int {

	if len(mat) == 0 || r*c != len(mat)*len(mat[0]) {
		return mat
	}

	res := make([][]int, r)
	cols := len(mat[0])
	var cRow, cCol int
	for i := 0; i < r; i++ {
		row := make([]int, c)
		for j := 0; j < c; j++ {
			row[j] = mat[cRow][cCol]
			cCol++
			if cCol >= cols {
				cCol = 0
				cRow++
			}
		}
		res[i] = row
	}
	return res

}
