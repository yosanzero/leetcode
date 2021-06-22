pub struct Solution{}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut rows: Vec<Vec<i32>> = vec![];
        for i in 0..num_rows as usize {
            let mut row: Vec<i32> = vec![];
            for j in 0..i+1 {
                if j == 0 || j == i {
                    row.push(1)
                } else {
                    row.push(rows[i-1][j-1] + rows[i-1][j]);
                }
            }
            rows.push(row);
        }
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]], Solution::generate(5));
    }
}
