pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(last_char) = stack.last() {
                if *last_char == c {
                    stack.remove(stack.len() - 1);
                    continue;
                }
            }
            stack.push(c);
        }
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("ca".to_string(), Solution::remove_duplicates("abbaca".to_string()));
    }
}
