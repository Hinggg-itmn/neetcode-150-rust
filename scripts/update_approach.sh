#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# update_approach.sh — Cập nhật cách giải mới cho bài toán cũ
# ============================================================
# Cách dùng:
#   ./scripts/update_approach.sh <category> <slug> "<Tiêu đề bài>" <difficulty> "<approach mới>" "<complexity mới>"
#
# Ví dụ:
#   ./scripts/update_approach.sh arrays_hashing two_sum "Two Sum" Easy "Sorting approach" "O(nlogn)/O(1)"
# ============================================================

if [ "$#" -lt 6 ]; then
    echo "Thiếu tham số. Cách dùng:"
    echo "  ./scripts/update_approach.sh <category> <slug> \"<Tiêu đề>\" <Easy|Medium|Hard> \"<approach mới>\" \"<complexity mới>\""
    exit 1
fi

CATEGORY="$1"
SLUG="$2"
TITLE="$3"
DIFFICULTY="$4"
NOTES="$5"
COMPLEXITY="$6"

PROBLEM_FILE="src/${CATEGORY}/${SLUG}.rs"
if [ ! -f "$PROBLEM_FILE" ]; then
    echo "Lỗi: Không tìm thấy bài toán tại ${PROBLEM_FILE}"
    exit 1
fi

TODAY=$(date +"%Y-%m-%d")

# ------------------------------------------------------------
# 1. Chạy test trước khi commit — chỉ lọc test của đúng bài này
#    (dự án dùng 1 crate chung theo src/<category>/<slug>.rs,
#    không phải mỗi bài 1 workspace riêng)
# ------------------------------------------------------------
echo ">> Đang chạy cargo test cho '${SLUG}'..."
cargo test "$SLUG" || { echo "Test fail! Dừng commit."; exit 1; }

# ------------------------------------------------------------
# 2. Cập nhật README.md — chèn ngay sau <!-- ROWS -->,
#    CÙNG một vị trí với finish_problem.sh, để 2 loại script
#    không còn ghi vào 2 chỗ khác nhau trong bảng (bug cũ)
# ------------------------------------------------------------
README="README.md"
if [ ! -f "$README" ]; then
    echo "Lỗi: Không tìm thấy README.md"
    exit 1
fi
if ! grep -q "<!-- ROWS -->" "$README"; then
    echo "Lỗi: Không tìm thấy marker <!-- ROWS --> trong README.md"
    exit 1
fi

# Link trỏ về đúng file note thật (notes/<slug>.md) thay vì đường dẫn
# không tồn tại như trước (<category>/<slug>)
NEW_ROW="| - | ${TODAY} | [${TITLE} (Update Approach)](notes/${SLUG}.md) | ${CATEGORY} | ${DIFFICULTY} | ${NOTES} | ${COMPLEXITY} |"

sed -i "/<!-- ROWS -->/a ${NEW_ROW}" "$README"
echo ">> Đã bổ sung lịch sử cập nhật vào README.md"

# ------------------------------------------------------------
# 3. Git commit và push — tự lấy tên nhánh hiện tại thay vì
#    hardcode "master" (tránh lỗi nếu bạn dùng "main")
# ------------------------------------------------------------
CURRENT_BRANCH=$(git branch --show-current)
COMMIT_MSG="${TODAY}: update approach for ${TITLE} - ${NOTES}"

git add .
git commit -m "$COMMIT_MSG"
git push origin "$CURRENT_BRANCH"

echo ">> Xong! Đã cập nhật cách giải mới cho ${TITLE}, commit và push lên nhánh ${CURRENT_BRANCH}."