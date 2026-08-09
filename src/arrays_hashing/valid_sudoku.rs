//! valid_sudoku
//! Time: O(1) | Space: O(1)
use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: [HashSet<char>; 9] = std::array::from_fn(|_| HashSet::new());
    let mut cols: [HashSet<char>; 9] = std::array::from_fn(|_| HashSet::new());
    let mut boxes: [HashSet<char>; 9] = std::array::from_fn(|_| HashSet::new());

    for row in 0..9 {
        for col in 0..9 {
            let val = board[row][col];

            if val == '.' {
                continue;
            }

            let box_index = (row / 3) * 3 + (col / 3);

            if rows[row].contains(&val) || cols[col].contains(&val) || boxes[box_index].contains(&val) {
                return false;
            }

            rows[row].insert(val);
            cols[col].insert(val);
            boxes[box_index].insert(val);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_board(rows: [&str; 9]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn valid_board_passes() {
        let board = to_board([
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(is_valid_sudoku(board));
    }

    #[test]
    fn duplicate_in_row_fails() {
        let board = to_board([
            "553.7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!is_valid_sudoku(board));
    }

    #[test]
    fn duplicate_in_box_fails() {
        let board = to_board([
            "8.....8..",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!is_valid_sudoku(board));
    }
}