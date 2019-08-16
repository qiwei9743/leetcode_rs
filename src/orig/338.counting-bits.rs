/*
 * @lc app=leetcode id=338 lang=rust
 *
 * [338] Counting Bits
 *
 * https://leetcode.com/problems/counting-bits/description/
 *
 * algorithms
 * Medium (65.22%)
 * Total Accepted:    182.4K
 * Total Submissions: 279.1K
 * Testcase Example:  '2'
 *
 * Given a non negative integer number num. For every numbers i in the range 0
 * ≤ i ≤ num calculate the number of 1's in their binary representation and
 * return them as an array.
 * 
 * Example 1:
 * 
 * 
 * Input: 2
 * Output: [0,1,1]
 * 
 * Example 2:
 * 
 * 
 * Input: 5
 * Output: [0,1,1,2,1,2]
 * 
 * 
 * Follow up:
 * 
 * 
 * It is very easy to come up with a solution with run time
 * O(n*sizeof(integer)). But can you do it in linear time O(n) /possibly in a
 * single pass?
 * Space complexity should be O(n).
 * Can you do it like a boss? Do it without using any builtin function like
 * __builtin_popcount in c++ or in any other language.
 * 
 */
struct Solution;

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        if num < 0 {
            return vec![];
        }
        let mut dp = vec![0; (num+1) as usize];
        dp[0] = 0;
        if num == 0 {
            return dp;
        }
        dp[1] = 1;

        if num == 1 {
            return dp;
        }
        let mut max_bit1 = 1;
        for i in 2..=num as usize {
            if i < (1 << (max_bit1 + 1)) {
                dp[i] = 1 + dp[i - (1<<max_bit1)];
            } else {
                dp[i] = 1;
                max_bit1 += 1;
            }
        }
        dp
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::count_bits(3), vec![0, 1, 1,2]);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::count_bits(4), vec![0, 1, 1, 2, 1]);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::count_bits(5), vec![0,1,1,2,1,2]);
    }
}