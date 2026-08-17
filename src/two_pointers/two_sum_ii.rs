//! two_sum_II
//! Time: O(n) | Space: O(1)

pub fn two_sum_ii(num: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, num.len() - 1);
    while left < right {
        let sum = num[left] + num[right];
        if sum == target {
            return vec![(left + 1) as i32, (right + 1) as i32];
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    vec![]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(two_sum_ii(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn answer_at_ends() {
        assert_eq!(two_sum_ii(vec![1, 2, 3, 4, 5], 6), vec![1, 5]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(two_sum_ii(vec![-3, -1, 0, 2, 4], 1), vec![2, 4]);
    }

    #[test]
    fn two_elements_only() {
        assert_eq!(two_sum_ii(vec![1, 3], 4), vec![1, 2]);
    }
}