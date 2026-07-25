//! LeetCode 242: Valid Anagram
//! https://leetcode.com/problems/valid-anagram/
//!
//! Approach: Sorting — chuẩn hóa cả 2 chuỗi bằng sort rồi so sánh.
//! Time: O(n log n) | Space: O(n)

pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    s_chars.sort_unstable();
    t_chars.sort_unstable();
    s_chars == t_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram() {
        assert_eq!(is_anagram("anagram", "nagaram"), true);
    }

    #[test]
    fn test_not_anagram() {
        assert_eq!(is_anagram("rat", "car"), false);
    }
}