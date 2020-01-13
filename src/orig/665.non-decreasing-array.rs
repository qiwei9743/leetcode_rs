/*
 * @lc app=leetcode id=665 lang=rust
 *
 * [665] Non-decreasing Array
 *
 * https://leetcode.com/problems/non-decreasing-array/description/
 *
 * algorithms
 * Easy (19.46%)
 * Total Accepted:    67.5K
 * Total Submissions: 348.3K
 * Testcase Example:  '[4,2,3]'
 *
 * 
 * Given an array with n integers, your task is to check if it could become
 * non-decreasing by modifying at most 1 element.
 * 
 * 
 * 
 * We define an array is non-decreasing if array[i]  holds for every i (1 
 * 
 * Example 1:
 * 
 * Input: [4,2,3]
 * Output: True
 * Explanation: You could modify the first 4 to 1 to get a non-decreasing
 * array.
 * 
 * 
 * 
 * Example 2:
 * 
 * Input: [4,2,1]
 * Output: False
 * Explanation: You can't get a non-decreasing array by modify at most one
 * element.
 * 
 * 
 * 
 * Note:
 * The n belongs to [1, 10,000].
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let x = (0..nums.len()-1).position(|x| nums[x] > nums[x+1]);
        if x.is_none() { return true; }
        let x = x.unwrap();
        if (x+1..nums.len()-1).any(|i| nums[i] > nums[i+1]) { return false; }
        let new_x = if x == 0 { std::i32::MIN } else { *nums.get(x-1).unwrap() };
        if new_x <= nums[x+1] { true }
        else { nums[x] <= *nums.get(x+2).unwrap_or(&std::i32::MAX) }
    }
    #[allow(dead_code)]
    pub fn check_possibility2(mut nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return true;
        }
        let x = (0..nums.len()-1).position(|x| nums[x] > nums[x+1]);
        if x.is_none() {
            return true;
        }
        let x = x.unwrap();
        let prev_x = nums[x];
        nums[x] = if x == 0 { std::i32::MIN } else { *nums.get(x-1).unwrap() };
        if Self::ordered(&nums[x..]) {
            return true;
        }
        nums[x] = prev_x;
        nums[x+1] = nums[x];
        Self::ordered(&nums[x+1..])
    }
    fn ordered(nums: &[i32]) -> bool {
        for i in 1..nums.len() {
            if nums[i-1] > nums[i] {
                return false;
            }
        }
        true
    }
}
