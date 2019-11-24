/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 *
 * https://leetcode.com/problems/reverse-string/description/
 *
 * algorithms
 * Easy (63.82%)
 * Total Accepted:    530.8K
 * Total Submissions: 822.1K
 * Testcase Example:  '["h","e","l","l","o"]'
 *
 * Write a function that reverses a string. The input string is given as an
 * array of characters char[].
 * 
 * Do not allocate extra space for another array, you must do this by modifying
 * the input array in-place with O(1) extra memory.
 * 
 * You may assume all the characters consist of printable ascii
 * characters.
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: ["h","e","l","l","o"]
 * Output: ["o","l","l","e","h"]
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: ["H","a","n","n","a","h"]
 * Output: ["h","a","n","n","a","H"]
 * 
 * 
 * 
 */
struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        for i in 0..s.len() / 2 {
            let back = s.len() - i - 1;
            let (left, right) = s.split_at_mut(back);
            std::mem::swap(&mut left[i], &mut right[0]);
        }
    }
}
