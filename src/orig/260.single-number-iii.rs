/*
 * @lc app=leetcode id=260 lang=rust
 *
 * [260] Single Number III
 *
 * https://leetcode.com/problems/single-number-iii/description/
 *
 * algorithms
 * Medium (57.82%)
 * Total Accepted:    119.7K
 * Total Submissions: 203.8K
 * Testcase Example:  '[1,2,1,3,2,5]'
 *
 * Given an array of numbers nums, in which exactly two elements appear only
 * once and all the other elements appear exactly twice. Find the two elements
 * that appear only once.
 * 
 * Example:
 * 
 * 
 * Input:  [1,2,1,3,2,5]
 * Output: [3,5]
 * 
 * Note:
 * 
 * 
 * The order of the result is not important. So in the above example, [5, 3] is
 * also correct.
 * Your algorithm should run in linear runtime complexity. Could you implement
 * it using only constant space complexity?
 * 
 */
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xorop = |p, n| p ^ n;
        let xor2 = nums.iter().fold(0, xorop);
        vec![nums.iter().filter(|x| **x & (xor2&-xor2) != 0 ).fold(0, xorop),
             nums.iter().filter(|x| (!(**x)) & (xor2&-xor2) != 0).fold(0, xorop)]
    }
}
