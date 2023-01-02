use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let half = arr.len() / 2;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;
        for num in arr {
            let count = *counter.entry(num).or_insert(0) + 1;
            counter.insert(num, count);

            if count > max_count {
                max_count = count;
            }
            if count as usize >= half {
                break;
            }
        }
        let mut ordered: Vec<(&i32, &i32)> = counter.iter().collect();
        ordered.sort_by(|a, b| (*b.1).cmp(a.1));
        let mut size = 0;
        let mut count = 0;
        for t in ordered {
            count += t.1;
            size += 1;
            if count as usize >= half {
                break;
            }
        }
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::min_set_size(vec![3,3,3,3,5,5,5,2,2,7]));
    }
}
