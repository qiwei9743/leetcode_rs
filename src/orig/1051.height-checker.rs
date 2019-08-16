/*
 * @lc app=leetcode id=1051 lang=rust
 *
 * [1051] Height Checker
 *
 * https://leetcode.com/problems/height-checker/description/
 *
 * algorithms
 * Easy (70.48%)
 * Total Accepted:    6.7K
 * Total Submissions: 9.5K
 * Testcase Example:  '[1,1,4,2,1,3]'
 *
 * Students are asked to stand in non-decreasing order of heights for an annual
 * photo.
 * 
 * Return the minimum number of students not standing in the right positions.
 * (This is the number of students that must move in order for all students to
 * be standing in non-decreasing order of height.)
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: [1,1,4,2,1,3]
 * Output: 3
 * Explanation: 
 * Students with heights 4, 3 and the last 1 are not standing in the right
 * positions.
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= heights.length <= 100
 * 1 <= heights[i] <= 100
 * 
 */
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut copy = heights.clone();
        copy.sort();
        copy.iter().zip(heights.iter()).filter(|(x, y)| x != y).count() as i32
    }
}
