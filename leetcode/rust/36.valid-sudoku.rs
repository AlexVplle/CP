use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: [HashSet<char>; 9] = [
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
        ];
        let mut columns: [HashSet<char>; 9] = [
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
        ];
        let mut boxes: [HashSet<char>; 9] = [
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
            HashSet::with_capacity(9),
        ];
        for i in 0..9 {
            for j in 0..9 {
                let value: char = board[i][j];
                if value != '.' {
                    if !rows[i].insert(value)
                        || !columns[j].insert(value)
                        || !boxes[i / 3 * 3 + j / 3].insert(value)
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}
