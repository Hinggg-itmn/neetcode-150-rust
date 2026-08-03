# 1. Two Sum

**Link:** https://leetcode.com/problems/two-sum/
**Category:** arrays_hashing
**Độ khó:** Easy
**Ngày làm:** 2026-08-03

## Các cách đã nghĩ tới
1. Brute force — 2 vòng lặp lồng nhau, so từng cặp `nums[i] + nums[j] == target`. O(n²) thời gian, O(1) không gian.
2. HashMap — duyệt 1 lần, với mỗi số tính `complement = target - num`, kiểm tra `complement` đã có trong map chưa (map lưu `giá_trị -> index`) trước khi ghi số hiện tại vào map. O(n) thời gian, O(n) không gian.
3. Sort + Two Pointers — sort mảng `(giá_trị, index_gốc)` theo giá trị, dùng 2 con trỏ từ 2 đầu tiến vào giữa. O(n log n) thời gian, O(n) không gian (cho mảng index).

## Vì sao chọn cách này
Chọn HashMap (cách 2) làm bản chính vì đạt O(n) — nhanh nhất về Big-O trong 3 cách. Cách 3 (sort + two pointers) là bản dự phòng khi cần tiết kiệm bộ nhớ hơn hoặc dữ liệu đã cache-friendly hơn do nằm liền mạch trong mảng (ít cache miss hơn HashMap dù Big-O kém hơn).

## Pattern / insight rút ra
- **Kiểm tra trước, ghi sau**: trong vòng lặp HashMap, phải tra `complement` trong map TRƯỚC khi ghi số hiện tại vào map — nếu đảo thứ tự, một số có thể tự khớp với chính nó.
- `entry()` không phải kiểu `Option` — nó là enum `Entry<K,V>` riêng (`Occupied`/`Vacant`), khác với `HashMap::get()` trả `Option`.
- `HashMap::insert()` trả về `bool` khi dùng qua `HashSet` — có thể tận dụng `!seen.insert(x)` để check trùng lặp trong 1 dòng, tránh tra cứu 2 lần.
- Đánh đổi Big-O lý thuyết vs hiệu năng thực tế: O(n) của HashMap có thể chậm hơn O(n log n) của mảng sort trong thực tế do cache locality — điểm sẽ gặp lại khi benchmark ETL pipeline.

## Lỗi đã mắc lúc làm
- Nhầm `seen.value`/`seen.key` — hiểu sai HashMap là 1 cặp biến đơn, thay vì 1 tập hợp nhiều cặp key-value cần tra cứu bằng `.get(&key)`.
- Chưa phân biệt được kiểu của `i` trong `for i in 0..n` là `usize` (không phải `i32`), vì `n = num.len()` trả `usize`.
- Viết `return arr[i,j]` — nhầm cú pháp index kiểu Python/NumPy, Rust không hỗ trợ index nhiều chiều bằng dấu phẩy.
- Ban đầu trả `Vec::new()` (rỗng) khi không tìm thấy nghiệm — nhận ra nên dùng `Option<Vec<i32>>` để phân biệt rõ "không tìm thấy" với "tìm thấy nhưng rỗng".