# 347. top_k_frequent_elements

**Link:** https://leetcode.com/problems/top-k-frequent-elements/
**Category:** arrays_hashing
**Độ khó:** 
**Ngày làm:** 2026-08-02

## Các cách đã nghĩ tới
1. Max-Heap (đẩy hết) — đếm tần suất bằng HashMap, đẩy toàn bộ `(count, val)` vào `BinaryHeap`, pop k lần lấy giá trị. O(n log n) thời gian.
2. Bucket Sort — tận dụng việc tần suất tối đa không vượt quá `n` (độ dài mảng), tạo `Vec<Vec<i32>>` kích thước `n+1`, bucket tại index `i` chứa các giá trị có tần suất đúng bằng `i`. Duyệt bucket từ cuối về đầu, gom đủ k giá trị. O(n) thời gian thực sự.
3. Min-Heap kích thước k — giữ heap chỉ tối đa k phần tử bằng `Reverse<(i32,i32)>`, khi tần suất mới cao hơn đỉnh heap (nhỏ nhất trong k) thì pop đỉnh rồi push mới. O(n log k).

## Vì sao chọn cách này
Bucket Sort đạt O(n) — nhanh nhất trong 3 cách, phù hợp khi biết trước giới hạn tần suất (không vượt quá độ dài mảng). Min-Heap (cách 3) hữu ích hơn khi `k` rất nhỏ so với `n`, không cần cấp phát bucket lớn.

## Pattern / insight rút ra
- BinaryHeap của Rust mặc định là max-heap — cần bọc `std::cmp::Reverse` để đảo thành min-heap.
- Bucket Sort tận dụng "giới hạn giá trị biết trước" (tần suất ∈ [0,n]) để thay thế việc sort/heap bằng index trực tiếp trong mảng — pattern quan trọng khi có ràng buộc rõ ràng về phạm vi giá trị.
- Heap giữ kích thước cố định k (thay vì đẩy hết n phần tử) giảm độ phức tạp từ O(n log n) xuống O(n log k).

## Lỗi đã mắc lúc làm
- Nhầm lẫn khái niệm "heap" (cấu trúc dữ liệu cây) với "heap" (vùng bộ nhớ động) — 2 khái niệm trùng tên nhưng không liên quan.
- Viết sai cú pháp bóc tuple trong Option: `Some(_, val)` thay vì `Some((_, val))` — Option chỉ chứa 1 giá trị (ở đây là 1 tuple), cần thêm 1 lớp ngoặc để bóc cả Option lẫn tuple bên trong cùng lúc.
- Quên dấu ngoặc gọi hàm: viết `heap.pop` thay vì `heap.pop()`.