/*
 * @lc app=leetcode id=50 lang=rust
 *
 * [50] Pow(x, n)
 *
 * https://leetcode.com/problems/powx-n/description/
 *
 * algorithms
 * Medium (28.39%)
 * Total Accepted:    376.3K
 * Total Submissions: 1.3M
 * Testcase Example:  '2.00000\n10'
 *
 * Implement pow(x, n), which calculates x raised to the power n (x^n).
 * 
 * Example 1:
 * 
 * 
 * Input: 2.00000, 10
 * Output: 1024.00000
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 2.10000, 3
 * Output: 9.26100
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: 2.00000, -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 * 
 * 
 * Note:
 * 
 * 
 * -100.0 < x < 100.0
 * n is a 32-bit signed integer, within the range [−2^31, 2^31 − 1]
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

use std::ops::BitAnd;

impl Solution {
    #[allow(dead_code)]
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n < 0 {
            x = 1.0/x;
            n = if n == std::i32::MIN { x *= x; std::i32::MAX } else { -n };
        }
        let mut res = 1.0;
        let mut cur = x;
        for i in 0..std::mem::size_of_val(&n)*8-1 {
            if n.bitand(1 << i as i32) > 0 {
                res *= cur;
            }
            cur *= cur;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_pow() {
        assert_eq!(
            Solution::my_pow(2.0, -2147483648), 0.0);
        assert_eq!(
            Solution::my_pow(-2.0, -2147483648), 0.0);
    }
}
