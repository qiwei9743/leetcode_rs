/*
 * @lc app=leetcode id=541 lang=rust
 *
 * [541] Reverse String II
 *
 * https://leetcode.com/problems/reverse-string-ii/description/
 *
 * algorithms
 * Easy (45.87%)
 * Total Accepted:    71.1K
 * Total Submissions: 152.7K
 * Testcase Example:  '"abcdefg"\n2'
 *
 * 
 * Given a string and an integer k, you need to reverse the first k characters
 * for every 2k characters counting from the start of the string. If there are
 * less than k characters left, reverse all of them. If there are less than 2k
 * but greater than or equal to k characters, then reverse the first k
 * characters and left the other as original.
 * 
 * 
 * Example:
 * 
 * Input: s = "abcdefg", k = 2
 * Output: "bacdfeg"
 * 
 * 
 * 
 * Restrictions: 
 * 
 * ⁠The string consists of lower English letters only.
 * ⁠Length of the given string and k will in the range [1, 10000]
 * 
 */
struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s: Vec<_> = s.chars().collect();
        for i in (0..s.len()).step_by(k * 2) {
            s[i..i+k].reverse();
        }
        s.iter().collect()
    }
}
