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

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# --- Escape dấu | để không phá vỡ bảng markdown ---
escape_pipe() { printf '%s' "$1" | sed 's/|/\\|/g'; }

TITLE_ESC=$(escape_pipe "$TITLE")
NOTES_ESC=$(escape_pipe "$NOTES")
COMPLEXITY_ESC=$(escape_pipe "$COMPLEXITY")

PROBLEM_FILE="src/${CATEGORY}/${SLUG}.rs"
if [ ! -f "$PROBLEM_FILE" ]; then
    echo "Lỗi: Không tìm thấy bài toán tại ${PROBLEM_FILE}"
    exit 1
fi

TODAY=$(date +"%Y-%m-%d")

# ------------------------------------------------------------
# 1. Chạy test trước khi commit — chỉ lọc test của đúng bài này
# ------------------------------------------------------------
echo ">> Đang chạy cargo test cho '${SLUG}'..."
cargo test "$SLUG" || { echo "Test fail! Dừng commit."; exit 1; }

# ------------------------------------------------------------
# 2. Cập nhật README.md — chèn ngay sau <!-- ROWS -->
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

NEW_ROW="| - | ${TODAY} | [${TITLE_ESC} (Update Approach)](notes/${SLUG}.md) | ${CATEGORY} | ${DIFFICULTY} | ${NOTES_ESC} | ${COMPLEXITY_ESC} |"

sed -i "/<!-- ROWS -->/a ${NEW_ROW}" "$README"

# --- Căn lại toàn bộ bảng cho RustRover hiển thị đẹp ---
"$SCRIPT_DIR/_format_table.sh" "$README"

echo ">> Đã bổ sung lịch sử cập nhật vào README.md"

# ------------------------------------------------------------
# 3. Git commit và push
# ------------------------------------------------------------
CURRENT_BRANCH=$(git branch --show-current)
COMMIT_MSG="${TODAY}: update approach for ${TITLE} - ${NOTES}"

git add .
git commit -m "$COMMIT_MSG"
git push origin "$CURRENT_BRANCH"

echo ">> Xong! Đã cập nhật cách giải mới cho ${TITLE}, commit và push lên nhánh ${CURRENT_BRANCH}."