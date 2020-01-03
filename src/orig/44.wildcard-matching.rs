/*
 * @lc app=leetcode id=44 lang=rust
 *
 * [44] Wildcard Matching
 *
 * https://leetcode.com/problems/wildcard-matching/description/
 *
 * algorithms
 * Hard (23.67%)
 * Total Accepted:    206.3K
 * Total Submissions: 871.7K
 * Testcase Example:  '"aa"\n"a"'
 *
 * Given an input string (s) and a pattern (p), implement wildcard pattern
 * matching with support for '?' and '*'.
 * 
 * 
 * '?' Matches any single character.
 * '*' Matches any sequence of characters (including the empty sequence).
 * 
 * 
 * The matching should cover the entire input string (not partial).
 * 
 * Note:
 * 
 * 
 * s could be empty and contains only lowercase letters a-z.
 * p could be empty and contains only lowercase letters a-z, and characters
 * like ? or *.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input:
 * s = "aa"
 * p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * s = "aa"
 * p = "*"
 * Output: true
 * Explanation: '*' matches any sequence.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input:
 * s = "cb"
 * p = "?a"
 * Output: false
 * Explanation: '?' matches 'c', but the second letter is 'a', which does not
 * match 'b'.
 * 
 * 
 * Example 4:
 * 
 * 
 * Input:
 * s = "adceb"
 * p = "*a*b"
 * Output: true
 * Explanation: The first '*' matches the empty sequence, while the second '*'
 * matches the substring "dce".
 * 
 * 
 * Example 5:
 * 
 * 
 * Input:
 * s = "acdcb"
 * p = "a*c?b"
 * Output: false
 * 
 * 
 */
//struct Solution;
//https://leetcode.com/problems/wildcard-matching/discuss/438843/C%2B%2B-DP-in-one-row-(9MB-%40-24ms)
impl Solution {
    pub fn is_match<S: AsRef<str>>(s: S, p: S) -> bool {
        let (s, p) = (s.as_ref().as_bytes(), p.as_ref().as_bytes());
        if s.is_empty() { return p.is_empty() || p.iter().all(|x| *x == b'*'); }
        else if p.is_empty() { return false; }

        let mut dp = vec![false; p.len()+1];
        dp[0] = true;
        for j in 1..dp.len() {
            dp[j] = if p[j-1] == b'*' { dp[j-1] } else { break };
        }
        for i in 1..=s.len() {
            let mut dp_i_1_j_1 = dp[0];
            for j in 1..dp.len() {
                let saved = dp[j];
                dp[j] = if s[i-1] == p[j-1] || p[j-1] == b'?' { dp_i_1_j_1 }
                else if p[j-1] == b'*' { dp[j] || dp[j-1] }
                else { false };
                dp_i_1_j_1 = saved;
            }
            if i == 1 { dp[0] = false; }
        }
        *dp.last().unwrap()
    }


    pub fn is_match_dp2<S: AsRef<str>>(s: S, p: S) -> bool {
        let (s, p) = (s.as_ref().as_bytes(), p.as_ref().as_bytes());
        if s.is_empty() { return p.is_empty() || p.iter().all(|x| *x == b'*'); }
        else if p.is_empty() { return false; }

        let mut dp = vec![vec![false; p.len()+1]; 2];
        dp[0][0] = true;
        for j in 1..dp[0].len() {
            dp[0][j] = if p[j-1] == b'*' { dp[0][j-1] } else { false };
        }
        for i in 1..=s.len() {
            for j in 1..dp[0].len() {
                dp[1][j] = if s[i-1] == p[j-1] || p[j-1] == b'?' { dp[0][j-1] }
                else if p[j-1] == b'*' { dp[0][j] || dp[1][j-1] }
                else { false };
            }
            if i == 1 { dp[0][0] = false; }
            dp.swap(0, 1);
        }
        *dp.first().unwrap().last().unwrap()
    }

    pub fn is_match_dpn<S: AsRef<str>>(s: S, p: S) -> bool {
        let (s, p) = (s.as_ref().as_bytes(), p.as_ref().as_bytes());
        if s.is_empty() { return p.is_empty() || p.iter().all(|x| *x == b'*'); }
        else if p.is_empty() { return false; }

        let mut dp = vec![vec![false; p.len()+1]; s.len()+1];
        dp[0][0] = true;
        for j in 1..dp[0].len() {
            dp[0][j] = if p[j-1] == b'*' { dp[0][j-1] } else { false };
        }
        for i in 1..dp.len() {
            for j in 1..dp[i].len() {
                dp[i][j] = if s[i-1] == p[j-1] || p[j-1] == b'?' { dp[i-1][j-1] }
                else if p[j-1] == b'*' { dp[i-1][j] || dp[i][j-1] }
                else { false };
            }
        }
        *dp.last().unwrap().last().unwrap()
    }

    pub fn is_match_tle<S: AsRef<str>>(s: S, p: S) -> bool {
        let s = s.as_ref().as_bytes();
        let p = p.as_ref().as_bytes();
        Self::patten_match(&s[..], &p[..], 0, 0)
    }
    pub fn patten_match(s: &[u8], p: &[u8], mut si: usize, mut pi: usize) -> bool {
        while si < s.len() && pi < p.len() {
            if p[pi] == b'*' {
                return (si..=s.len()).any(|ssi| Self::patten_match(
                    s, p, ssi,
                    Self::find_not_star(p, pi+1)));
            }
            if s[si] != p[pi] && p[pi] != b'?' { return false; }
            si += 1;
            pi += 1;
        }
        if pi < p.len() && p[pi] == b'*' {
            return (si..=s.len()).any(|ssi| Self::patten_match(
                s, p, ssi,
                Self::find_not_star(p, pi+1)));
        }
        si == s.len() && pi == p.len()
    }

    fn find_not_star(p: &[u8], from: usize) -> usize {
        for i in from..p.len() {
            if p[i] != b'*' {
                return i;
            }
        }
        p.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match_44() {
        assert_eq!(Solution::is_match("aa", "aa"), true);
        assert_eq!(Solution::is_match("", ""), true);
        assert_eq!(Solution::is_match("a", ""), false);
        assert_eq!(Solution::is_match("a", "*"), true);
        assert_eq!(Solution::is_match("", ""), true);
        assert_eq!(Solution::is_match("aa", "a"), false);
        assert_eq!(Solution::is_match("aa", "*"), true);

        assert_eq!(
            Solution::is_match(String::from(""), String::from("*")),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("a"), String::from("a*")),
            true
        );

        assert_eq!(Solution::is_match("aa", "*"), true);
        assert_eq!(Solution::is_match("adceb", "*a*b"), true);
        assert_eq!(Solution::is_match("bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab", "b*b*ab**ba*b**b***bba"), false);

        assert_eq!(Solution::is_match("cb", "?a"), false);
        assert_eq!(Solution::is_match("acdcb", "a*c?b"), false);
        assert_eq!(Solution::is_match("baab", "a?"), false);
    }
}
