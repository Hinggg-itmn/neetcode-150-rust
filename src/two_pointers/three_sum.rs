//! three_sum
//! Time: O(n^2) | Space: O(n) (cho việc sort, không tính output)

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut result = Vec::new();
    let n = nums.len();

    for i in 0..n {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        if nums[i] > 0 {
            break;
        }

        let (mut left, mut right) = (i + 1, n - 1);
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_result(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for triplet in &mut v {
            triplet.sort();
        }
        v.sort();
        v
    }

    #[test]
    fn basic_case() {
        let result = sort_result(three_sum(vec![-1, 0, 1, 2, -1, -4]));
        let expected = sort_result(vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn no_triplet() {
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn all_zeros() {
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn empty_and_short() {
        assert_eq!(three_sum(vec![]), Vec::<Vec<i32>>::new());
        assert_eq!(three_sum(vec![1, 2]), Vec::<Vec<i32>>::new());
    }
}