/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
 *
 * https://leetcode.com/problems/binary-tree-preorder-traversal/description/
 *
 * algorithms
 * Medium (53.60%)
 * Likes:    1113
 * Dislikes: 48
 * Total Accepted:    420K
 * Total Submissions: 782.6K
 * Testcase Example:  '[1,null,2,3]'
 *
 * Given a binary tree, return the preorder traversal of its nodes' values.
 * 
 * Example:
 * 
 * 
 * Input: [1,null,2,3]
 * ⁠  1
 * ⁠   \
 * ⁠    2
 * ⁠   /
 * ⁠  3
 * 
 * Output: [1,2,3]
 * 
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut res = vec![];
        let mut stack = vec![AccessOrder::Root(root.unwrap())];
        while let Some(ac_root) = stack.pop() {
            match ac_root {
                AccessOrder::Root(root) => {
                    res.push(root.borrow().val);
                    stack.push(AccessOrder::Left(root.clone()));
                },
                AccessOrder::Left(root) => {
                    stack.push(AccessOrder::Right(root.clone()));
                    if let Some(ref child) = root.borrow().left {
                        stack.push(AccessOrder::Root(child.clone()));
                    }
                },
                AccessOrder::Right(root) => {
                    if let Some(ref child) = root.borrow().right {
                        stack.push(AccessOrder::Root(child.clone()));
                    }
                }
            }
        }
        res
    }
}
// @lc code=end
