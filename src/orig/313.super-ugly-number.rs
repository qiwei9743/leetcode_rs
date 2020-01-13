/*
 * @lc app=leetcode id=313 lang=rust
 *
 * [313] Super Ugly Number
 *
 * https://leetcode.com/problems/super-ugly-number/description/
 *
 * algorithms
 * Medium (42.28%)
 * Total Accepted:    67.7K
 * Total Submissions: 157K
 * Testcase Example:  '12\n[2,7,13,19]'
 *
 * Write a program to find the n^th super ugly number.
 * 
 * Super ugly numbers are positive numbers whose all prime factors are in the
 * given prime list primes of size k.
 * 
 * Example:
 * 
 * 
 * Input: n = 12, primes = [2,7,13,19]
 * Output: 32 
 * Explanation: [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first
 * 12 
 * ⁠            super ugly numbers given primes = [2,7,13,19] of size 4.
 * 
 * Note:
 * 
 * 
 * 1 is a super ugly number for any given primes.
 * The given numbers in primes are in ascending order.
 * 0 < k ≤ 100, 0 < n ≤ 10^6, 0 < primes[i] < 1000.
 * The n^th super ugly number is guaranteed to fit in a 32-bit signed integer.
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let (mut indexes, mut res) = (vec![0; primes.len()], vec![1i32; n as usize]);
        for i in 1..n as usize {
            res[i] = indexes.iter().enumerate().map(|(ii, &i)| {
                if let (t, false) = res[i].overflowing_mul(primes[ii]) {
                    t } else { return std::i32::MAX}
            }).min().unwrap();
            indexes.iter_mut().enumerate().for_each(
                |(jj, j)| if res[*j]*primes[jj] <= res[i] { *j += 1; })
        }
        *res.last().unwrap()
    }
}
