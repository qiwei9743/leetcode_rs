/*
 * @lc app=leetcode id=29 lang=rust
 *
 * [29] Divide Two Integers
 *
 * https://leetcode.com/problems/divide-two-integers/description/
 *
 * algorithms
 * Medium (16.19%)
 * Likes:    900
 * Dislikes: 4354
 * Total Accepted:    238.9K
 * Total Submissions: 1.5M
 * Testcase Example:  '10\n3'
 *
 * Given two integers dividend and divisor, divide two integers without using
 * multiplication, division and mod operator.
 * 
 * Return the quotient after dividing dividend by divisor.
 * 
 * The integer division should truncate toward zero.
 * 
 * Example 1:
 * 
 * 
 * Input: dividend = 10, divisor = 3
 * Output: 3
 * 
 * Example 2:
 * 
 * 
 * Input: dividend = 7, divisor = -3
 * Output: -2
 * 
 * Note:
 * 
 * 
 * Both dividend and divisor will be 32-bit signed integers.
 * The divisor will never be 0.
 
* Assume we are dealing with an environment which could only store integers
 * within the 32-bit signed integer range: [−2^31,  2^31 − 1]. For the purpose
 * of this problem, assume that your function returns 2^31 − 1 when the
 * division result overflows.
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        let sig = if (divisor > 0 && dividend > 0) || (divisor < 0 && dividend < 0) { 1 } else { -1 };


        let dividend = dividend as i64;
        let divisor = divisor as i64;
        let dividend = dividend.abs();
        let divisor = divisor.abs();

        let (mut l, mut r) = (0, dividend);
        //println!("l={} r={}", l, r);
        while l <= r {
            let m = l + (r - l) / 2;
            if Self::mul(m, divisor) > dividend {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        let res = sig * (l - 1);
        //println!("res={}", res);
        if res < std::i32::MIN as i64 {
            std::i32::MIN
        } else if res > std::i32::MAX as i64 {
            std::i32::MAX
        } else {
            res as i32
        }
    }

    fn mul(mut x: i64, mut y: i64) -> i64 {
        if x == 0 || y == 0 { return 0; }
        let mut res = 0;
        //println!("x={} y={}", x, y);
        while y > 0 {
            if y & 1 > 0 {
                res += x;
            }
            x <<= 1;
            y >>= 1;
        }
        //println!("res={}", res);
        res
    }
}
// @lc code=end
