/*
 * @lc app=leetcode id=441 lang=rust
 *
 * [441] Arranging Coins
 *
 * https://leetcode.com/problems/arranging-coins/description/
 *
 * algorithms
 * Easy (39.10%)
 * Likes:    252
 * Dislikes: 547
 * Total Accepted:    85.7K
 * Total Submissions: 218.8K
 * Testcase Example:  '5'
 *
 * You have a total of n coins that you want to form in a staircase shape,
 * where every k-th row must have exactly k coins.
 * ⁠
 * Given n, find the total number of full staircase rows that can be formed.
 * 
 * n is a non-negative integer and fits within the range of a 32-bit signed
 * integer.
 * 
 * Example 1:
 * 
 * n = 5
 * 
 * The coins can form the following rows:
 * ¤
 * ¤ ¤
 * ¤ ¤
 * 
 * Because the 3rd row is incomplete, we return 2.
 * 
 * 
 * 
 * Example 2:
 * 
 * n = 8
 * 
 * The coins can form the following rows:
 * ¤
 * ¤ ¤
 * ¤ ¤ ¤
 * ¤ ¤
 * 
 * Because the 4th row is incomplete, we return 3.
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let (mut l, mut r) = (0, n as i64 + 1);
        while l < r {
            let m = l + (r - l) / 2;
            if m * (m + 1) / 2 > n as i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32 - 1
    }
}
// @lc code=end
