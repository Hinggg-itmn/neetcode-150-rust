//! two-sum
//! Time: O(?) | Space: O(?)
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&old_index) = seen.get(&complement) {
            return Some(vec![old_index as i32, i as i32]);
        }

        seen.insert(num, i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_pair_at_start() 
    {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, Some(vec![0, 1]));
    }
}

