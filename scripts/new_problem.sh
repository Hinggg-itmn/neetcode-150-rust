#!/usr/bin/env bash
# scripts/new_problem.sh <category> <slug>
# Ví dụ: ./scripts/new_problem.sh arrays_hashing two-sum
set -e

if [ "$#" -lt 2 ]; then
    echo "Thiếu tham số. Cách dùng: ./scripts/new_problem.sh <category> <slug>"
    exit 1
fi

CATEGORY=$1
RAW_SLUG=$2

# Tự động thay thế mọi dấu gạch ngang (-) thành gạch dưới (_) cho Rust chuẩn cú pháp
SLUG=$(echo "$RAW_SLUG" | tr '-' '_')

mkdir -p "src/${CATEGORY}"
mkdir -p "notes"

# Kiểm tra trùng CẢ file code lẫn file note trước khi làm gì cả,
# tránh trường hợp chạy lại lỡ tay ghi đè mất note đã viết
if [ -f "src/${CATEGORY}/${SLUG}.rs" ]; then
    echo "❌ Lỗi: Bài '${SLUG}' đã tồn tại tại src/${CATEGORY}/${SLUG}.rs! Không ghi đè để tránh mất code."
    exit 1
fi
if [ -f "notes/${SLUG}.md" ]; then
    echo "❌ Lỗi: Note '${SLUG}' đã tồn tại tại notes/${SLUG}.md! Không ghi đè để tránh mất note."
    exit 1
fi

echo "Đang gọi API LeetCode để lấy thông tin bài '${RAW_SLUG}'..."

API_URL="https://leetcode.com/graphql"
QUERY='{"query":"query getQuestionDetail($titleSlug: String!) { question(titleSlug: $titleSlug) { questionId title translatedTitle } }","variables":{"titleSlug":"'"$RAW_SLUG"'"}}'

# "|| true" để nếu mạng lỗi / API sập, script KHÔNG bị set -e giết ngang,
# mà rơi xuống nhánh "không tìm thấy ID" bên dưới như thiết kế ban đầu
RESPONSE=$(curl -s -X POST -H "Content-Type: application/json" -d "$QUERY" "$API_URL" || true)
QUESTION_ID=$(echo "$RESPONSE" | grep -o '"questionId":"[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$QUESTION_ID" ]; then
    QUESTION_ID="?"
    echo "⚠️ Không tìm thấy ID tự động qua API, bạn có thể tự điền số vào file note."
else
    echo "✅ Đã tìm thấy mã bài: $QUESTION_ID"
fi

# 1. Tạo file mã nguồn .rs
cat > "src/${CATEGORY}/${SLUG}.rs" << RUST
//! $SLUG
//! Time: O(?) | Space: O(?)

pub fn ${SLUG}() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // TODO
    }
}
RUST

# 2. Đăng ký module vào mod.rs của category
if [ -f "src/${CATEGORY}/mod.rs" ]; then
    if ! grep -q "pub mod ${SLUG};" "src/${CATEGORY}/mod.rs"; then
        echo "pub mod ${SLUG};" >> "src/${CATEGORY}/mod.rs"
    fi
else
    echo "pub mod ${SLUG};" > "src/${CATEGORY}/mod.rs"
fi

# 3. Tạo file notes
cat > "notes/${SLUG}.md" << NOTES
# ${QUESTION_ID}. ${SLUG}

**Link:** https://leetcode.com/problems/${RAW_SLUG}/
**Category:** ${CATEGORY}
**Độ khó:** 
**Ngày làm:** $(date +%F)

## Các cách đã nghĩ tới
1. 

## Vì sao chọn cách này

## Pattern / insight rút ra

## Lỗi đã mắc lúc làm
NOTES

echo "🎉 Đã tạo xong src/${CATEGORY}/${SLUG}.rs và notes/${SLUG}.md"