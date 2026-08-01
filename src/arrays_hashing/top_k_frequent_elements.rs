//! top_k_frequent_elements
//! Time: O(N log N) | Space: O(N)
use std::collections::{HashMap, BinaryHeap};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq: HashMap<i32, i32> = HashMap::new();

    // Bước 1: đếm tần suất
    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    // Bước 2: đẩy (tần_suất, giá_trị) vào BinaryHeap
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    for (val, count) in freq {
        heap.push((count, val)); // Sửa: Thêm dấu đóng ngoặc đơn )
    }

    // Bước 3: pop k lần, lấy giá trị (phần tử thứ 2 của tuple)
    let mut result = Vec::new();
    for _ in 0..k {
        if let Some((_, val)) = heap.pop() { // Sửa: Thêm cặp ngoặc đơn cho tuple và hàm pop()
            result.push(val);
        }
    }

    result
} // Sửa: Xóa bỏ 1 dấu } thừa ở đây

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}
