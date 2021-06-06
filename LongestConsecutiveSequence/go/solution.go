func longestConsecutive(nums []int) int {
    if len(nums) == 0 {
        return 0
    }
    
    sort.Ints(nums)
    
    m := make(map[int]bool)
    uniq := [] int{}

    for _, ele := range nums {
        if !m[ele] {
            m[ele] = true
            uniq = append(uniq, ele)
        }
    }
    
    length := 1
    counter := uniq[0]
    max_val := 1
    for _, val := range uniq[1:]{
        if val == (counter + 1) {
            length += 1
            counter += 1
        }else{
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
