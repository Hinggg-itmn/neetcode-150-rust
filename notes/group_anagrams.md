# 49. group_anagrams

**Link:** https://leetcode.com/problems/group-anagrams/
**Category:** arrays_hashing
**Độ khó:** 
**Ngày làm:** 2026-07-30

## Các cách đã nghĩ tới
1. HashMap<String, Vec<String>> — với mỗi chuỗi, chuẩn hóa bằng cách sort ký tự (`Vec<char>` → `sort_unstable()` → gom lại thành `String` làm key), dùng `entry(key).or_insert(Vec::new()).push(s)` để gom nhóm.

## Vì sao chọn cách này
Chuỗi đã sort ký tự là "canonical form" — mọi anagram của nhau luôn cho ra cùng 1 key sau khi sort, nên gom nhóm bằng HashMap là cách tự nhiên và hiệu quả nhất (O(n · k log k), n = số từ, k = độ dài từ dài nhất).

## Pattern / insight rút ra
- `entry(key).or_insert(default).push(...)` — pattern "tra cứu 1 lần, xử lý cả 2 nhánh có/chưa có key" khi cần gom nhóm (group-by), tránh phải viết `if map.contains_key() {...} else {...}` tốn 2 lần tra cứu.
- `or_insert(Vec::new())` luôn tạo `Vec::new()` dù key đã tồn tại hay chưa (chỉ dùng khi Vacant) — Clippy khuyến nghị `or_insert_with(Vec::new)` để tránh cấp phát thừa khi không cần.

## Lỗi đã mắc lúc làm
Chưa quen đọc chuỗi gọi hàm nối tiếp nhiều dấu `.` (`map.entry(...).or_insert(...).push(...)`) — cần bóc từng bước: `entry()` trả `Entry`, `or_insert()` trả `&mut Vec<String>`, `push()` thao tác trực tiếp lên Vec đó trong map.