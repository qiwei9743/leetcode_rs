/*
 * @lc app=leetcode id=1011 lang=rust
 *
 * [1011] Capacity To Ship Packages Within D Days
 *
 * https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/description/
 *
 * algorithms
 * Medium (56.24%)
 * Likes:    689
 * Dislikes: 23
 * Total Accepted:    27.6K
 * Total Submissions: 48.9K
 * Testcase Example:  '[1,2,3,4,5,6,7,8,9,10]\n5'
 *
 * A conveyor belt has packages that must be shipped from one port to another
 * within D days.
 * 
 * The i-th package on the conveyor belt has a weight of weights[i].  Each day,
 * we load the ship with packages on the conveyor belt (in the order given by
 * weights). We may not load more weight than the maximum weight capacity of
 * the ship.
 * 
 * Return the least weight capacity of the ship that will result in all the
 * packages on the conveyor belt being shipped within D days.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: weights = [1,2,3,4,5,6,7,8,9,10], D = 5
 * Output: 15
 * Explanation: 
 * A ship capacity of 15 is the minimum to ship all the packages in 5 days like
 * this:
 * 1st day: 1, 2, 3, 4, 5
 * 2nd day: 6, 7
 * 3rd day: 8
 * 4th day: 9
 * 5th day: 10
 * 
 * Note that the cargo must be shipped in the order given, so using a ship of
 * capacity 14 and splitting the packages into parts like (2, 3, 4, 5), (1, 6,
 * 7), (8), (9), (10) is not allowed. 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: weights = [3,2,2,4,1,4], D = 3
 * Output: 6
 * Explanation: 
 * A ship capacity of 6 is the minimum to ship all the packages in 3 days like
 * this:
 * 1st day: 3, 2
 * 2nd day: 2, 4
 * 3rd day: 1, 4
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: weights = [1,2,3,1,1], D = 4
 * Output: 3
 * Explanation: 
 * 1st day: 1
 * 2nd day: 2
 * 3rd day: 3
 * 4th day: 1, 1
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= D <= weights.length <= 50000
 * 1 <= weights[i] <= 500
 * 
 */

// @lc code=start
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let (mut l, mut r) = (1, weights.iter().map(|&x| x as i64).sum());
        while l < r {
            let m = l + (r - l) / 2;
            if Self::fit(&weights, m, d as i64) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32
    }
    fn fit(weights: &Vec<i32>, load: i64, mut d: i64) -> bool {
        let mut left = if d > 0 { d -= 1; load } else { 0 };
        for w in weights {
            let w1 = *w as i64;
            match left.cmp(&w1) {
                std::cmp::Ordering::Less => {
                    if d > 0 {
                        if load < w1 {
                            return false;
                        }
                        left = load - w1;
                        d -= 1;
                    } else {
                        return false;
                    }
                },
                _ => {
                    left -= w1;
                },
            }
        }
        true
    }
}
// @lc code=end
