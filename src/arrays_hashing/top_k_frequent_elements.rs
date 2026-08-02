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
}

pub fn top_k_frequent_bucket(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    // Bước 2: tạo buckets, mỗi bucket là Vec<i32> chứa các giá trị có cùng tần suất
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
    // Điền: duyệt freq, đẩy val vào đúng buckets[count]
    for (num,count) in freq
    {
	    buckets[count as usize].push(num);
	}
    // Bước 3: duyệt buckets từ cuối (index n) về đầu, gom đủ k giá trị
    let mut result = Vec::new();
    for buckets in buckets.into_iter().rev()
    {
	    for num in buckets
	    {
		    result.push(num);
		    if result.len() ==k as usize{
			    return result
			}
		}
	}
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
    #[test]
    fn test_top_k_frequent_bucket() {
        assert_eq!(top_k_frequent_bucket(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(top_k_frequent_bucket(vec![1], 1), vec![1]);
    }
}
