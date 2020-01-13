/*
 * @lc app=leetcode id=258 lang=rust
 *
 * [258] Add Digits
 *
 * https://leetcode.com/problems/add-digits/description/
 *
 * algorithms
 * Easy (54.46%)
 * Total Accepted:    246.8K
 * Total Submissions: 452.5K
 * Testcase Example:  '38'
 *
 * Given a non-negative integer num, repeatedly add all its digits until the
 * result has only one digit.
 * 
 * Example:
 * 
 * 
 * Input: 38
 * Output: 2 
 * Explanation: The process is like: 3 + 8 = 11, 1 + 1 = 2. 
 * Since 2 has only one digit, return it.
 * 
 * 
 * Follow up:
 * Could you do it without any loop/recursion in O(1) runtime?
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num > 9 {
            num = num.to_string()
                .bytes()
                .rev()
                .fold(0, |acc, x| acc + i32::from(x - b'0'));
        }
        num
    }
}
