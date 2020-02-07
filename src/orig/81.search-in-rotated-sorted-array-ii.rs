/*
 * @lc app=leetcode id=81 lang=rust
 *
 * [81] Search in Rotated Sorted Array II
 *
 * https://leetcode.com/problems/search-in-rotated-sorted-array-ii/description/
 *
 * algorithms
 * Medium (32.89%)
 * Likes:    935
 * Dislikes: 396
 * Total Accepted:    206.2K
 * Total Submissions: 626.9K
 * Testcase Example:  '[2,5,6,0,0,1,2]\n0'
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown
 * to you beforehand.
 * 
 * (i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).
 * 
 * You are given a target value to search. If found in the array return true,
 * otherwise return false.
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [2,5,6,0,0,1,2], target = 0
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [2,5,6,0,0,1,2], target = 3
 * Output: false
 * 
 * Follow up:
 * 
 * 
 * This is a follow up problem to Search in Rotated Sorted Array, where nums
 * may contain duplicates.
 * Would this affect the run-time complexity? How and why?
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() < 3 {
            return nums.iter().position(|&x| x == target).is_some();
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r && nums[l] == nums[r] {
            while l+1 < r && nums[l+1] == nums[r] { l += 1; }
            while l < r && nums[l] == nums[r] { r -= 1; }
        }
        let nums = &nums[l..=r];

        if nums.len() < 3 {
            return nums.iter().position(|&x| x == target).is_some();
        }

        let last_pos = Self::find_last(nums);
        nums[..=last_pos].binary_search(&target)
            .or_else(|_| nums[last_pos+1..].binary_search(&target))
            .is_ok()

    }
    fn find_last(nums: &[i32]) -> usize {
        let (mut l, mut r) = (0, nums.len()-2);
        while l <= r {
            let m = l + (r - l) / 2;
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

    pub fn search3(nums: Vec<i32>, target: i32) -> bool {
        nums.iter().position(|&x| x == target).is_some()
    }
    pub fn search2(nums: Vec<i32>, target: i32) -> bool {
        if let Some(last_pos) = nums.iter().zip(nums.iter().skip(1))
            .position(|(&x, &n)| x > n) {

                nums[..=last_pos].binary_search(&target).is_ok() ||
                    nums[last_pos+1..].binary_search(&target).is_ok()
        } else {
            nums.binary_search(&target).is_ok()
        }
    }
}
// @lc code=end
