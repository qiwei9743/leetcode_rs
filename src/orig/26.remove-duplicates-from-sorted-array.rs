/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
 *
 * algorithms
 * Easy (41.55%)
 * Total Accepted:    731K
 * Total Submissions: 1.7M
 * Testcase Example:  '[1,1,2]'
 *
 * Given a sorted array nums, remove the duplicates in-place such that each
 * element appear only once and return the new length.
 * 
 * Do not allocate extra space for another array, you must do this by modifying
 * the input array in-place with O(1) extra memory.
 * 
 * Example 1:
 * 
 * 
 * Given nums = [1,1,2],
 * 
 * Your function should return length = 2, with the first two elements of nums
 * being 1 and 2 respectively.
 * 
 * It doesn't matter what you leave beyond the returned length.
 * 
 * Example 2:
 * 
 * 
 * Given nums = [0,0,1,1,1,2,2,3,3,4],
 * 
 * Your function should return length = 5, with the first five elements of nums
 * being modified to 0, 1, 2, 3, and 4 respectively.
 * 
 * It doesn't matter what values are set beyond the returned length.
 * 
 * 
 * Clarification:
 * 
 * Confused why the returned value is an integer but your answer is an array?
 * 
 * Note that the input array is passed in by reference, which means
 * modification to the input array will be known to the caller as well.
 * 
 * Internally you can think of this:
 * 
 * 
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeDuplicates(nums);
 * 
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len
 * elements.
 * for (int i = 0; i < len; i++) {
 * print(nums[i]);
 * }
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut j = 0;
        for i in 1..nums.len() {
            if nums[j] < nums[i] {
                j += 1;
                nums[j] = nums[i];
            }
        }
        if !nums.is_empty() { j as i32 +1 } else { 0 }
    }
    #[allow(dead_code)]
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        let mut slow = 1;
        let mut quick = 1;
        while slow < nums.len() && quick < nums.len() {
            if nums[slow-1] < nums[slow] {
                slow += 1;
                quick = std::cmp::max(slow, quick);
            } else if nums[slow-1] < nums[quick] {
                nums.swap(slow, quick);
                slow += 1;
                quick += 1;
            } else {
                quick += 1;
            }
        }
        nums.truncate(slow);
        slow as i32
    }
}
