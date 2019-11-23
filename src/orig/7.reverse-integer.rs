/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 *
 * https://leetcode.com/problems/reverse-integer/description/
 *
 * algorithms
 * Easy (25.45%)
 * Total Accepted:    857K
 * Total Submissions: 3.4M
 * Testcase Example:  '123'
 *
 * Given a 32-bit signed integer, reverse digits of an integer.
 * 
 * Example 1:
 * 
 * 
 * Input: 123
 * Output: 321
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: -123
 * Output: -321
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: 120
 * Output: 21
 * 
 * 
 * Note:
 * Assume we are dealing with an environment which could only store integers
 * within the 32-bit signed integer range: [−2^31,  2^31 − 1]. For the purpose
 * of this problem, assume that your function returns 0 when the reversed
 * integer overflows.
 * 
 */
struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let sig = if x >= 0 { 1 } else { -1 };
        x = x.abs();
        let mut y = 0i32;
        while x > 0 {
            let (yy, overflow) = y.overflowing_mul(10);
            if overflow {
                return 0;
            }
            let (yy, overflow) = yy.overflowing_add(x % 10);
            if overflow {
                return 0;
            }
            x /= 10;
            y = yy;
        }

        sig * y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}