#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# update_approach.sh — Cập nhật cách giải mới cho bài toán cũ
# ============================================================
# Cách dùng:
#   ./scripts/update_approach.sh <category_folder> <problem_slug> "<Tiêu đề bài>" <difficulty> "<Ghi chú approach mới>"
#
# Ví dụ:
#   ./scripts/update_approach.sh arrays_hashing two_sum "Two Sum" Easy "Sorting approach O(nlogn)"
# ============================================================

if [ "$#" -lt 5 ]; then
    echo "Thiếu tham số. Cách dùng:"
    echo "  ./scripts/update_approach.sh <category_folder> <problem_slug> \"<Tiêu đề>\" <Easy|Medium|Hard> \"<Ghi chú approach mới>\""
    exit 1
fi

CATEGORY="$1"
SLUG="$2"
TITLE="$3"
DIFFICULTY="$4"
NOTES="$5"

# Tùy thuộc vào cấu trúc dự án của bạn (đang để chung trong 1 thư mục src/ hay mỗi bài 1 thư mục riêng)
# Dựa theo code script trước của bạn, project dùng cấu trúc thư mục con hoặc file .rs
PROBLEM_PATH="src/${CATEGORY}/${SLUG}.rs"
if [ ! -f "$PROBLEM_PATH" ]; then
    PROBLEM_PATH="${CATEGORY}/${SLUG}" # Trường hợp cấu trúc cũ dạng workspace riêng
    if [ ! -d "$PROBLEM_PATH" ]; then
        echo "Lỗi: Không tìm thấy bài toán tại src/${CATEGORY}/${SLUG}.rs hoặc ${CATEGORY}/${SLUG}"
        exit 1
    fi
fi

TODAY=$(date +"%Y-%m-%d")

# ------------------------------------------------------------
# 1. Chạy test trước khi commit
# ------------------------------------------------------------
echo ">> Đang chạy cargo test để đảm bảo code mới viết không bị lỗi..."
if [ -d "${CATEGORY}/${SLUG}" ] && [ -f "${CATEGORY}/${SLUG}/Cargo.toml" ]; then
    (cd "${CATEGORY}/${SLUG}" && cargo test) || { echo "Test fail! Dừng commit."; exit 1; }
elif [ -f "Cargo.toml" ]; then
    cargo test || { echo "Test fail! Dừng commit."; exit 1; }
fi

# ------------------------------------------------------------
# 2. Cập nhật thêm dòng log vào README.md
# ------------------------------------------------------------
README="README.md"
if [ -f "$README" ]; then
    NEW_ROW="| ${TODAY} | [${TITLE} (Update Approach)](${CATEGORY}/${SLUG}) | ${DIFFICULTY} | ${CATEGORY} | ${NOTES} |"

    # Chèn dòng mới vào bảng lịch sử trong README
    awk -v newrow="$NEW_ROW" '
        { print }
        /^\|-+/ && !inserted { print newrow; inserted=1 }
    ' "$README" > "${README}.tmp" && mv "${README}.tmp" "$README"
    
    echo ">> Đã bổ sung lịch sử cập nhật vào README.md"
fi

# ------------------------------------------------------------
# 3. Git commit và push
# ------------------------------------------------------------
COMMIT_MSG="${TODAY}: update approach for ${TITLE} - ${NOTES}"

git add .
git commit -m "$COMMIT_MSG"
git push origin master

echo ">> Xong! Đã cập nhật cách giải mới, commit và push thành công."