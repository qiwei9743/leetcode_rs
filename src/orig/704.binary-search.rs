/*
 * @lc app=leetcode id=704 lang=rust
 *
 * [704] Binary Search
 *
 * https://leetcode.com/problems/binary-search/description/
 *
 * algorithms
 * Easy (50.28%)
 * Likes:    433
 * Dislikes: 38
 * Total Accepted:    95.3K
 * Total Submissions: 189.5K
 * Testcase Example:  '[-1,0,3,5,9,12]\n9'
 *
 * Given a sorted (in ascending order) integer array nums of n elements and a
 * target value, write a function to search target in nums. If target exists,
 * then return its index, otherwise return -1.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [-1,0,3,5,9,12], target = 2
 * Output: -1
 * Explanation: 2 does not exist in nums so return -1
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * You may assume that all elements in nums are unique.
 * n will be in the range [1, 10000].
 * The value of each element in nums will be in the range [-9999, 9999].
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] >= target {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if l < nums.len() && nums[l] == target { l as i32 } else { -1 }
    }
}
// @lc code=end
