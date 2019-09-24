/*
 * @lc app=leetcode id=395 lang=rust
 *
 * [395] Longest Substring with At Least K Repeating Characters
 *
 * https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/description/
 *
 * algorithms
 * Medium (39.19%)
 * Total Accepted:    54K
 * Total Submissions: 137.3K
 * Testcase Example:  '"aaabb"\n3'
 *
 * 
 * Find the length of the longest substring T of a given string (consists of
 * lowercase letters only) such that every character in T appears no less than
 * k times.
 * 
 * 
 * Example 1:
 * 
 * Input:
 * s = "aaabb", k = 3
 * 
 * Output:
 * 3
 * 
 * The longest substring is "aaa", as 'a' is repeated 3 times.
 * 
 * 
 * 
 * Example 2:
 * 
 * Input:
 * s = "ababbc", k = 2
 * 
 * Output:
 * 5
 * 
 * The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is
 * repeated 3 times.
 * 
 * 
 */
struct Solution;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::longest(&s[..], k)
    }
    fn longest(s: &str, k: i32) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut hcntr = HashMap::new();
        for c in s.chars() {
            *hcntr.entry(c).or_insert(0) += 1;
        }
        let mut spliter_set = hcntr
            .iter()
            .filter(|(c, cnt)| **cnt < k)
            .map(|(c, cnt)| c).collect::<HashSet<_>>();

        if spliter_set.is_empty() {
            return s.len() as i32;
        }

        let mut len = 0;
        let mut max = 0;
        for (i, c) in s.chars().enumerate() {
            if !spliter_set.contains(&c) {
                len += 1;
            } else {
                if len > 0 {
                    max = std::cmp::max(max, Self::longest(&s[i-len..i], k));
                }
                len = 0;
            }
        }
        if len > 0 {
            max = std::cmp::max(max, Self::longest(&s[s.len()-len..], k));
        }
        max
    }

//    pub fn longest_substring(s: String, k: i32) -> i32 {
//        if s.is_empty() {
//            return 0;
//        }
//        let mut res = 0;
//        let mut current = HashMap::new();
//        let mut future = HashMap::new();
//        let chars:Vec<_> = s.chars().collect();
//        for c in chars.iter() {
//            let mut entry = future.entry(c).or_insert(0);
//            *entry += 1;
//        }
//
//        let mut start = 0;
//
//        for (i, c) in chars.iter().enumerate() {
//            *future.get_mut(&c).unwrap() -= 1;
//            *(current.entry(c).or_insert(0)) += 1;
//
//            if current.get(&c).unwrap_or(&0) + future.get(&c).unwrap_or(&0) < k as usize{
//                while current[&c] > 0 {
//                    *current.get_mut(&chars[start]).unwrap() -= 1;
//                    start += 1;
//                }
//            } else {
//                if current.iter().all(|(_, value)| *value >= k as usize) {
//                    res = std::cmp::max(res, i-start+1);
//                }
//            }
//        }
//        res as i32
//    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_with_repeating_chars() {
        assert_eq!(Solution::longest_substring("aaabb".into(), 3), 3);
        assert_eq!(Solution::longest_substring("ababbc".into(), 2), 5);
        assert_eq!(Solution::longest_substring("".into(), 2), 0);
        assert_eq!(Solution::longest_substring("aaaaa".into(), 2), 5);
        assert_eq!(Solution::longest_substring("aaaaabbc".into(), 2), 7);
        assert_eq!(Solution::longest_substring("bbaaacbd".into(), 3), 3);
    }

}
