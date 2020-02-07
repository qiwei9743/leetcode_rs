/*
 * @lc app=leetcode id=4 lang=rust
 *
  * [4] Median of Two Sorted Arrays
 *
 * https://leetcode.com/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (27.81%)
 * Likes:    5702
 * Dislikes: 851
 * Total Accepted:    572.3K
 * Total Submissions: 2M
 * Testcase Example:  '[1,3]\n[2]'
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 * 
 * Find the median of the two sorted arrays. The overall run time complexity
 * should be O(log (m+n)).
 * 
 * You may assume nums1 and nums2 cannot be both empty.
 * 
 * Example 1:
 * 
 * 
 * nums1 = [1, 3]
 * nums2 = [2]
 * 
 * The median is 2.0
 * 
 * 
 * Example 2:
 * 
 * 
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 * 
 * The median is (2 + 3)/2 = 2.5
 * 
 * 
 */

// @lc code=start

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        if total % 2 == 0 {
            (Self::nth(&nums1[..], &nums2[..], total / 2) +
                Self::nth(&nums1[..], &nums2[..], total / 2 + 1)) / 2.0
        } else {
            Self::nth(&nums1[..], &nums2[..], total / 2 + 1)
        }
    }

    fn nth(nums1: &[i32], nums2: &[i32], k: usize) -> f64 {
        let (mut l, mut r) = (
            *std::cmp::min(nums1.first().unwrap_or(&std::i32::MAX), nums2.first().unwrap_or(&std::i32::MAX)),
            *std::cmp::max(nums1.last().unwrap_or(&std::i32::MIN), nums2.last().unwrap_or(&std::i32::MIN))
        );
        while l < r {
            let m = l + (r - l) / 2;
            if Self::upper_bound(&nums1, m) + Self::upper_bound(&nums2, m) >= k {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as f64
    }
    fn upper_bound(arr: &[i32], t: i32) -> usize {
        let (mut l, mut r) = (0, arr.len());
        while l < r {
            let m = l + (r - l) / 2;
            if arr[m] > t  {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }

    fn nth2(nums1: &[i32], nums2: &[i32], k: usize) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::nth(nums2, nums1, k);
        }
        if nums1.is_empty() {
            return nums2[k-1] as f64;
        }
        if k == 1 {
            return std::cmp::min(nums1[0], nums2[0]) as f64;
        }
        let pa = std::cmp::min(k / 2, nums1.len());
        let pb = k - pa;
        if nums1[pa-1] > nums2[pb-1] {
            Self::nth(nums1, &nums2[pb..], k - pb)
        } else {
            Self::nth(&nums1[pa..], nums2, k - pa)
        }
    }
}
// @lc code=end
