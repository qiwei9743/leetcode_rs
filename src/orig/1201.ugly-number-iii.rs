/*
 * @lc app=leetcode id=1201 lang=rust
 *
 * [1201] Ugly Number III
 *
 * https://leetcode.com/problems/ugly-number-iii/description/
 *
 * algorithms
 * Medium (24.98%)
 * Total Accepted:    5K
 * Total Submissions: 20.1K
 * Testcase Example:  '3\n2\n3\n5'
 *
 * Write a program to find the n-th ugly number.
 * 
 * Ugly numbers are positive integers which are divisible by a or b or c.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: n = 3, a = 2, b = 3, c = 5
 * Output: 4
 * Explanation: The ugly numbers are 2, 3, 4, 5, 6, 8, 9, 10... The 3rd is 4.
 * 
 * Example 2:
 * 
 * 
 * Input: n = 4, a = 2, b = 3, c = 4
 * Output: 6
 * Explanation: The ugly numbers are 2, 3, 4, 6, 8, 9, 10, 12... The 4th is
 * 6.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: n = 5, a = 2, b = 11, c = 13
 * Output: 10
 * Explanation: The ugly numbers are 2, 4, 6, 8, 10, 11, 12, 13... The 5th is
 * 10.
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: n = 1000000000, a = 2, b = 217983653, c = 336916467
 * Output: 1999999984
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= n, a, b, c <= 10^9
 * 1 <= a * b * c <= 10^18
 * It's guaranteed that the result will be in range [1, 2 * 10^9]
 * 
 * 
 */

#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let (mut multiple, mut factors) = (vec![1; 3], vec![a, b, c]);
        let mut num = 0;
        for i in 0..n as usize {
            num = multiple.iter().zip(factors.iter()).map(|(&mul, &factor)| mul*factor).min().unwrap();
            multiple.iter_mut().zip(factors.iter()).for_each(|(mul, &factor)| if *mul * factor == num { *mul += 1; } );
        }
        num
    }
}
