//! container_with_most_water
//! Time: O(N) | Space: O(1)

pub fn max_area(heights: Vec<i32>) -> i32 {
    if heights.len() < 2 {
        return 0;
    }

    let mut max_water = 0;
    let mut left = 0;
    let mut right = heights.len() - 1;

    while left < right {
        let h = std::cmp::min(heights[left], heights[right]);
        let w = (right - left) as i32;
        let current_water = h * w;
        
        max_water = std::cmp::max(current_water, max_water);

        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_with_most_water() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
        assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(max_area(vec![1, 2, 1]), 2);
    }
}