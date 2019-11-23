/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 *
 * https://leetcode.com/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (47.36%)
 * Total Accepted:    302.6K
 * Total Submissions: 621.9K
 * Testcase Example:  '5'
 *
 * Given a non-negative integer numRows, generate the first numRows of Pascal's
 * triangle.
 * 
 * 
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it.
 * 
 * Example:
 * 
 * 
 * Input: 5
 * Output:
 * [
 * ⁠    [1],
 * ⁠   [1,1],
 * ⁠  [1,2,1],
 * ⁠ [1,3,3,1],
 * ⁠[1,4,6,4,1]
 * ]
 * 
 * 
 */
struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec![];
        }
        let mut res = vec![vec![1]];
        for row_n in 1..num_rows {
            let prev = &res[res.len()-1];
            let mut cur = vec![];
            for idx in 0..row_n as usize +1 {
                cur.push(
                    *prev.get(idx).unwrap_or(&0) +
                        *prev.get(idx-1).unwrap_or(&0));
            }
            res.push(cur);
        }
        res
    }
}
