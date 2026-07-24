//! LeetCode 217: Contains Duplicate
//! https://leetcode.com/problems/contains-duplicate/
//!
//! Approach: HashSet — insert từng phần tử, nếu insert trả về false
//! nghĩa là giá trị đã tồn tại.
//! Time: O(n) | Space: O(n)

use std::collections::HashSet;

pub fn contains_duplicate(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &num in nums {
        if !seen.insert(num) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate() {
        assert_eq!(contains_duplicate(&[1, 2, 3, 1]), true);
    }

    #[test]
    fn test_no_duplicate() {
        assert_eq!(contains_duplicate(&[1, 2, 3, 4]), false);
    }

    #[test]
    fn test_empty() {
        assert_eq!(contains_duplicate(&[]), false);
    }
}