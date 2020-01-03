/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (37.59%)
 * Total Accepted:    761.7K
 * Total Submissions: 2M
 * Testcase Example:  '"()"'
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 * 
 * An input string is valid if:
 * 
 * 
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 * 
 * 
 * Note that an empty string is also considered valid.
 * 
 * Example 1:
 * 
 * 
 * Input: "()"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "()[]{}"
 * Output: true
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: "(]"
 * Output: false
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: "([)]"
 * Output: false
 * 
 * 
 * Example 5:
 * 
 * 
 * Input: "{[]}"
 * Output: true
 * 
 * 
 */
struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;
        let mut stack = vec!['#'];
        let map: HashMap<_,_> = vec![('(', ')'), ('[', ']'), ('{', '}')].into_iter().collect();
        for c in s.chars() {
            if map.contains_key(&c) {
                stack.push(c);
            } else {
                match map.get(stack.last().unwrap()) {
                    Some(val) if *val == c => stack.pop(),
                    _ => return false
                };
            }
        }
        stack.len() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_parentheses_i() {
        assert_eq!(
            Solution::is_valid(String::from("()")), true);
        assert_eq!(
            Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(
            Solution::is_valid(String::from("(]")), false);
        assert_eq!(
            Solution::is_valid(String::from("([)]")), false);
        assert_eq!(
            Solution::is_valid(String::from("{[]}")), true);
    }
}
