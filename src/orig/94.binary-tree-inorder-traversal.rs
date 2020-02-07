/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
 *
 * https://leetcode.com/problems/binary-tree-inorder-traversal/description/
 *
 * algorithms
 * Medium (60.20%)
 * Likes:    2363
 * Dislikes: 98
 * Total Accepted:    605.4K
 * Total Submissions: 1M
 * Testcase Example:  '[1,null,2,3]'
 *
 * Given a binary tree, return the inorder traversal of its nodes' values.
 * 
 * Example:
 * 
 * 
 * Input: [1,null,2,3]
 * ⁠  1
 * ⁠   \
 * ⁠    2
 * ⁠   /
 * ⁠  3
 * 
 * Output: [1,3,2]
 * 
 * Follow up: Recursive solution is trivial, could you do it iteratively?
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

enum AccessOrder {
    Left(Rc<RefCell<TreeNode>>),
    Root(Rc<RefCell<TreeNode>>),
    Right(Rc<RefCell<TreeNode>>),
}

use crate::TreeNode;
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut res = vec![];
        let mut stack = vec![AccessOrder::Left(root.as_ref().unwrap().clone())];
        while let Some(ao_root) = stack.pop() {
            match ao_root {
                AccessOrder::Left(root) => {
                    stack.push(AccessOrder::Root(root.clone()));
                    if root.borrow().left.is_some() {
                        stack.push(AccessOrder::Left(root.borrow().left.as_ref().unwrap().clone()));
                    }
                },
                AccessOrder::Root(root) => {
                    res.push(root.borrow().val);
                    if root.borrow().right.is_some() {
                        stack.push(AccessOrder::Left(root.borrow().right.as_ref().unwrap().clone()));
                    }
                },
                AccessOrder::Right(root) => (),
            }
        }
        res
    }
}
// @lc code=end
