pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }

        let mut result: Vec<i32> = vec![];
        let mut left_half: Vec<i32> = Solution::gray_code(n-1);
        let mut right_half: Vec<i32> = left_half.iter().rev().map(|i| i + (1 << n-1)).collect();

        result.append(&mut left_half);
        result.append(&mut right_half);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1, 3, 2], Solution::gray_code(2));
        assert_eq!(vec![0,1], Solution::gray_code(1));
    }
}
