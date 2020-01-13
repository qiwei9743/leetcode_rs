/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 *
 * https://leetcode.com/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (26.66%)
 * Total Accepted:    498.5K
 * Total Submissions: 1.9M
 * Testcase Example:  '"babad"'
 *
 * Given a string s, find the longest palindromic substring in s. You may
 * assume that the maximum length of s is 1000.
 * 
 * Example 1:
 * 
 * 
 * Input: "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "cbbd"
 * Output: "bb"
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome2(s: String) -> String {
        // perf not really good. cost 8 ms. below solution cost 0 ms.
        if s.len() < 2 {
            return s;
        }
        let s = s.split("").filter(|x| !x.is_empty()).collect::<Vec<_>>().join("#");
        // println!("{:?}", s);
        let s = s.as_bytes();
        let mut dp = vec![0; s.len()];
        let mut mid = 0;
        let mut right_most = 0;
        let mut start = 0;
        let mut len = 0;

        let mut i = 1;
        while i < s.len() - len {
            let mut ilen = if i >= right_most {
                0
            } else {
                std::cmp::min(dp[2*mid - i], right_most-i-1)
            };

            while i > ilen && i + ilen + 1 < s.len() && s[i-ilen-1] == s[i+ilen+1] {
                ilen += 1;
            }
            if i + ilen > right_most {
                mid = i;
                right_most = i + ilen;
            }
            if s[start-len..=start+len].iter().filter(|x| **x != b'#').count() < s[i-ilen..=i+ilen].iter().filter(|x| **x != b'#').count() {
                start = i;
                len = ilen;
            }
            dp[i] = ilen;
            i += 1;
        }
        // let s1 = s[start-len..=start+len].iter().map(|x| *x).filter(|x| *x != b'#').collect();
        let s1 = s[start-len..=start+len].iter().copied().filter(|x| *x != b'#').collect();
        String::from_utf8(s1).unwrap()
    }
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> String {
        //https://leetcode.com/problems/longest-palindromic-substring/discuss/381827/Java-2ms-time-used
        // cost 0 ms in rust.
        let mut i = 0;
        let mut start = 0;
        let mut max = 0;
        let s: Vec<_> = s.chars().collect();
        while i < s.len() - max / 2 {
            let mut j = i;
            let mut k = i;
            while k+1 < s.len() && s[k] == s[k+1] {
                k += 1;
            }
            i = k+1;
            while j > 0 && k + 1 < s.len() && s[j-1] == s[k+1] {
                j -= 1;
                k += 1;
            }
            let length = k - j + 1;
            if length > max {
                max = length;
                start = j;
            }
        }
        s[start..start+max].iter().collect()
    }
}

#[cfg(test)]
mod testps {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome(String::from("babad")), String::from("bab"));
        
    }
    #[test]
    fn test_longest_palindrome2() {
        assert_eq!(Solution::longest_palindrome(String::from("aa")), String::from("aa"));
    }
    #[test]
    fn test_longest_palindrome3() {
        assert_eq!(Solution::longest_palindrome(String::from("abb")), String::from("bb"));
    }
}
