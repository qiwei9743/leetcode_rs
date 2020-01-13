/*
 * @lc app=leetcode id=167 lang=rust
 *
 * [167] Two Sum II - Input array is sorted
 *
 * https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/
 *
 * algorithms
 * Easy (51.07%)
 * Total Accepted:    313.1K
 * Total Submissions: 603.6K
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * Given an array of integers that is already sorted in ascending order, find
 * two numbers such that they add up to a specific target number.
 * 
 * The function twoSum should return indices of the two numbers such that they
 * add up to the target, where index1 must be less than index2.
 * 
 * Note:
 * 
 * 
 * Your returned answers (both index1 and index2) are not zero-based.
 * You may assume that each input would have exactly one solution and you may
 * not use the same element twice.
 * 
 * 
 * Example:
 * 
 * 
 * Input: numbers = [2,7,11,15], target = 9
 * Output: [1,2]
 * Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.
 * 
 */
#[cfg(feature = "local")]
#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut low, mut high) = (0, numbers.len()-1);
        loop {
            if numbers[low] + numbers[high] > target {
                high -= 1;
            } else if numbers[low] + numbers[high] < target {
                low += 1;
            } else {
                return vec![low as i32 + 1, high as i32 + 1];
            }
        }
    }
}
