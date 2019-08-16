/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 *
 * https://leetcode.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (33.15%)
 * Total Accepted:    466.7K
 * Total Submissions: 1.4M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * Write a function to find the longest common prefix string amongst an array
 * of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 * Example 1:
 *
 *
 * Input: ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 * Note:
 *
 * All given inputs are in lowercase letters a-z.
 *
 */
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        if strs.len() == 1 {
            return String::from(&strs[0][..]);
        }
        let mut prev: Vec<_> = strs[0..strs.len() - 1].iter().map(|x| x.chars()).collect();
        let mut next: Vec<_> = strs[1..strs.len()].iter().map(|x| x.chars()).collect();
        let min_length = strs.iter().map(|x| x.len()).min().unwrap_or(0);
        //println!("{}", min_length);

        for i in 0..min_length {
            for j in 0..prev.len() {
                let a1 = prev[j].next();
                let b1 = next[j].next();
                //println!("{} {:?} {:?}", i, a1, b1);
                if a1 != b1 {
                    return String::from(&strs[0][0..i]);
                }
            }
        }
        String::from(&strs[0][..min_length])
    }
}
// pub struct Solution;
