/*
 * @lc app=leetcode id=36 lang=rust
 *
 * [36] Valid Sudoku
 *
 * https://leetcode.com/problems/valid-sudoku/description/
 *
 * algorithms
 * Medium (45.61%)
 * Total Accepted:    287.3K
 * Total Submissions: 629.8K
 * Testcase Example:  '[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]'
 *
 * Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be
 * validated according to the following rules:
 * 
 * 
 * Each row must contain the digits 1-9 without repetition.
 * Each column must contain the digits 1-9 without repetition.
 * Each of the 9 3x3 sub-boxes of the grid must contain the digits 1-9 without
 * repetition.
 * 
 * 
 * 
 * A partially filled sudoku which is valid.
 * 
 * The Sudoku board could be partially filled, where empty cells are filled
 * with the character '.'.
 * 
 * Example 1:
 * 
 * 
 * Input:
 * [
 * ⁠ ["5","3",".",".","7",".",".",".","."],
 * ⁠ ["6",".",".","1","9","5",".",".","."],
 * ⁠ [".","9","8",".",".",".",".","6","."],
 * ⁠ ["8",".",".",".","6",".",".",".","3"],
 * ⁠ ["4",".",".","8",".","3",".",".","1"],
 * ⁠ ["7",".",".",".","2",".",".",".","6"],
 * ⁠ [".","6",".",".",".",".","2","8","."],
 * ⁠ [".",".",".","4","1","9",".",".","5"],
 * ⁠ [".",".",".",".","8",".",".","7","9"]
 * ]
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * [
 * ["8","3",".",".","7",".",".",".","."],
 * ["6",".",".","1","9","5",".",".","."],
 * [".","9","8",".",".",".",".","6","."],
 * ["8",".",".",".","6",".",".",".","3"],
 * ["4",".",".","8",".","3",".",".","1"],
 * ["7",".",".",".","2",".",".",".","6"],
 * [".","6",".",".",".",".","2","8","."],
 * [".",".",".","4","1","9",".",".","5"],
 * [".",".",".",".","8",".",".","7","9"]
 * ]
 * Output: false
 * Explanation: Same as Example 1, except with the 5 in the top left corner
 * being 
 * ⁠   modified to 8. Since there are two 8's in the top left 3x3 sub-box, it
 * is invalid.
 * 
 * 
 * Note:
 * 
 * 
 * A Sudoku board (partially filled) could be valid but is not necessarily
 * solvable.
 * Only the filled cells need to be validated according to the mentioned
 * rules.
 * The given board contain only digits 1-9 and the character '.'.
 * The given board size is always 9x9.
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut sub = vec![vec![vec![false;9]; 3]; 3];
        let (mut row, mut col) = (vec![vec![false; 9]; 9], vec![vec![false; 9]; 9]);

        for (i, line) in board.iter().enumerate() {
            for (j, n) in line.iter().enumerate()
                .filter(|(i, n)| n.is_digit(10))
                .map(|(i, n)| (i, n.to_digit(10).unwrap() as usize - 1)) {
                    let (subi, subj) = (i/3, j/3);
                    if sub[subi][subj][n] || row[i][n] || col[j][n] { return false; }
                    sub[subi][subj][n] = if sub[subi][subj][n] { return false; } else { true };
                    row[i][n] = if row[i][n] { return false; } else { true };
                    col[j][n] = if col[j][n] { return false; } else { true };
            }
        }
        true
    }

    #[allow(dead_code)]
    pub fn is_valid_sudoku2(board: Vec<Vec<char>>) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut row = HashMap::new();
        let mut col = HashMap::new();
        let mut sub: Vec<Vec<HashSet<char>>> = vec![vec![Default::default(); 3]; 3];
        for (i, line) in board.iter().enumerate() {
            for (j, item) in line.iter().enumerate() {
                if !item.is_digit(10) {
                    continue
                }
                let (subi, subj) = (i/3, j/3);
                if sub[subi][subj].contains(item) { return false; }
                sub[subi][subj].insert(*item);

                let mut rowset = row.entry(i).or_insert_with(|| HashSet::<char>::new());
                if rowset.contains(item) { return false; }
                rowset.insert(*item);

                let mut colset = col.entry(j).or_insert_with(|| HashSet::<char>::new());
                if colset.contains(item) { return false; }
                colset.insert(*item);
            }
        }
        true
    }
}
