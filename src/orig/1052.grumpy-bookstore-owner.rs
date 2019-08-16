/*
 * @lc app=leetcode id=1052 lang=rust
 *
 * [1052] Grumpy Bookstore Owner
 *
 * https://leetcode.com/problems/grumpy-bookstore-owner/description/
 *
 * algorithms
 * Medium (51.44%)
 * Total Accepted:    4.8K
 * Total Submissions: 9.4K
 * Testcase Example:  '[1,0,1,2,1,1,7,5]\n[0,1,0,1,0,1,0,1]\n3'
 *
 * Today, the bookstore owner has a store open for customers.length minutes.
 * Every minute, some number of customers (customers[i]) enter the store, and
 * all those customers leave after the end of that minute.
 * 
 * On some minutes, the bookstore owner is grumpy.  If the bookstore owner is
 * grumpy on the i-th minute, grumpy[i] = 1, otherwise grumpy[i] = 0.  When the
 * bookstore owner is grumpy, the customers of that minute are not satisfied,
 * otherwise they are satisfied.
 * 
 * The bookstore owner knows a secret technique to keep themselves not grumpy
 * for X minutes straight, but can only use it once.
 * 
 * Return the maximum number of customers that can be satisfied throughout the
 * day.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], X = 3
 * Output: 16
 * Explanation: The bookstore owner keeps themselves not grumpy for the last 3
 * minutes. 
 * The maximum number of customers that can be satisfied = 1 + 1 + 1 + 1 + 7 +
 * 5 = 16.
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= X <= customers.length == grumpy.length <= 20000
 * 0 <= customers[i] <= 1000
 * 0 <= grumpy[i] <= 1
 * 
 */
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let x = x as usize;
        //let mut max_straight_sum:i32 = customers[..x-1].iter().filter(||).sum();
        let mut max_straight_sum:i32 = (0..x-1).map(
            |i| if grumpy[i] == 0 {0} else {customers[i]}
        ).sum();

        let (max, j) = (x-1..customers.len()).map(| i | {
            let mut range_sum = 0;
            if grumpy[i] == 1 {
                max_straight_sum += customers[i];
                range_sum = max_straight_sum;
            }
            if grumpy[i-x+1] == 1 {
                max_straight_sum -= customers[i-x+1];
            }
            println!("{} {} {}", i, range_sum, max_straight_sum);
            (range_sum, i)
        }).max().unwrap_or((0, 0));

        println!("{} {}", max, j);

        (0..customers.len()).map(
            |i| {
                if i <= j  && j < i+x {
                    customers[i]
                } else {
                    if grumpy[i] == 1 {0} else {customers[i]}
                }
            }).sum()
    }
}
