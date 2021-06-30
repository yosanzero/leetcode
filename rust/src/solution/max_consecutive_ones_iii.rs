use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut flipped_queue = VecDeque::new();

        let mut cursor_left = 0;
        let mut max_consecutive = 0;

        for (index, value) in nums.iter().enumerate() {
            if *value == 1 || flipped_queue.len() < k as usize {
                if *value != 1 {
                    flipped_queue.push_back(index);
                }
            } else { // value != 0 && flipped > k
                if let Some(left_index) = flipped_queue.pop_front() {
                    flipped_queue.push_back(index);
                    cursor_left = left_index + 1;
                } else {
                    cursor_left = index + 1;
                }
            }

            let cursor_right = index + 1;

            if max_consecutive <= (cursor_right - cursor_left) {
                max_consecutive = cursor_right - cursor_left;
            }
        }
        max_consecutive as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, Solution::longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2));
        assert_eq!(10, Solution::longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3));
    }
}
