/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 *
 * https://leetcode.com/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (33.12%)
 * Total Accepted:    443.6K
 * Total Submissions: 1.3M
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * Given a string, determine if it is a palindrome, considering only
 * alphanumeric characters and ignoring cases.
 * 
 * Note: For the purpose of this problem, we define empty string as valid
 * palindrome.
 * 
 * Example 1:
 * 
 * 
 * Input: "A man, a plan, a canal: Panama"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "race a car"
 * Output: false
 * 
 * 
 */
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<_> = s.chars().filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase()).collect();
        s.iter().zip(s.iter().rev()).take(s.len() / 2)
            .all(|x| x.0.to_string() == x.1.to_string())
    }
}
