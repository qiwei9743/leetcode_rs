/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 *
 * https://leetcode.com/problems/implement-strstr/description/
 *
 * algorithms
 * Easy (32.67%)
 * Total Accepted:    486.7K
 * Total Submissions: 1.5M
 * Testcase Example:  '"hello"\n"ll"'
 *
 * Implement strStr().
 * 
 * Return the index of the first occurrence of needle in haystack, or -1 if
 * needle is not part of haystack.
 * 
 * Example 1:
 * 
 * 
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 * 
 * 
 * Clarification:
 * 
 * What should we return when needle is an empty string? This is a great
 * question to ask during an interview.
 * 
 * For the purpose of this problem, we will return 0 when needle is an empty
 * string. This is consistent to C's strstr() and Java's indexOf().
 * 
 */
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1
        }
        for i in 0..haystack.len()-needle.len()+1 {
            if &needle[..] == &haystack[i..i+needle.len()] {
                return i as i32
            }
        }
        -1
    }
}
