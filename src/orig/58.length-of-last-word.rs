/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 *
 * https://leetcode.com/problems/length-of-last-word/description/
 *
 * algorithms
 * Easy (32.29%)
 * Total Accepted:    295.2K
 * Total Submissions: 914.7K
 * Testcase Example:  '"Hello World"'
 *
 * Given a string s consists of upper/lower-case alphabets and empty space
 * characters ' ', return the length of last word in the string.
 * 
 * If the last word does not exist, return 0.
 * 
 * Note: A word is defined as a character sequence consists of non-space
 * characters only.
 * 
 * Example:
 * 
 * 
 * Input: "Hello World"
 * Output: 5
 * 
 * 
 * 
 * 
 */
struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.chars().rev()
            .skip_while(|c| c.is_whitespace())
            .enumerate()
            .find(|(idx, c)| c.is_whitespace())
            .map_or_else(
                || s.chars().filter(|c| c.is_alphabetic()).count(),
                |(idx, _)| idx ) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("  ".into()), 0);
        assert_eq!(Solution::length_of_last_word(" fff ".into()), 3);
        assert_eq!(Solution::length_of_last_word("fff".into()), 3);
        assert_eq!(Solution::length_of_last_word("".into()), 0);
        assert_eq!(Solution::length_of_last_word("hello world".into()), 5);
        assert_eq!(Solution::length_of_last_word("hello world  ".into()), 5);
    }
}
