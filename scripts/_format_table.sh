#!/usr/bin/env bash
# scripts/_format_table.sh <file.md>
# Căn lại tất cả bảng markdown trong file cho thẳng cột.
# Dùng chung cho finish_problem.sh và update_approach.sh
set -euo pipefail

FILE="${1:-README.md}"
[ -f "$FILE" ] || { echo "Không tìm thấy $FILE"; exit 1; }

# Ưu tiên gawk (có gensub), fallback awk thường
AWK_BIN="awk"
if command -v gawk >/dev/null 2>&1; then
    AWK_BIN="gawk"
fi

"$AWK_BIN" '
function repeat(ch, n,   s, i) {
    s = ""
    for (i = 0; i < n; i++) s = s ch
    return s
}

# Bắt đầu 1 bảng mới
function table_start() {
    in_table = 1
    nrows = 0
    for (i in width) delete width[i]
    for (i in rowlen) delete rowlen[i]
    for (i in cells) delete cells[i]
}

# Ghi nhận 1 dòng của bảng
function table_row(line,   m, i, c) {
    sub(/^\|/, "", line)
    sub(/\|[ \t]*$/, "", line)
    m = split(line, arr, "|")
    nrows++
    rowlen[nrows] = m
    for (i = 1; i <= m; i++) {
        c = arr[i]
        gsub(/^[ \t]+|[ \t]+$/, "", c)
        cells[nrows, i] = c
        if (length(c) > width[i]) width[i] = length(c)
    }
}

# In bảng ra
function table_flush(   r, c, val, pad) {
    for (r = 1; r <= nrows; r++) {
        printf "|"
        for (c = 1; c <= rowlen[r]; c++) {
            val = cells[r, c]
            # Dòng phân cách: --- hoặc :---: etc.
            if (val ~ /^:?-+:?$/) {
                pad = repeat("-", width[c])
                printf " %s |", pad
            } else {
                printf " %-*s |", width[c], val
            }
        }
        printf "\n"
    }
    in_table = 0
}

/^\|/ {
    if (!in_table) table_start()
    table_row($0)
    next
}

{
    if (in_table) table_flush()
    print
}

END {
    if (in_table) table_flush()
}
' "$FILE" > "$FILE.tmp" && mv "$FILE.tmp" "$FILE"

echo ">> Đã căn lại bảng trong $FILE"