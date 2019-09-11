/*
 * @lc app=leetcode id=205 lang=rust
 *
 * [205] Isomorphic Strings
 *
 * https://leetcode.com/problems/isomorphic-strings/description/
 *
 * algorithms
 * Easy (37.91%)
 * Total Accepted:    225.6K
 * Total Submissions: 592K
 * Testcase Example:  '"egg"\n"add"'
 *
 * Given two strings s and t, determine if they are isomorphic.
 * 
 * Two strings are isomorphic if the characters in s can be replaced to get t.
 * 
 * All occurrences of a character must be replaced with another character while
 * preserving the order of characters. No two characters may map to the same
 * character but a character may map to itself.
 * 
 * Example 1:
 * 
 * 
 * Input: s = "egg", t = "add"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s = "foo", t = "bar"
 * Output: false
 * 
 * Example 3:
 * 
 * 
 * Input: s = "paper", t = "title"
 * Output: true
 * 
 * Note:
 * You may assume both s and t have the same length.
 * 
 */
struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut charmap = [0u8; 256];

        //o two characters may map to the same o two characters may map to the same.
        let mut hasmappedto = [false; 256];

        s.bytes().zip(t.bytes()).all(|(x, y)| {
            if charmap[x as usize] == 0 {
                if hasmappedto[y as usize] {
                    return false;
                }
                charmap[x as usize] = y;
                hasmappedto[y as usize] = true;
            }

            charmap[x as usize] == y
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(Solution::is_isomorphic("paper".into(), "title".into()),
                   true);
        assert_eq!(Solution::is_isomorphic("foo".into(), "bar".into()),
                   false);
        assert_eq!(Solution::is_isomorphic("egg".into(), "add".into()),
                   true);
        assert_eq!(Solution::is_isomorphic("ab".into(), "aa".into()),
                   false);
    }
}
