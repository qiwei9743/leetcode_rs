/*
 * @lc app=leetcode id=1091 lang=rust
 *
 * [1091] Shortest Path in Binary Matrix
 *
 * https://leetcode.com/problems/shortest-path-in-binary-matrix/description/
 *
 * algorithms
 * Medium (37.24%)
 * Likes:    222
 * Dislikes: 31
 * Total Accepted:    17.9K
 * Total Submissions: 48.2K
 * Testcase Example:  '[[0,1],[1,0]]'
 *
 * In an N by N square grid, each cell is either empty (0) or blocked (1).
 * 
 * A clear path from top-left to bottom-right has length k if and only if it is
 * composed of cells C_1, C_2, ..., C_k such that:
 * 
 * 
 * Adjacent cells C_i and C_{i+1} are connected 8-directionally (ie., they are
 * different and share an edge or corner)
 * C_1 is at location (0, 0) (ie. has value grid[0][0])
 * C_k is at location (N-1, N-1) (ie. has value grid[N-1][N-1])
 * If C_i is located at (r, c), then grid[r][c] is empty (ie. grid[r][c] ==
 * 0).
 * 
 * 
 * Return the length of the shortest such clear path from top-left to
 * bottom-right.  If such a path does not exist, return -1.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: [[0,1],[1,0]]
 * 
 * 
 * Output: 2
 * 
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [[0,0,0],[1,1,0],[1,1,0]]
 * 
 * 
 * Output: 4
 * 
 * 
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= grid.length == grid[0].length <= 100
 * grid[r][c] is 0 or 1
 * 
 * 
 */

// @lc code=start

#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 || *grid.last().unwrap().last().unwrap() == 1 {
            return -1;
        }

        if grid.len() == 1 {
            return 1;
        }

        use std::collections::VecDeque;
        let mut dq = VecDeque::new();

        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1),
                             (1, 1), (1, -1), (-1, 1), (-1, -1)];

        dq.push_back((0, 0));
        let mut distance = 1;
        while !dq.is_empty() {
            distance += 1;

            let dq_size = dq.len();
            for _ in 0..dq_size {
                let pos = dq.pop_front().unwrap();
                for dir in &directions {
                    let (x, y) = (pos.0 + dir.0, pos.1 + dir.1);
                    if Self::is_valid_pos(x, y, grid.len() as i32)
                        && grid[x as usize][y as usize] == 0 {

                        if x as usize == grid.len() - 1 && y as usize == grid.len() - 1 {
                            return distance;
                        }
                        grid[x as usize][y as usize] = 1;
                        dq.push_back((x, y));
                    }
                }
            }
        }
        -1
    }
    fn is_valid_pos(x: i32, y: i32, N: i32) -> bool {
        0 <= x && x < N && 0 <= y && y < N
    }
}
// @lc code=end
