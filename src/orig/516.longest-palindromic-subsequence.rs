/*
 * @lc app=leetcode id=516 lang=rust
 *
 * [516] Longest Palindromic Subsequence
 *
 * https://leetcode.com/problems/longest-palindromic-subsequence/description/
 *
 * algorithms
 * Medium (47.87%)
 * Total Accepted:    72.1K
 * Total Submissions: 149.2K
 * Testcase Example:  '"bbbab"'
 *
 * 
 * Given a string s, find the longest palindromic subsequence's length in s.
 * You may assume that the maximum length of s is 1000.
 * 
 * 
 * Example 1:
 * Input: 
 * 
 * "bbbab"
 * 
 * Output: 
 * 
 * 4
 * 
 * One possible longest palindromic subsequence is "bbbb".
 * 
 * 
 * Example 2:
 * Input:
 * 
 * "cbbd"
 * 
 * Output:
 * 
 * 2
 * 
 * One possible longest palindromic subsequence is "bb".
 * 
 */
struct Solution;
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut dp = vec![vec![0; s.len()]; s.len()];
        let sc1 = s.chars().collect::<Vec<_>>();
        let sc2 = s.chars().rev().collect::<Vec<_>>();
        for i in 0..sc1.len() {
            for j in 0..sc1.len() {
                dp[i][j] = if sc1[i] == sc2[j] {
                    if i == 0 || j == 0 {
                        1
                    } else {
                        dp[i-1][j-1] + 1
                    }
                } else {
                    std::cmp::max(if i > 1 { dp[i-1][j] } else { 0 },
                                  if j > 1 { dp[i][j-1] } else { 0 })
                }
            }
        }
        dp.last().unwrap_or(0).last().unwrap_or(0)
    }
}
