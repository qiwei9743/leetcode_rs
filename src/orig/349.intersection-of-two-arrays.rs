/*
 * @lc app=leetcode id=349 lang=rust
 *
 * [349] Intersection of Two Arrays
 *
 * https://leetcode.com/problems/intersection-of-two-arrays/description/
 *
 * algorithms
 * Easy (58.75%)
 * Likes:    579
 * Dislikes: 994
 * Total Accepted:    291.8K
 * Total Submissions: 495.6K
 * Testcase Example:  '[1,2,2,1]\n[2,2]'
 *
 * Given two arrays, write a function to compute their intersection.
 * 
 * Example 1:
 * 
 * 
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2]
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [9,4]
 * 
 * 
 * Note:
 * 
 * 
 * Each element in the result must be unique.
 * The result can be in any order.
 * 
 * 
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1 = nums1.into_iter().collect::<std::collections::HashSet<_>>();
        nums2.into_iter().filter(|x| nums1.contains(&x))
            .collect::<std::collections::HashSet<_>>()
            .into_iter().collect()
    }
}
// @lc code=end
