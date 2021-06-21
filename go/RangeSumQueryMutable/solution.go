package rsq

type NumArray struct {
	num_arr []int
}

func Constructor(nums []int) NumArray {
	val := NumArray{}
	val.num_arr = nums
	return val
}

func (this *NumArray) Update(index int, val int) {
	this.num_arr[index] = val
}

func (this *NumArray) SumRange(left int, right int) int {
	sum := 0
	for ; left <= right; left++ {
		sum += this.num_arr[left]
	}
	return sum
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * obj.Update(index,val);
 * param_2 := obj.SumRange(left,right);
 */
