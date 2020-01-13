/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
 *
 * https://leetcode.com/problems/binary-tree-level-order-traversal/description/
 *
 * algorithms
 * Medium (49.34%)
 * Total Accepted:    411.4K
 * Total Submissions: 830.4K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, return the level order traversal of its nodes' values.
 * (ie, from left to right, level by level).
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
 * return its level order traversal as:
 * 
 * [
 * ⁠ [3],
 * ⁠ [9,20],
 * ⁠ [15,7]
 * ]
 * 
 * 
 */
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

use std::collections::VecDeque;

use crate::TreeNode;
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut dq = VecDeque::new();
        let mut res = vec![];
        dq.push_back(root);
        while ! dq.is_empty() {
            let size = dq.len();
            let mut lr = vec![];
            for _ in 0..size {
                let r = dq.pop_front().unwrap();
                if let Some(ref rc) = r.as_ref().unwrap().borrow().left {
                    dq.push_back(Some(Rc::clone(rc)));
                }
                if let Some(ref rc) = r.as_ref().unwrap().borrow().right {
                    dq.push_back(Some(Rc::clone(rc)));
                }
                lr.push(r.as_ref().unwrap().borrow().val);
            }
            res.push(lr);
        }
        res
    }
}
