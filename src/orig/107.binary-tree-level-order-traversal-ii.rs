/*
 * @lc app=leetcode id=107 lang=rust
 *
 * [107] Binary Tree Level Order Traversal II
 *
 * https://leetcode.com/problems/binary-tree-level-order-traversal-ii/description/
 *
 * algorithms
 * Easy (47.55%)
 * Total Accepted:    240.4K
 * Total Submissions: 503.6K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, return the bottom-up level order traversal of its
 * nodes' values. (ie, from left to right, level by level from leaf to root).
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
 * return its bottom-up level order traversal as:
 * 
 * [
 * ⁠ [15,7],
 * ⁠ [9,20],
 * ⁠ [3]
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

use crate::TreeNode;
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back(root.unwrap());
        let mut res = vec![];
        while !q.is_empty() {
            let size = q.len();
            let mut cur = vec![];
            for _ in 0..size {
                let root = q.pop_front().unwrap();
                if let Some(child) = root.borrow_mut().left.take() {
                    q.push_back(child);
                }
                if let Some(child) = root.borrow_mut().right.take() {
                    q.push_back(child);
                }
                cur.push(root.borrow().val);
            }
            res.push(cur);
        }
        res.into_iter().rev().collect()
    }

    pub fn level_order_bottom2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut dq = std::collections::VecDeque::new();
        if let Some(ref r) = root {
            dq.push_back(Rc::clone(r));
        } else {
            return res;
        }
        while ! dq.is_empty() {
            let cnt = dq.len();
            let mut level = vec![];
            for _ in 0..cnt {
                let rc_node = dq.pop_front().unwrap();
                level.push(rc_node.borrow().val);

                let tree_node = rc_node.borrow();
                level.push(tree_node.val);
                if let Some(n) = tree_node.left.as_ref() {
                    dq.push_back(Rc::clone(n));
                }
                /*
                if let Some(n) = tree_node.right.as_ref() {
                    dq.push_back(Rc::clone(n));
                }
                */
                if let Some(n) = rc_node.borrow().right.as_ref() {
                    dq.push_back(Rc::clone(n));
                };
            }
            res.push(level);
        }
        res.into_iter().rev().collect()
    }
}
