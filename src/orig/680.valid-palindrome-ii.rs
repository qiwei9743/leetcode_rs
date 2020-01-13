/*
 * @lc app=leetcode id=680 lang=rust
 *
 * [680] Valid Palindrome II
 *
 * https://leetcode.com/problems/valid-palindrome-ii/description/
 *
 * algorithms
 * Easy (35.38%)
 * Total Accepted:    106.8K
 * Total Submissions: 301.7K
 * Testcase Example:  '"aba"'
 *
 * 
 * Given a non-empty string s, you may delete at most one character.  Judge
 * whether you can make it a palindrome.
 * 
 * 
 * Example 1:
 * 
 * Input: "aba"
 * Output: True
 * 
 * 
 * 
 * Example 2:
 * 
 * Input: "abca"
 * Output: True
 * Explanation: You could delete the character 'c'.
 * 
 * 
 * 
 * Note:
 * 
 * The string will only contain lowercase characters a-z.
 * The maximum length of the string is 50000.
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn valid_palindrome(s: String) -> bool {
        Self::helper(s.as_bytes(), 1)
    }
    fn helper(bytes: &[u8], cnt: i32) -> bool {
        for i in 0..bytes.len() / 2 {
            if bytes[i] != bytes[bytes.len()-i-1] {
                return if cnt > 0 {
                    Self::helper(&bytes[i..bytes.len()-i-1], cnt - 1) ||
                        Self::helper(&bytes[i+1..=bytes.len()-i-1], cnt - 1)
                } else { false }
            }
        }
        true
    }
}
