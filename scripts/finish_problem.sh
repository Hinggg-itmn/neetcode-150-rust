#!/usr/bin/env bash
# scripts/finish_problem.sh <slug> <category> <"tên bài"> <độ khó> <cách giải> <"complexity">
# Ví dụ:
#   ./scripts/finish_problem.sh two_sum arrays_hashing "Two Sum" Easy HashMap "O(n)/O(n)"
set -e

SLUG=$1
CATEGORY=$2
NAME=$3
DIFF=$4
APPROACH=$5
COMPLEXITY=$6

README="README.md"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# --- Escape dấu | để không phá vỡ bảng markdown ---
escape_pipe() { printf '%s' "$1" | sed 's/|/\\|/g'; }

NAME_ESC=$(escape_pipe "$NAME")
APPROACH_ESC=$(escape_pipe "$APPROACH")
COMPLEXITY_ESC=$(escape_pipe "$COMPLEXITY")

# Tự động tính số thứ tự dựa trên số dòng bài đã có trong bảng (sau <!-- ROWS -->)
NUM=$(awk '
    /<!-- ROWS -->/ { found=1; next }
    found && /^\| *[0-9]+ *\|/ { count++ }
    END { print count + 1 }
' "$README")

# Chèn dòng mới ngay sau marker <!-- ROWS -->
sed -i "/<!-- ROWS -->/a | $NUM | $(date +%F) | $NAME_ESC | $CATEGORY | $DIFF | $APPROACH_ESC | $COMPLEXITY_ESC |" "$README"

# --- Căn lại toàn bộ bảng cho RustRover hiển thị đẹp ---
"$SCRIPT_DIR/_format_table.sh" "$README"

git add .
git commit -m "day $NUM: $SLUG ($CATEGORY) - $APPROACH $COMPLEXITY"
git push

echo "Đã cập nhật README, commit và push xong bài $NAME (day $NUM)."