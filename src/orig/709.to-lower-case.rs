/*
 * @lc app=leetcode id=709 lang=rust
 *
 * [709] To Lower Case
 *
 * https://leetcode.com/problems/to-lower-case/description/
 *
 * algorithms
 * Easy (77.38%)
 * Total Accepted:    162.8K
 * Total Submissions: 208.6K
 * Testcase Example:  '"Hello"'
 *
 * Implement function ToLowerCase() that has a string parameter str, and
 * returns the same string in lowercase.
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: "Hello"
 * Output: "hello"
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "here"
 * Output: "here"
 * 
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: "LOVELY"
 * Output: "lovely"
 * 
 * 
 * 
 * 
 */
impl Solution {
    pub fn to_lower_case(str: String) -> String {
        //str.to_lowercase()
        str.chars().map(|c| c.to_lowercase().next().unwrap()).collect()
    }
}
