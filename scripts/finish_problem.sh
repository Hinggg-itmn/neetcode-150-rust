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

# Tự động tính số thứ tự dựa trên số dòng bài đã có trong bảng (sau <!-- ROWS -->)
# -> không cần tự gõ tay, nên không còn lo gõ nhầm/quên tăng số
NUM=$(awk '
    /<!-- ROWS -->/ { found=1; next }
    found && /^\| *[0-9]+ *\|/ { count++ }
    END { print count + 1 }
' "$README")

sed -i "/<!-- ROWS -->/a | $NUM | $(date +%F) | $NAME | $CATEGORY | $DIFF | $APPROACH | $COMPLEXITY |" "$README"

git add .
git commit -m "day $NUM: $SLUG ($CATEGORY) - $APPROACH $COMPLEXITY"
git push

echo "Đã cập nhật README, commit và push xong bài $NAME (day $NUM)."