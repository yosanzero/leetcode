pub struct Solution {}

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let total_len: i32 = matchsticks.iter().sum();
        if total_len % 4 != 0 {
            return false;
        }

        let target_len = total_len / 4;

        let mut sorted_matchsticks = matchsticks.clone();
        sorted_matchsticks.sort_by(|a, b| b.cmp(a));

        return Solution::can_arrange(&matchsticks, 0, target_len, [0, 0, 0, 0]);
    }

    fn can_arrange(matchsticks: &Vec<i32>, cursor: usize, target_len: i32, mut edges: [i32; 4]) -> bool {
        if matchsticks.len() <= cursor {
            for edge in edges.iter() {
                if *edge != target_len {
                    return false;
                }
            }
            return true;
        }

        for i in 0..4 {
            let matchstick = matchsticks[cursor];
            if edges[i] + matchstick <= target_len {
                edges[i] += matchstick;
                if Solution::can_arrange(matchsticks, cursor + 1, target_len, edges) {
                    return true;
                } else {
                    edges[i] -= matchstick;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::makesquare(vec![5,5,5,5,4,4,4,4,3,3,3,3]));
        assert_eq!(true, Solution::makesquare(vec![1,1,2,2,2]));
        assert_eq!(false, Solution::makesquare(vec![3,3,3,3,4]));
    }
}
