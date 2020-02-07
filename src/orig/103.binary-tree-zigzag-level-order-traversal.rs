/*
 * @lc app=leetcode id=103 lang=rust
 *
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/description/
 *
 * algorithms
 * Medium (44.96%)
 * Likes:    1512
 * Dislikes: 84
 * Total Accepted:    297.9K
 * Total Submissions: 662K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, return the zigzag level order traversal of its nodes'
 * values. (ie, from left to right, then right to left for the next level and
 * alternate between).
 * 
 * 
 * For example:
 * Given binary tree [3,9,20,null,null,15,7],
 * 
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 * 
 * 
 * 
 * return its zigzag level order traversal as:
 * 
 * [
 * ⁠ [3],
 * ⁠ [20,9],
 * ⁠ [15,7]
 * ]
 * 
 * 
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut positive = true;
        let mut q = std::collections::VecDeque::new();
        q.push_back(root.unwrap());
        let mut res = vec![];
        while !q.is_empty() {
            let mut new_q = std::collections::VecDeque::new();
            res.push(
                if positive {
                    q.iter().map(|root| root.borrow().val).collect();
                } else {
                    q.iter().rev().map(|root| root.borrow().val).collect();
                });

            for r in q {
                if let Some(ref rc_root) = r.borrow().left {
                    new_q.push_back(rc_root.clone());
                }

                if let Some(ref rc_root) = r.borrow().right {
                    new_q.push_back(rc_root.clone());
                }
            }

            positive = !positive;
            q = new_q;
        }
        res
    }
}
// @lc code=end
