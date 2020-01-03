/*
 * @lc app=leetcode id=137 lang=rust
 *
 * [137] Single Number II
 *
 * https://leetcode.com/problems/single-number-ii/description/
 *
 * algorithms
 * Medium (46.74%)
 * Total Accepted:    187.1K
 * Total Submissions: 391.9K
 * Testcase Example:  '[2,2,3,2]'
 *
 * Given a non-empty array of integers, every element appears three times
 * except for one, which appears exactly once. Find that single one.
 * 
 * Note:
 * 
 * Your algorithm should have a linear runtime complexity. Could you implement
 * it without using extra memory?
 * 
 * Example 1:
 * 
 * 
 * Input: [2,2,3,2]
 * Output: 3
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [0,1,0,1,0,1,99]
 * Output: 99
 * 
 */
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        Self::_single(nums, 3, 1)
    }

    fn _single(nums: Vec<i32>, k: i32, _l: i32) -> i32 {
        (0..std::mem::size_of::<i32>()*8).map(|i| {
            if nums.iter().filter(|n| **n & (1 << i) != 0).count() % k as usize > 0 {
                1 << i } else { 0 } }).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number_ii() {
        assert_eq!(Solution::single_number(vec![1,1,1,3,3,3,2]), 2);
        assert_eq!(Solution::single_number(vec![1,1,1,3]), 3);
        assert_eq!(Solution::single_number(vec![1,1,1,3,3]), 3);
        assert_eq!(Solution::single_number(vec![1,1,1,2,3,3,3]), 2);
        assert_eq!(Solution::single_number(vec![1,1,1,3,3]), 3);
        assert_eq!(Solution::single_number(vec![1,1,1,3,3,3,4]), 4);
        assert_eq!(Solution::single_number(vec![1,1,1,3,3,3,4,4]), 4);
        assert_eq!(Solution::single_number(vec![1,1,1,2,2,2,3,3,3,4]), 4);
        assert_eq!(Solution::single_number(vec![1,1,1,2,2,2,3,3,3,4,4]), 4);
        assert_eq!(Solution::single_number(vec![-1,1,1,1]), -1);

    }
}
