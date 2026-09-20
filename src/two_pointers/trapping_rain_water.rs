//! trapping_rain_water
//! Time: O(n) cho cả 3 bản (trap là O(n^2) do 2 vòng lặp lồng) | Space: v1=O(1), v2=O(n), v3=O(1)

pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    }
    let mut total_water = 0;
    for i in 0..n {
        let mut max_left = 0;
        for j in 0..=i {
            max_left = max_left.max(height[j]);
        }
        let mut max_right = 0;
        for j in i..n {
            max_right = max_right.max(height[j]);
        }
        let current_water = std::cmp::min(max_left, max_right) - height[i];
        total_water += current_water;
    }
    total_water
}

pub fn trap_v2(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    }
    let mut total_water = 0;
    let mut right_max = vec![0; n];
    let mut left_max = vec![0; n];
    left_max[0] = height[0];
    for i in 1..n {
        left_max[i] = std::cmp::max(left_max[i - 1], height[i]);
    }
    right_max[n - 1] = height[n - 1];
    if n > 1 {
        for i in (0..n - 1).rev() {
            right_max[i] = std::cmp::max(right_max[i + 1], height[i]);
        }
    }
    for i in 0..n {
        total_water += std::cmp::min(left_max[i], right_max[i]) - height[i];
    }
    total_water
}

pub fn trap_v3(height: Vec<i32>) -> i32 {
    let mut total_water = 0;
    let n = height.len();
    if n == 0 {
        return 0;
    }
    let mut right = n - 1;
    let mut left = 0;
    let mut max_right = 0;
    let mut max_left = 0;
    while left < right {
        if height[left] < height[right] {
            if height[left] >= max_left {
                max_left = height[left];
            } else {
                total_water += max_left - height[left];
            }
            left += 1;
        } else {
            if height[right] >= max_right {
                max_right = height[right];
            } else {
                total_water += max_right - height[right];
            }
            right -= 1;
        }
    }
    total_water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brute_force_basic() {
        assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn v2_basic() {
        assert_eq!(trap_v2(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn v3_basic() {
        assert_eq!(trap_v3(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn three_versions_agree_on_your_example() {
        let case = vec![0,1,0,3,1,0,1,3,3];
        assert_eq!(trap(case.clone()), trap_v2(case.clone()));
        assert_eq!(trap_v2(case.clone()), trap_v3(case));
    }

    #[test]
    fn no_water_cases() {
        assert_eq!(trap(vec![]), 0);
        assert_eq!(trap_v2(vec![1]), 0);
        assert_eq!(trap_v3(vec![1,2,3,4]), 0); // dốc tăng dần, không đọng nước
    }
}
