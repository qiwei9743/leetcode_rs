/*
 * @lc app=leetcode id=215 lang=rust
 *
 * [215] Kth Largest Element in an Array
 *
 * https://leetcode.com/problems/kth-largest-element-in-an-array/description/
 *
 * algorithms
 * Medium (49.21%)
 * Total Accepted:    430.3K
 * Total Submissions: 866.6K
 * Testcase Example:  '[3,2,1,5,6,4]\n2'
 *
 * Find the kth largest element in an unsorted array. Note that it is the kth
 * largest element in the sorted order, not the kth distinct element.
 * 
 * Example 1:
 * 
 * 
 * Input: [3,2,1,5,6,4] and k = 2
 * Output: 5
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [3,2,3,1,2,4,5,5,6] and k = 4
 * Output: 4
 * 
 * Note: 
 * You may assume k is always valid, 1 ≤ k ≤ array's length.
 * 
 */
//struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let K = nums.len() - k as usize;
        let mut low = 0;
        let mut high = nums.len() - 1;
        loop {
            let t = Self::partition(&mut nums[..], low, high);
            if t < K {
                low = t + 1;
            } else if t > K {
                high = t - 1;
            } else {
                return nums[t];
            }
        }
    }
    fn partition(nums: &mut [i32], mut low: usize, mut high: usize) -> usize {
        let slot = nums[low];
        while low < high {
            while low < high && slot <= nums[high] {
                high -= 1;
            }
            if low < high {
                nums[low] = nums[high];
            }
            while low < high && nums[low] <= slot {
                low += 1;
            }
            if low < high {
                nums[high] =  nums[low];
            }
        }
        nums[low] = slot;
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(Solution::find_kth_largest(vec![1,2,3,4,5], 1), 5);
        assert_eq!(Solution::find_kth_largest(vec![1,2,3,4,5], 2), 4);
        assert_eq!(Solution::find_kth_largest(vec![4,5,1,2,3,], 2), 4);
    }
}