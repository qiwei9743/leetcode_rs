/*
 * @lc app=leetcode id=96 lang=rust
 *
 * [96] Unique Binary Search Trees
 *
 * https://leetcode.com/problems/unique-binary-search-trees/description/
 *
 * algorithms
 * Medium (49.36%)
 * Likes:    2491
 * Dislikes: 92
 * Total Accepted:    246.4K
 * Total Submissions: 498.9K
 * Testcase Example:  '3'
 *
 * Given n, how many structurally unique BST's (binary search trees) that store
 * values 1 ... n?
 * 
 * Example:
 * 
 * 
 * Input: 3
 * Output: 5
 * Explanation:
 * Given n = 3, there are a total of 5 unique BST's:
 * 
 * ⁠  1         3     3      2      1
 * ⁠   \       /     /      / \      \
 * ⁠    3     2     1      1   3      2
 * ⁠   /     /       \                 \
 * ⁠  2     1         2                 3
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        for i in 1..=n as usize {
            for j in 0..i {
                let left = j;
                let right = i - left - 1;
                dp[i] += dp[left] * dp[right];
            }
        }
        dp[n as usize]
    }
}
// @lc code=end
