/*
 * @lc app=leetcode id=145 lang=rust
 *
 * [145] Binary Tree Postorder Traversal
 *
 * https://leetcode.com/problems/binary-tree-postorder-traversal/description/
 *
 * algorithms
 * Hard (51.79%)
 * Likes:    1323
 * Dislikes: 69
 * Total Accepted:    323K
 * Total Submissions: 621.4K
 * Testcase Example:  '[1,null,2,3]'
 *
 * Given a binary tree, return the postorder traversal of its nodes' values.
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
 * Output: [3,2,1]
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


use crate::TreeNode;

enum Access {
    Left(Rc<RefCell<TreeNode>>),
    Right(Rc<RefCell<TreeNode>>),
    Root(Rc<RefCell<TreeNode>>),
}

use std::rc::Rc;
use std::cell::RefCell;

#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut stack = vec![Access::Left(root.unwrap())];
        let mut res = vec![];
        while let Some(ac_root) = stack.pop() {
            match ac_root {
                Access::Left(root) => {
                    stack.push(Access::Right(root.clone()));
                    if let Some(ref child) = root.borrow().left {
                        stack.push(Access::Left(child.clone()));
                    }
                },
                Access::Right(root) => {
                    stack.push(Access::Root(root.clone()));
                    if let Some(ref child) = root.borrow().right {
                        stack.push(Access::Left(child.clone()));
                    }
                },
                Access::Root(root) => {
                    res.push(root.borrow().val);
                }
            }
        }
        res
    }
}
// @lc code=end
