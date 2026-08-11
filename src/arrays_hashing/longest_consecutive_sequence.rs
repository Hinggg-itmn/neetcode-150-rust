//! longest_consecutive_sequence
//! Time: O(n) | Space: O(n)
use std::collections::HashSet;

pub fn longest_consecutive_sequence(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut longest = 0;
    for &x in &num_set {
        if !num_set.contains(&(x - 1)) {
            let mut current_num = x;
            let mut current_steak = 1;
            while num_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_steak += 1;
            }
            longest = longest.max(current_steak);
        }
    }
    longest
}

pub fn longest_consecutive_sequence_v2(nums: Vec<i32>) -> i32 {
    if nums.is_empty(){
        return 0;
    }
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let mut longest= 1;
    let mut current =1;
    for i in 1..sorted.len(){
        if sorted[i]== sorted[i-1]+1{
            current +=1;
        }else {
            current = 1;
        }
        longest= longest.max(current);

    }
    longest
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let input = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive_sequence(input.clone()), 4);
        assert_eq!(longest_consecutive_sequence_v2(input), 4);
    }

    #[test]
    fn empty_array() {
        let input = vec![];
        assert_eq!(longest_consecutive_sequence(input.clone()), 0);
        assert_eq!(longest_consecutive_sequence_v2(input), 0);
    }

    #[test]
    fn with_duplicates() {
        let input = vec![1, 2, 0, 1];
        assert_eq!(longest_consecutive_sequence(input.clone()), 3);
        assert_eq!(longest_consecutive_sequence_v2(input), 3);
    }

    #[test]
    fn all_same_number() {
        let input = vec![5, 5, 5, 5];
        assert_eq!(longest_consecutive_sequence(input.clone()), 1);
        assert_eq!(longest_consecutive_sequence_v2(input), 1);
    }
}
