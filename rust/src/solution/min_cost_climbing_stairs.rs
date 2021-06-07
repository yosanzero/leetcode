use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.len() <= 2 {
            return *cost.get(0).unwrap();
        }

        let mut cost_left = *cost.get(0).unwrap();
        let mut cost_right = *cost.get(1).unwrap();
        let mut index = 2;
        loop {
            if let Some(x) = cost.get(index) {
                let tmp = cost_right;
                cost_right = min(cost_left + x, cost_right + x);
                cost_left = tmp;
                index += 1;
            } else {
                return min(cost_left, cost_right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10,15,20]));
        assert_eq!(6, Solution::min_cost_climbing_stairs(vec![1,100,1,1,1,100,1,1,100,1]));
    }
}
