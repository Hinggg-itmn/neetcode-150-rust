# NeetCode 150 - Rust

## Cách dùng script

### 1. Bắt đầu bài mới
```bash
./scripts/new_problem.sh <category> <slug-co-gach-ngang>
```
Tự tạo `src/<category>/<slug>.rs` (todo!()), đăng ký vào `mod.rs`, và tạo `notes/<slug>.md`. Gọi API LeetCode để tự điền số bài (nếu mạng lỗi thì điền `?`, không sao). Sẽ báo lỗi nếu bài đã tồn tại (không ghi đè).

```bash
# ví dụ
./scripts/new_problem.sh arrays_hashing two-sum
```

### 2. Làm xong bài → ghi vào README + commit + push
```bash
./scripts/finish_problem.sh <slug_gach_duoi> <category> "<Tên bài>" <Easy|Medium|Hard> <cách_giải> "<complexity>"
```
Tự tính số thứ tự (đếm số bài đã có, không cần tự gõ), thêm 1 dòng vào bảng bên dưới, commit và push luôn.

```bash
# ví dụ
./scripts/finish_problem.sh two_sum arrays_hashing "Two Sum" Easy HashMap "O(n)/O(n)"
```

### 3. Nghĩ ra cách giải mới cho bài cũ
```bash
./scripts/update_approach.sh <category> <slug_gach_duoi> "<Tên bài>" <Easy|Medium|Hard> "<approach mới>" "<complexity mới>"
```
Chạy `cargo test` cho đúng bài đó trước, nếu pass mới thêm dòng "(Update Approach)" vào README, commit và push (tự lấy nhánh git hiện tại).

```bash
# ví dụ
./scripts/update_approach.sh arrays_hashing two_sum "Two Sum" Easy "Sorting approach" "O(nlogn)/O(1)"
```

> Lưu ý: `slug` trong `new_problem.sh` dùng dấu gạch ngang (đúng URL LeetCode, vd `two-sum`), còn `finish_problem.sh`/`update_approach.sh` dùng gạch dưới (đúng tên file Rust, vd `two_sum`) — script `new_problem.sh` tự động đổi `-` thành `_` khi tạo file.

---

## Lịch sử làm bài

| # | Ngày | Bài | Category | Độ khó | Cách giải | Complexity |
|---|------|-----|----------|--------|-----------|------------|
<!-- ROWS -->
| 6 | 2026-08-04 | Encode AndDecode String | arrays_hashing | Medium | Chunked | Transfer |
| - | 2026-08-03 | [Top K Frequent Elements (Update Approach)](notes/top_k_frequent_elements.md) | arrays_hashing | Medium | Bucket Sort approach | O(n)/O(n) |
| 5 | 2026-08-02 | Top K Frequency Elements | top-k-frequent-elements | Medium | BinaryHeap | O(n)/O(n) |
| 4 | 2026-07-30 | Group_Anagrams | arrays_hashing | Medium | Sorting | O(n)/O(nlogn) |
| - | 2026-07-28 | [Valid Anagram (Update Approach)](notes/valid_anagram.md) | arrays_hashing | Easy | Frequency Array | O(n)/O(1) |
| 1 | 2026-07-26 | Two Sum | arrays_hashing | Easy | HashMap | O(n)/O(n) |
| 2 | 2026-07-24 | Contains Duplicate | arrays_hashing | Easy | HashSet | O(n)/O(n) |
| 3 | 2026-07-25 | Valid Anagram | arrays_hashing | Easy | Sorting | O(nlogn)/O(n) |