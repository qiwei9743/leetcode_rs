/*
 * @lc app=leetcode id=712 lang=rust
 *
 * [712] Minimum ASCII Delete Sum for Two Strings
 *
 * https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/description/
 *
 * algorithms
 * Medium (56.54%)
 * Likes:    703
 * Dislikes: 41
 * Total Accepted:    27.5K
 * Total Submissions: 48.5K
 * Testcase Example:  '"sea"\n"eat"'
 *
 * Given two strings s1, s2, find the lowest ASCII sum of deleted characters to
 * make two strings equal.
 * 
 * Example 1:
 * 
 * Input: s1 = "sea", s2 = "eat"
 * Output: 231
 * Explanation: Deleting "s" from "sea" adds the ASCII value of "s" (115) to
 * the sum.
 * Deleting "t" from "eat" adds 116 to the sum.
 * At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum
 * possible to achieve this.
 * 
 * 
 * 
 * Example 2:
 * 
 * Input: s1 = "delete", s2 = "leet"
 * Output: 403
 * Explanation: Deleting "dee" from "delete" to turn the string into "let",
 * adds 100[d]+101[e]+101[e] to the sum.  Deleting "e" from "leet" adds 101[e]
 * to the sum.
 * At the end, both strings are equal to "let", and the answer is
 * 100+101+101+101 = 403.
 * If instead we turned both strings into "lee" or "eet", we would get answers
 * of 433 or 417, which are higher.
 * 
 * 
 * 
 * Note:
 * 0 < s1.length, s2.length .
 * All elements of each string will have an ASCII value in [97, 122]. 
 * 
 */

// @lc code=start
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        
    }
}
// @lc code=end
