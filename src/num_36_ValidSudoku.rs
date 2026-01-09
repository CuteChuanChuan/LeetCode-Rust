use super::Solution;

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use itertools::Itertools;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::new(); 9];
        let mut cols = vec![HashSet::new(); 9];
        let mut subboxes = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                let num = board[i][j];
                if num == '.' {
                    continue;
                }

                let box_idx = (i / 3) * 3 + j / 3;

                if rows[i].contains(&num)
                    || cols[j].contains(&num)
                    || subboxes[box_idx].contains(&num)
                {
                    return false;
                }

                rows[i].insert(num);
                cols[j].insert(num);
                subboxes[box_idx].insert(num);
            }
        }

        true
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
