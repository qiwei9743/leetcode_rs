/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 *
 * https://leetcode.com/problems/merge-sorted-array/description/
 *
 * algorithms
 * Easy (36.51%)
 * Total Accepted:    431K
 * Total Submissions: 1.2M
 * Testcase Example:  '[1,2,3,0,0,0]\n3\n[2,5,6]\n3'
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as
 * one sorted array.
 * 
 * Note:
 * 
 * 
 * The number of elements initialized in nums1 and nums2 are m and n
 * respectively.
 * You may assume that nums1 has enough space (size that is greater or equal to
 * m + n) to hold additional elements from nums2.
 * 
 * 
 * Example:
 * 
 * 
 * Input:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 * 
 * Output: [1,2,2,3,5,6]
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        for i in (0..nums1.len()).rev() {
            if m < 1 || n < 1 {
                break
            }
            nums1[i] = match nums1[m-1].cmp(&nums2[n-1]) {
                std::cmp::Ordering::Less => {
                    n -= 1;
                    nums2[n]
                },
                std::cmp::Ordering::Greater => {
                    m -= 1;
                    nums1[m]
                },
                std::cmp::Ordering::Equal => {
                    m -= 1;
                    nums1[m]
                }
            };
        }
        nums1[..n].clone_from_slice(&nums2[..n]);
    }
}
