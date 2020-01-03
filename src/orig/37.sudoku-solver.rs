/*
 * @lc app=leetcode id=37 lang=rust
 *
 * [37] Sudoku Solver
 *
 * https://leetcode.com/problems/sudoku-solver/description/
 *
 * algorithms
 * Hard (39.66%)
 * Total Accepted:    152.9K
 * Total Submissions: 385.5K
 * Testcase Example:  '[["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]'
 *
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 * 
 * A sudoku solution must satisfy all of the following rules:
 * 
 * 
 * Each of the digits 1-9 must occur exactly once in each row.
 * Each of the digits 1-9 must occur exactly once in each column.
 * Each of the the digits 1-9 must occur exactly once in each of the 9 3x3
 * sub-boxes of the grid.
 * 
 * 
 * Empty cells are indicated by the character '.'.
 * 
 * 
 * A sudoku puzzle...
 * 
 * 
 * ...and its solution numbers marked in red.
 * 
 * Note:
 * 
 * 
 * The given board contain only digits 1-9 and the character '.'.
 * You may assume that the given Sudoku puzzle will have a single unique
 * solution.
 * The given board size is always 9x9.
 * 
 * 
 */
struct Solution;
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = vec![vec![false; 9]; 9];
        let mut cols = vec![vec![false; 9]; 9];
        let mut boxes = vec![vec![vec![false; 9]; 3]; 3];

        for (i, line) in board.iter().enumerate() {
            for (j, n) in line.iter().enumerate()
                .filter(|(k, strn)| strn.is_digit(10))
                .map(|(k, strn)| (k, strn.to_digit(10).unwrap() as usize - 1)) {
                    let (x, y) = (i/3, j/3);
                    boxes[x][y][n] = true;
                    rows[i][n] = true;
                    cols[j][n] = true;
                }
        }
        Solution::dfs(board, Solution::next_empty_pos(board, 0),
                      &mut rows, &mut cols, &mut boxes);
    }

    fn dfs(board: &mut Vec<Vec<char>>, pos: usize,
           rows: &mut Vec<Vec<bool>>,
           cols: &mut Vec<Vec<bool>>,
           boxes: &mut Vec<Vec<Vec<bool>>>) -> bool {
        if pos > 80 {
            return true;
        }
        let (x, y) = (pos/9, pos%9);
        let (i, j) = (x/3, y/3);

        for n in 0..9 {
            if !boxes[i][j][n] && !rows[x][n] && !cols[y][n] {

                board[x][y] = std::char::from_digit(n as u32 + 1, 10).unwrap();
                boxes[i][j][n] = true;
                rows[x][n] = true;
                cols[y][n] = true;


                if Self::dfs(board, Self::next_empty_pos(board,  pos), rows, cols, boxes) {
                    return true;
                }
                board[x][y] = '.';
                boxes[i][j][n] = false;
                rows[x][n] = false;
                cols[y][n] = false;
            }
        }
        false
    }

    fn next_empty_pos(board: &Vec<Vec<char>>, pos: usize) -> usize {
        for cur in pos..81 {
            let (x, y) = (cur/9, cur%9);
            if !board[x][y].is_digit(10) {
                return cur;
            }
        }
        std::usize::MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sudoku() {
        let mut board = vec![vec!['5','3','.','.','7','.','.','.','.'],
                             vec!['6','.','.','1','9','5','.','.','.'],
                             vec!['.','9','8','.','.','.','.','6','.'],
                             vec!['8','.','.','.','6','.','.','.','3'],
                             vec!['4','.','.','8','.','3','.','.','1'],
                             vec!['7','.','.','.','2','.','.','.','6'],
                             vec!['.','6','.','.','.','.','2','8','.'],
                             vec!['.','.','.','4','1','9','.','.','5'],
                             vec!['.','.','.','.','8','.','.','7','9']];

        Solution::solve_sudoku(&mut board);
        print!("{:?}", board);
    }
}
