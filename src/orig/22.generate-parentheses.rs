/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 *
 * https://leetcode.com/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (56.41%)
 * Total Accepted:    377.8K
 * Total Submissions: 668.5K
 * Testcase Example:  '3'
 *
 * 
 * Given n pairs of parentheses, write a function to generate all combinations
 * of well-formed parentheses.
 * 
 * 
 * 
 * For example, given n = 3, a solution set is:
 * 
 * 
 * [
 * ⁠ "((()))",
 * ⁠ "(()())",
 * ⁠ "(())()",
 * ⁠ "()(())",
 * ⁠ "()()()"
 * ]
 * 
 */
struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let cur = String::new();

        Self::generate(0, 0, n, &mut String::new(), &mut res);
        res
    }

    fn generate(left: i32, right: i32, n: i32, cur: &mut String, res: &mut Vec<String>) {
        if left > n {
            return
        }
        if right == n {
            res.push(cur.clone());
            return
        }

        cur.push('(');
        Self::generate(left+1, right, n, cur, res);
        cur.pop();

        if left > right {
            cur.push(')');
            Self::generate(left, right+1, n, cur, res);
            cur.pop();
        }
    }
}
