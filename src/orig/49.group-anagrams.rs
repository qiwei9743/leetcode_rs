/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 *
 * https://leetcode.com/problems/group-anagrams/description/
 *
 * algorithms
 * Medium (48.57%)
 * Total Accepted:    380.3K
 * Total Submissions: 781.4K
 * Testcase Example:  '["eat","tea","tan","ate","nat","bat"]'
 *
 * Given an array of strings, group anagrams together.
 * 
 * Example:
 * 
 * 
 * Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * Output:
 * [
 * ⁠ ["ate","eat","tea"],
 * ⁠ ["nat","tan"],
 * ⁠ ["bat"]
 * ]
 * 
 * Note:
 * 
 * 
 * All inputs will be in lowercase.
 * The order of your output does not matter.
 * 
 * 
 */
struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
        for st in strs {
            let mut ochars = st.chars().collect::<Vec<_>>();
            ochars.sort();
            let key = ochars.into_iter().collect::<String>();
            hash.entry(key).or_insert(vec![]).push(st);
        }
        hash.into_iter().map(|(_, value)| value).collect()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1() {
//         assert_eq!(Solution::group_anagrams(
//             vec!["eat", "tea", "tan", "ate", "nat", "bat"].into_iter().map(|x| String::from(x)).collect()),

//                    vec![
//                        vec!["ate","eat","tea"],
//                        vec!["nat","tan"],
//                        vec!["bat"]])
//     }
// }
