/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 *
 * https://leetcode.com/problems/3sum/description/
 *
 * algorithms
 * Medium (24.40%)
 * Total Accepted:    596.6K
 * Total Submissions: 2.4M
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such
 * that a + b + c = 0? Find all unique triplets in the array which gives the
 * sum of zero.
 * 
 * Note:
 * 
 * The solution set must not contain duplicate triplets.
 * 
 * Example:
 * 
 * 
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 * 
 * A solution set is:
 * [
 * ⁠ [-1, 0, 1],
 * ⁠ [-1, -1, 2]
 * ]
 * 
 * 
 */
use std::collections::HashSet;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        nums.sort();
        for k in 2..nums.len() {
            findTwoSum(&nums, 0, k-1, -nums[k], &mut res);
        }
        res.into_iter().map(|(x, y, z)| vec![x, y, z]).collect()
    }
}

fn findTwoSum(nums: &Vec<i32>, mut i: usize, mut j: usize, target: i32, res: &mut HashSet<(i32, i32, i32)>) {
    while i < j {
        if nums[i] + nums[j] < target {
            i += 1;
        } else if nums[i] + nums[j] > target {
            j -= 1;
        } else {
            res.insert((nums[i], nums[j], -target));
            i += 1;
            j -= 1;
            while i < j && nums[i-1] == nums[i] {
                i += 1;
            }
            while i < j && nums[j+1] == nums[j] {
                j -= 1;
            }
        }
    }
}
