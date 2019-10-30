/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 *
 * https://leetcode.com/problems/maximum-subarray/description/
 *
 * algorithms
 * Easy (42.05%)
 * Total Accepted:    425.4K
 * Total Submissions: 1M
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * Given an integer array nums, find the contiguous subarray (containing at
 * least one number) which has the largest sum and return its sum.
 * 
 * Example:
 * 
 * 
 * Input: [-2,1,-3,4,-1,2,1,-5,4],
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 * 
 * 
 * Follow up:
 * 
 * If you have figured out the O(n) solution, try coding another solution using
 * the divide and conquer approach, which is more subtle.
 * 
 */

fn divide(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
    if start >= end {
        nums[start]
    } else {
        let mid  = start + (end - start) / 2;
        let leftmax = divide(&nums, start, mid-1);
        let rightmax = divide(&nums, mid+1, end);
        let mut sum = 0;
        let mut left = 0;
        let mut right = 0;
        if mid - 1 >= 0 {
            for i in mid-1..=0 {
                sum += nums[i];
                if left < sum {
                    left = sum;
                }
            }
        }
        sum = 0;
        if mid+1 <= end {
            for i in mid+1..=end {
                sum += nums[i];
                if right < sum {
                    right = sum;
                }
            }
        }
        println!("leftmax={} rightmax={} left={} rigth={} mid={}",
                 leftmax, rightmax, left, right, mid);
        *vec![leftmax, rightmax, nums[mid]+left+right].iter().max().unwrap()
    }
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        divide(&nums, 0, nums.len()-1)
    }
}
