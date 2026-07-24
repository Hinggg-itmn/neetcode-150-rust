#!/usr/bin/env bash
# scripts/new_problem.sh <category> <slug>

CATEGORY=$1
SLUG=$2

# Đảm bảo thư mục đích tồn tại
mkdir -p "src/${CATEGORY}"
mkdir -p "notes"

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

# Kiểm tra xem mod.rs đã có dòng pub mod chưa để tránh bị lặp
if [ -f "src/${CATEGORY}/mod.rs" ]; then
    if ! grep -q "pub mod ${SLUG};" "src/${CATEGORY}/mod.rs"; then
        echo "pub mod ${SLUG};" >> "src/${CATEGORY}/mod.rs"
    fi
else
    echo "pub mod ${SLUG};" > "src/${CATEGORY}/mod.rs"
fi

cat > "notes/${SLUG}.md" << EOF
# ${SLUG}

**Link:**
**Category:** ${CATEGORY}
**Độ khó:**
**Ngày làm:** $(date +%F)

## Các cách đã nghĩ tới

## Vì sao chọn cách này

## Pattern / insight rút ra

## Lỗi đã mắc lúc làm
EOF

echo "Đã tạo src/${CATEGORY}/${SLUG}.rs và notes/${SLUG}.md"
