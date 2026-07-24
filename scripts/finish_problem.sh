#!/usr/bin/env bash
# scripts/finish_problem.sh <num> <slug> <category> <"tên bài"> <độ khó> <cách giải> <"complexity">
set -e

NUM=$1
SLUG=$2
CATEGORY=$3
NAME=$4
DIFF=$5
APPROACH=$6
COMPLEXITY=$7

sed -i "/<!-- ROWS -->/a | $NUM | $(date +%F) | $NAME | $CATEGORY | $DIFF | $APPROACH | $COMPLEXITY |" README.md

git add .
git commit -m "day $NUM: $SLUG ($CATEGORY) - $APPROACH $COMPLEXITY"
git push

echo "Đã cập nhật README, commit và push xong bài $NAME."
