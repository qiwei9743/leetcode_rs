/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 *
 * https://leetcode.com/problems/two-sum/description/
 *
 * algorithms
 * Easy (42.15%)
 * Total Accepted:    1.6M
 * Total Submissions: 3.7M
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * Given an array of integers, return indices of the two numbers such that they
 * add up to a specific target.
 * 
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 * 
 * 
 * Example:
 * 
 * 
 * Given nums = [2, 7, 11, 15], target = 9,
 * 
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 * 
 * 
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<_> = nums.iter().enumerate().map(|(i, d)| (i, d)).collect();
        nums.sort_by_key(|k| k.1);
        let (mut low, mut high) = (0, nums.len()-1);
        loop {
            match target.cmp(&(nums[low].1 + nums[high].1)) {
                std::cmp::Ordering::Equal => {
                    return vec![nums[low].0 as i32, nums[high].0 as i32];
                },
                std::cmp::Ordering::Greater => {
                    low += 1;
                }
                std::cmp::Ordering::Less => {
                    high -= 1;
                }
            }
        }
    }
}
