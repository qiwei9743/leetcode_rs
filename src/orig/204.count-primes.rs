/*
 * @lc app=leetcode id=204 lang=rust
 *
 * [204] Count Primes
 *
 * https://leetcode.com/problems/count-primes/description/
 *
 * algorithms
 * Easy (29.46%)
 * Total Accepted:    257.6K
 * Total Submissions: 870.8K
 * Testcase Example:  '10'
 *
 * Count the number of prime numbers less than a non-negative number, n.
 * 
 * Example:
 * 
 * 
 * Input: 10
 * Output: 4
 * Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
 * 
 * 
 */
struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let mut primes = vec![true; n as usize];
        let sq = (n as f64).sqrt().floor() as usize;
        for i in 2..=sq {
            for j in (i*i..n as usize).step_by(i) {
                primes[j] = false;
            }
        }
        primes.iter().filter(|x| **x).count() as i32 - 2
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_primes(10), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::count_primes(2), 0);
    }
}