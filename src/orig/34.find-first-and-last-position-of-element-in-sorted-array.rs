/*
 * @lc app=leetcode id=34 lang=rust
 *
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
 *
 * algorithms
 * Medium (34.01%)
 * Total Accepted:    335K
 * Total Submissions: 984.2K
 * Testcase Example:  '[5,7,7,8,8,10]\n8'
 *
 * Given an array of integers nums sorted in ascending order, find the starting
 * and ending position of a given target value.
 * 
 * Your algorithm's runtime complexity must be in the order of O(log n).
 * 
 * If the target is not found in the array, return [-1, -1].
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    fn lower_bound(vec: &[i32], mut low: usize, mut high: usize, target: i32) -> i32 {
        while low < high {
            let mid = low + (high-low) / 2;
            if vec[mid] < target {
                low = mid + 1;
            } else {
                high = mid
            }
        }
        low as i32
    }

    fn upper_bound(vec: &[i32], mut low: usize, mut high: usize, target: i32) -> i32 {
        while low < high {
            let mid = low +(high-low) / 2;
            if vec[mid] <= target {
                low = mid + 1;
            } else if vec[mid] > target {
                high = mid
            }
        }
        high as i32
    }
    #[allow(dead_code)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let lower = Self::lower_bound(&nums, 0, nums.len() as usize, target);
        if lower as usize >= nums.len() || nums[lower as usize] != target {
            return vec![-1, -1];
        }
        let upper = Self::upper_bound(&nums, lower as usize, nums.len() as usize, target);
        vec![lower, upper-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower() {
        let vv = vec![1,1,2,2,3,4,5];
        let lower = Solution::lower_bound(&vv, 0, vv.len(), 1);
        assert_eq!(lower, 0);

        assert_eq!(Solution::lower_bound(&vv, 1, vv.len(), 2), 2);
    }

    #[test]
    fn test_find_first1() {
        let vv = vec![1,2,3,4,4,4,5,5,6,7,7,7,8];
        let res = Solution::search_range(vv, 1);
        assert_eq!(res, vec![0, 0]);
    }
    #[test]
    fn test_example1() {
        let vv = vec![5,7,7,8,8,10];
        let res = Solution::search_range(vv, 8);
        assert_eq!(res, vec![3, 4]);
    }
    #[test]
    fn test_example2() {
        let vv = vec![5,7,7,8,8,10];
        let res = Solution::search_range(vv, 6);
        assert_eq!(res, vec![-1, -1]);
    }
    #[test]
    fn test_1_1() {
        let vv = vec![1];
        let res = Solution::search_range(vv, 1);
        assert_eq!(res, vec![0, 0]);
    }
}
