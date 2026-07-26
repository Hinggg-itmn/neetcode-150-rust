#!/usr/bin/env bash
# scripts/new_problem.sh <category> <slug>
# Ví dụ: ./scripts/new_problem.sh arrays_hashing two-sum


CATEGORY=$1
RAW_SLUG=$2

# Tự động thay thế mọi dấu gạch ngang (-) thành gạch dưới (_) cho Rust chuẩn cú pháp
SLUG=$(echo "$RAW_SLUG" | tr '-' '_')

# Đảm bảo thư mục tồn tại
mkdir -p "src/${CATEGORY}"
mkdir -p "notes"

echo "Đang gọi API LeetCode để lấy thông tin bài '${RAW_SLUG}'..."

# Gọi API công khai của LeetCode dùng slug gốc có dấu (-)
API_URL="https://leetcode.com/graphql"
QUERY='{"query":"query getQuestionDetail($titleSlug: String!) { question(titleSlug: $titleSlug) { questionId title translatedTitle } }","variables":{"titleSlug":"'"$RAW_SLUG"'"}}'

RESPONSE=$(curl -s -X POST -H "Content-Type: application/json" -d "$QUERY" "$API_URL")
QUESTION_ID=$(echo "$RESPONSE" | grep -o '"questionId":"[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$QUESTION_ID" ]; then
    QUESTION_ID="?"
    echo "⚠️ Không tìm thấy ID tự động qua API, bạn có thể tự điền số vào file note."
else
    echo "✅ Đã tìm thấy mã bài: $QUESTION_ID"
fi

# Kiểm tra nếu file .rs đã tồn tại thì báo lỗi và dừng lại
if [ -f "src/${CATEGORY}/${SLUG}.rs" ]; then
    echo "❌ Lỗi: Bài '${SLUG}' đã tồn tại trong thư mục ${CATEGORY}! Không ghi đè để tránh mất code."
    exit 1
fi
# 1. Tạo file mã nguồn .rs (Dùng tên đã đổi dấu _ chuẩn Rust)
cat > "src/${CATEGORY}/${SLUG}.rs" << EOF
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
EOF

# 2. Đăng ký module vào mod.rs của category
if [ -f "src/${CATEGORY}/mod.rs" ]; then
    if ! grep -q "pub mod ${SLUG};" "src/${CATEGORY}/mod.rs"; then
        echo "pub mod ${SLUG};" >> "src/${CATEGORY}/mod.rs"
    fi
else
    echo "pub mod ${SLUG};" > "src/${CATEGORY}/mod.rs"
fi

# 3. Tạo file notes (Dùng $RAW_SLUG cho link LeetCode, $SLUG cho tiêu đề file)
cat > "notes/${SLUG}.md" << EOF
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
EOF

echo "🎉 Đã tạo xong src/${CATEGORY}/${SLUG}.rs và notes/${SLUG}.md"