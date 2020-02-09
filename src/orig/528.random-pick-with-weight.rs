/*
 * @lc app=leetcode id=528 lang=rust
 *
 * [528] Random Pick with Weight
 *
 * https://leetcode.com/problems/random-pick-with-weight/description/
 *
 * algorithms
 * Medium (43.35%)
 * Likes:    451
 * Dislikes: 849
 * Total Accepted:    50.1K
 * Total Submissions: 115.6K
 * Testcase Example:  '["Solution", "pickIndex"]\n[[[1]], []]'
 *
 * Given an array w of positive integers, where w[i] describes the weight of
 * index i, write a function pickIndex which randomly picks an index in
 * proportion to its weight.
 * 
 * Note:
 * 
 * 
 * 1 <= w.length <= 10000
 * 1 <= w[i] <= 10^5
 * pickIndex will be called at most 10000 times.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: 
 * ["Solution","pickIndex"]
 * [[[1]],[]]
 * Output: [null,0]
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 
 * ["Solution","pickIndex","pickIndex","pickIndex","pickIndex","pickIndex"]
 * [[[1,3]],[],[],[],[],[]]
 * Output: [null,0,1,1,1,0]
 * 
 * 
 * Explanation of Input Syntax:
 * 
 * The input is two lists: the subroutines called and their arguments.
 * Solution's constructor has one argument, the array w. pickIndex has no
 * arguments. Arguments are always wrapped with a list, even if there aren't
 * any.
 * 
 */

// @lc code=start
struct Solution {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        
    }
    
    fn pick_index(&self) -> i32 {
        
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
// @lc code=end
