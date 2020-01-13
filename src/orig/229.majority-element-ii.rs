/*
 * @lc app=leetcode id=229 lang=rust
 *
 * [229] Majority Element II
 *
 * https://leetcode.com/problems/majority-element-ii/description/
 *
 * algorithms
 * Medium (32.79%)
 * Total Accepted:    115K
 * Total Submissions: 345K
 * Testcase Example:  '[3,2,3]'
 *
 * Given an integer array of size n, find all elements that appear more than ⌊
 * n/3 ⌋ times.
 * 
 * Note: The algorithm should run in linear time and in O(1) space.
 * 
 * Example 1:
 * 
 * 
 * Input: [3,2,3]
 * Output: [3]
 * 
 * Example 2:
 * 
 * 
 * Input: [1,1,1,3,3,2,2,2]
 * Output: [1,2]
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let (mut y, mut z, mut cy, mut cz) = (0, 0, 0, 1);
        for x in nums.iter() {
            if *x == y {
                cy += 1;
            } else if *x == z {
                cz += 1;
            } else if cy == 0 {
                y = *x;
                cy = 1;
            } else if cz == 0 {
                z = *x;
                cz = 1;
            } else {
                cy -= 1;
                cz -= 1;
            }
        }

        vec![y, z].into_iter().filter(|x| {
            nums.iter().filter(|t| *t == x).count() > nums.len() / 3
        }).collect()
    }
}
