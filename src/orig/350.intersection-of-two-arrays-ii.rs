/*
 * @lc app=leetcode id=350 lang=rust
 *
 * [350] Intersection of Two Arrays II
 *
 * https://leetcode.com/problems/intersection-of-two-arrays-ii/description/
 *
 * algorithms
 * Easy (50.04%)
 * Likes:    985
 * Dislikes: 330
 * Total Accepted:    278.8K
 * Total Submissions: 556.5K
 * Testcase Example:  '[1,2,2,1]\n[2,2]'
 *
 * Given two arrays, write a function to compute their intersection.
 * 
 * Example 1:
 * 
 * 
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2,2]
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [4,9]
 * 
 * 
 * Note:
 * 
 * 
 * Each element in the result should appear as many times as it shows in both
 * arrays.
 * The result can be in any order.
 * 
 * 
 * Follow up:
 * 
 * 
 * What if the given array is already sorted? How would you optimize your
 * algorithm?
 * What if nums1's size is small compared to nums2's size? Which algorithm is
 * better?
 * What if elements of nums2 are stored on disk, and the memory is limited such
 * that you cannot load all elements into the memory at once?
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort();
        nums2.sort();
        let mut res = Vec::new();
        let (mut iter1, mut iter2) = (nums1.into_iter(), nums2.into_iter());
        let (mut n1, mut n2) = (iter1.next(), iter2.next());
        loop {
            if let (Some(nn1), Some(nn2)) = (n1, n2) {
                match nn1.cmp(&nn2) {
                    std::cmp::Ordering::Equal => {
                        res.push(nn1);
                        n1 = iter1.next();
                        n2 = iter2.next();
                    },
                    std::cmp::Ordering::Greater => {
                        n2 = iter2.next();
                    },
                    std::cmp::Ordering::Less => {
                        n1 = iter1.next();
                    }
                }
            } else {
                break
            }
        }
        res
    }
}
// @lc code=end
