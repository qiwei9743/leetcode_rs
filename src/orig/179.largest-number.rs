/*
 * @lc app=leetcode id=179 lang=rust
 *
 * [179] Largest Number
 *
 * https://leetcode.com/problems/largest-number/description/
 *
 * algorithms
 * Medium (26.31%)
 * Total Accepted:    141K
 * Total Submissions: 532.7K
 * Testcase Example:  '[10,2]'
 *
 * Given a list of non negative integers, arrange them such that they form the
 * largest number.
 * 
 * Example 1:
 * 
 * 
 * Input: [10,2]
 * Output: "210"
 * 
 * Example 2:
 * 
 * 
 * Input: [3,30,34,5,9]
 * Output: "9534330"
 * 
 * 
 * Note: The result may be very large, so you need to return a string instead
 * of an integer.
 * 
 */
struct Solution;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        if nums.iter().all(|x| x == &0) {
            return "0".into();
        }

        nums.sort_by(|x, y| {
            let a = Self::concat_number(x, y);
            let b = Self::concat_number(y, x);
            a.cmp(&b)
        });

        nums.into_iter().rev().map(|x| x.to_string() ).collect()
    }

    fn concat_number(a: &i32, b: &i32) -> u64 {
        let mut c = a.to_string();
        c.push_str(&b.to_string());
        c.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_number() {
        assert_eq!(Solution::largest_number(vec![10,2]), "210".to_owned());
        assert_eq!(Solution::largest_number(vec![3,30,34,5,9]), "9534330".to_owned());
    }
}