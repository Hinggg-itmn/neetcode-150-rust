# 271. encode_and_decode_strings

**Link:** https://leetcode.com/problems/encode-and-decode-strings/
**Category:** arrays_hashing
**Độ khó:** 
**Ngày làm:** 2026-08-03

## Các cách đã nghĩ tới
1. Dùng độ dài chuỗi gắn thêm một ký tự phân tách với dữ liệu chuỗi như trong bài là len#string và nếu bạn lo nếu chuỗi đó cũng có kí hiệu # thì trong vòng lệnh while cắt là ở vị trí đầu tiên nó nhìn thấy từ trái và từ dữ liệu len thu thấp trước # đầu tiên đó thì đã suy ra toàn bộ string rồi

## Vì sao chọn cách này
Thuật toán phân tách hợp lý bằng byte 
## Pattern / insight rút ra
Thêm kiến thức mới về Chunked, làm quen với việc chuyển từ kích thước động -> byte
## Lỗi đã mắc lúc làm
Chưa quen sử dụng wrap(), borrow rule của rust và thư viện của rust đa số chưa quen syntax