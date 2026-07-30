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
pub fn is_anagram_v2(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut count = [0i32; 26];
    for b in s.bytes() {
        // Bước 1: tăng count tại vị trí tương ứng với ký tự b
        count[(b - b'a') as usize] += 1;
    }
    for b in t.bytes() {
        // Bước 2: giảm count tại vị trí tương ứng với ký tự b
        count[(b - b'a') as usize] -= 1;
    }
    // Bước 3: kiểm tra toàn bộ count có phải toàn số 0 không
    count.iter().all(|&c| c == 0)
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