/*
 * @lc app=leetcode id=80 lang=rust
 *
 * [80] Remove Duplicates from Sorted Array II
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/description/
 *
 * algorithms
 * Medium (41.03%)
 * Total Accepted:    226.8K
 * Total Submissions: 542.7K
 * Testcase Example:  '[1,1,1,2,2,3]'
 *
 * Given a sorted array nums, remove the duplicates in-place such that
 * duplicates appeared at most twice and return the new length.
 * 
 * Do not allocate extra space for another array, you must do this by modifying
 * the input array in-place with O(1) extra memory.
 * 
 * Example 1:
 * 
 * 
 * Given nums = [1,1,1,2,2,3],
 * 
 * Your function should return length = 5, with the first five elements of nums
 * being 1, 1, 2, 2 and 3 respectively.
 * 
 * It doesn't matter what you leave beyond the returned length.
 * 
 * Example 2:
 * 
 * 
 * Given nums = [0,0,1,1,1,1,2,3,3],
 * 
 * Your function should return length = 7, with the first seven elements of
 * nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively.
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
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }
        let mut i = 2;
        for j in 2..nums.len() {
            if nums[i-2] != nums[j] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        nums.truncate(i);
        i as i32
    }
    #[allow(dead_code)]
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut cnt = 1;
        let mut j = 1;
        for i in 1..nums.len() {
            if nums[i-1] == nums[i] {
                cnt += 1;
            } else {
                cnt = 1;
            }
            nums[j] = nums[i];
            if cnt < 3 {
                j += 1;
            }
        }
        nums.truncate(j);
        j as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplciates_for_sorted_array_ii() {
        let mut ll = vec![1,2,3];
        Solution::remove_duplicates(&mut ll);
        assert_eq!(ll, vec![1,2,3]);


        ll = vec![1,1,1,2,3];
        Solution::remove_duplicates(&mut ll);
        assert_eq!(ll, vec![1,1,2,3]);

        ll = vec![1,1,1,1,2,3];
        Solution::remove_duplicates(&mut ll);
        assert_eq!(ll, vec![1,1,2,3]);
    }
}
