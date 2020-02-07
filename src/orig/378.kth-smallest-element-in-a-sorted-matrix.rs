/*
 * @lc app=leetcode id=378 lang=rust
 *
 * [378] Kth Smallest Element in a Sorted Matrix
 *
 * https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/description/
 *
 * algorithms
 * Medium (51.82%)
 * Likes:    1712
 * Dislikes: 101
 * Total Accepted:    147.8K
 * Total Submissions: 285K
 * Testcase Example:  '[[1,5,9],[10,11,13],[12,13,15]]\n8'
 *
 * Given a n x n matrix where each of the rows and columns are sorted in
 * ascending order, find the kth smallest element in the matrix.
 * 
 * 
 * Note that it is the kth smallest element in the sorted order, not the kth
 * distinct element.
 * 
 * 
 * Example:
 * 
 * matrix = [
 * ⁠  [ 1,  5,  9],
 * ⁠  [10, 11, 13],
 * ⁠  [12, 13, 15]
 * ],
 * k = 8,
 * 
 * return 13.
 * 
 * 
 * 
 * Note: 
 * You may assume k is always valid, 1 ≤ k ≤ n^2.
 */

// @lc code=start
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        // simular as 4
        let (mut l, mut r) = (
            *matrix.first().unwrap().first().unwrap(),
            *matrix.last().unwrap().last().unwrap() + 1);
        while l < r {
            let m = l + (r - l) / 2;
            let total = matrix.iter().map(|row| {
                let (mut left, mut right) = (0, row.len());
                while left < right {
                    let mid = left + (right - left) / 2;
                    if row[mid] > m {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                }
                left
            }).sum::<usize>() as i32;
            // println!("total={} m = {}", total, m);
            if total >= k {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32
    }
}
// @lc code=end
