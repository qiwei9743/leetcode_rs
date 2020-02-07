/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 *
 * https://leetcode.com/problems/search-in-rotated-sorted-array/description/
 *
 * algorithms
 * Medium (33.53%)
 * Likes:    3549
 * Dislikes: 392
 * Total Accepted:    552.5K
 * Total Submissions: 1.6M
 * Testcase Example:  '[4,5,6,7,0,1,2]\n0'
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown
 * to you beforehand.
 * 
 * (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 * 
 * You are given a target value to search. If found in the array return its
 * index, otherwise return -1.
 * 
 * You may assume no duplicate exists in the array.
 * 
 * Your algorithm's runtime complexity must be in the order of O(log n).
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 * 
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let first = Self::find_first(&nums[..]);
        nums[first..].binary_search(&target)
            .map(|i| first + i)
            .or_else(|_| nums[..first].binary_search(&target))
            .map(|x| x as i32).unwrap_or(-1) as i32
    }
    fn find_first(nums: &[i32]) -> usize {
        let (mut l, mut r) = (1, nums.len());
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] < nums[0] {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if l == nums.len() { 0 } else { l }
    }












    pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return nums.iter().position(|&x| x == target).map_or(-1, |x| x as i32)
        }
        let last_pos = Self::find_last2(&nums[..]);
        match nums[..last_pos+1].binary_search(&target) {
            Ok(i) => i as i32,
            Err(_) => {
                match nums[last_pos+1..].binary_search(&target) {
                    Ok(i) => (last_pos + 1 + i) as i32,
                    Err(_) => -1,
                }
            }
        }
    }

    pub fn find_last2(nums: &[i32]) -> usize {
        let (mut l, mut r) = (0, nums.len() - 2);
        while l <= r {
            let m = l + (r - l) /2;
            if nums[m] > nums[m+1] {
                return m;
            }
            if nums[m] >= nums[0] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        l
    }
}
// @lc code=end

