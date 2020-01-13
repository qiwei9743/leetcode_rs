/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 *
 * https://leetcode.com/problems/sqrtx/description/
 *
 * algorithms
 * Easy (31.90%)
 * Total Accepted:    443.8K
 * Total Submissions: 1.4M
 * Testcase Example:  '4'
 *
 * Implement int sqrt(int x).
 * 
 * Compute and return the square root of x, where x is guaranteed to be a
 * non-negative integer.
 * 
 * Since the return type is an integer, the decimal digits are truncated and
 * only the integer part of the result is returned.
 * 
 * Example 1:
 * 
 * 
 * Input: 4
 * Output: 2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 8
 * Output: 2
 * Explanation: The square root of 8 is 2.82842..., and since 
 * the decimal part is truncated, 2 is returned.
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut left, mut right) = (0u64, x as u64 + 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if mid * mid > x as u64 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32 - 1
    }

    #[allow(dead_code)]
    pub fn my_sqrt2(x: i32) -> i32 {
        let (mut low, mut high) = (0u64, x as u64 /2+1);
        while low < high {
            let mid = low + (high - low + 1) / 2;
            if mid * mid > x as u64 { high = mid - 1; } else { low = mid; }
        }
        low as i32
    }
}
