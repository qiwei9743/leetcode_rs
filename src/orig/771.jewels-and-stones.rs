/*
 * @lc app=leetcode id=771 lang=rust
 *
 * [771] Jewels and Stones
 *
 * https://leetcode.com/problems/jewels-and-stones/description/
 *
 * algorithms
 * Easy (83.47%)
 * Total Accepted:    317.1K
 * Total Submissions: 379.3K
 * Testcase Example:  '"aA"\n"aAAbbbb"'
 *
 * You're given strings J representing the types of stones that are jewels, and
 * S representing the stones you have.  Each character in S is a type of stone
 * you have.  You want to know how many of the stones you have are also
 * jewels.
 * 
 * The letters in J are guaranteed distinct, and all characters in J and S are
 * letters. Letters are case sensitive, so "a" is considered a different type
 * of stone from "A".
 * 
 * Example 1:
 * 
 * 
 * Input: J = "aA", S = "aAAbbbb"
 * Output: 3
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: J = "z", S = "ZZ"
 * Output: 0
 * 
 * 
 * Note:
 * 
 * 
 * S and J will consist of letters and have length at most 50.
 * The characters in J are distinct.
 * 
 * 
 */

#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let jset = j.chars().collect::<std::collections::HashSet<_>>();
        s.chars().filter(|x| jset.contains(x)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_jewels_in_stones("aA".into(), "aAAbbbb".into()), 3);
        //assert_eq!(Solution::num_jewels_in_stones("z".into(), "ZZ".into()), );
    }
}
