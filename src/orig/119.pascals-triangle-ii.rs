/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 *
 * https://leetcode.com/problems/pascals-triangle-ii/description/
 *
 * algorithms
 * Easy (44.76%)
 * Total Accepted:    233.8K
 * Total Submissions: 510.3K
 * Testcase Example:  '3'
 *
 * Given a non-negative index k where k ≤ 33, return the k^th index row of the
 * Pascal's triangle.
 * 
 * Note that the row index starts from 0.
 * 
 * 
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it.
 * 
 * Example:
 * 
 * 
 * Input: 3
 * Output: [1,3,3,1]
 * 
 * 
 * Follow up:
 * 
 * Could you optimize your algorithm to use only O(k) extra space?
 * 
 */
struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut prev = vec![1];
        if row_index == 0 {
            return prev;
        }
        for row_n in  1..=row_index {
            let cur = (0..row_n as usize + 1)
                .into_iter()
                .map(|x| {
                    *prev.get(x).unwrap_or(&0) + *prev.get(x-1).unwrap_or(&0)
                }).collect::<Vec<_>>();
            prev = cur;
        }
        prev
    }
}
