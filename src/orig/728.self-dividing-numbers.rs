/*
 * @lc app=leetcode id=728 lang=rust
 *
 * [728] Self Dividing Numbers
 *
 * https://leetcode.com/problems/self-dividing-numbers/description/
 *
 * algorithms
 * Easy (71.08%)
 * Total Accepted:    101.5K
 * Total Submissions: 140.8K
n * Testcase Example:  '1\n22'
 *
 * 
 * A self-dividing number is a number that is divisible by every digit it
 * contains.
 * 
 * For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 ==
 * 0, and 128 % 8 == 0.
 * 
 * Also, a self-dividing number is not allowed to contain the digit zero.
 * 
 * Given a lower and upper number bound, output a list of every possible self
 * dividing number, including the bounds if possible.
 * 
 * Example 1:
 * 
 * Input: 
 * left = 1, right = 22
 * Output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
 * 
 * 
 * 
 * Note:
 * The boundaries of each input argument are 1 .
 * 
 */
struct Solution;
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right).filter(|&n| {
            let mut nn = n;
            if nn == 0 { return false; }
            while nn != 0 {
                let reminder = nn % 10;
                if reminder == 0 {
                    return false;
                }
                if n % reminder != 0 { return false; }
                nn = nn / 10;
            }
            true
        }).collect()
    }
}
