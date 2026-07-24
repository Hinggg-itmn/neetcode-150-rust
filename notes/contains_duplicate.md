# 217. Contains Duplicate

**Link:** https://leetcode.com/problems/contains-duplicate/
**Category:** Arrays & Hashing
**Độ khó:** Easy
**Ngày làm:** 2026-07-24

## Các cách đã nghĩ tới
1. Brute force — so từng cặp — O(n²) / O(1)
2. Sort trước, so liền kề — O(n log n) / O(1)
3. HashSet — O(n) / O(n)   ← đã chọn cách này

## Vì sao chọn cách 3
Đánh đổi thời gian lấy bộ nhớ hợp lý vì input không quá lớn về mặt bộ nhớ,
và đây là dạng bài "kiểm tra trùng lặp" sẽ gặp lại nhiều lần.

## Pattern / insight rút ra
"Cần kiểm tra đã-thấy-chưa" → luôn nghĩ tới HashSet trước tiên.

## Lỗi đã mắc lúc làm
(để trống nếu không có, hoặc ghi lại bug cụ thể — ví dụ quên early-return)