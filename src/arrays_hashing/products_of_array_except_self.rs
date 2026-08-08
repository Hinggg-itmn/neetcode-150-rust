//! products_of_array_except_self
//! Time: O(n) | Space: O(n)

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut prefix = vec![1; n];
    let mut suffix = vec![1; n];
    let mut result = vec![1; n];

    // Bước 1: tính prefix, duyệt trái sang phải, bắt đầu từ index 1
    for i in 1..n {
        prefix[i]=prefix[i-1]* nums[i-1]
    }

    // Bước 2: tính suffix, duyệt phải sang trái, bắt đầu từ index n-2
    for i in (0..n-1).rev() {
        suffix[i]=suffix[i+1]*nums[i+1]
    }

    // Bước 3: result[i] = prefix[i] * suffix[i]
    for i in 0..n
    {
        result[i]=suffix[i]*prefix[i]
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
}
}
