/*
 * @lc app=leetcode id=365 lang=rust
 *
 * [365] Water and Jug Problem
 *
 * https://leetcode.com/problems/water-and-jug-problem/description/
 *
 * algorithms
 * Medium (29.71%)
 * Likes:    211
 * Dislikes: 605
 * Total Accepted:    33.3K
 * Total Submissions: 111.7K
 * Testcase Example:  '3\n5\n4'
 *
 * You are given two jugs with capacities x and y litres. There is an infinite
 * amount of water supply available. You need to determine whether it is
 * possible to measure exactly z litres using these two jugs.
 * 
 * If z liters of water is measurable, you must have z liters of water
 * contained within one or both buckets by the end.
 * 
 * Operations allowed:
 * 
 * 
 * Fill any of the jugs completely with water.
 * Empty any of the jugs.
 * Pour water from one jug into another till the other jug is completely full
 * or the first jug itself is empty.
 * 
 * 
 * Example 1: (From the famous "Die Hard" example)
 * 
 * 
 * Input: x = 3, y = 5, z = 4
 * Output: True
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: x = 2, y = 6, z = 5
 * Output: False
 * 
 */

// @lc code=start

#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        Self::dfs(&mut std::collections::HashSet::new(), 0, 0, x, y, z)
    }
    #[allow(dead_code)]
    fn dfs(visited: &mut std::collections::HashSet<(i32, i32)>,
           cx:i32, cy:i32, x: i32, y: i32, z: i32) -> bool {
        if !visited.insert((cx, cy)) { return false; }
        if cx+cy == z { return true; }

        Self::dfs(visited, x, cy, x, y, z) || Self::dfs(visited, cx, y, x, y, z) ||
            Self::dfs(visited, 0, cy, x, y, z) || Self::dfs(visited, cx, 0, x, y, z) ||
            Self::dfs(visited, std::cmp::max(0, cx-(y-cy)), std::cmp::min(y, cy+cx), x, y, z) ||
            Self::dfs(visited, std::cmp::min(x, cx+cy), std::cmp::max(0, cy-(x-cx)), x, y, z)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_measure_water() {
        //assert_eq!(Solution::can_measure_water(3, 5, 4), true);
        assert_eq!(Solution::can_measure_water(2, 6, 5), true);
    }
}
