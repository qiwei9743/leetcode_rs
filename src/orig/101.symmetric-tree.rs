/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
 *
 * https://leetcode.com/problems/symmetric-tree/description/
 *
 * algorithms
 * Easy (44.09%)
 * Total Accepted:    444.3K
 * Total Submissions: 1M
 * Testcase Example:  '[1,2,2,3,4,4,3]'
 *
 * Given a binary tree, check whether it is a mirror of itself (ie, symmetric
 * around its center).
 * 
 * For example, this binary tree [1,2,2,3,4,4,3] is symmetric:
 * 
 * 
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠/ \ / \
 * 3  4 4  3
 * 
 * 
 * 
 * 
 * But the following [1,2,2,null,3,null,3] is not:
 * 
 * 
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠  \   \
 * ⁠  3    3
 * 
 * 
 * 
 * 
 * Note:
 * Bonus points if you could solve it both recursively and iteratively.
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
    #[allow(dead_code)]
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(rc_root) = &root {
            Self::is_tree_equal(&rc_root.borrow().left,
                                &rc_root.borrow().right)
        } else {
            true
        }
    }
    fn is_tree_equal(root1: &Option<Rc<RefCell<TreeNode>>>,
                     root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {

        match (root1, root2) {
            (Some(rc_root1), Some(rc_root2)) if rc_root1.borrow().val == rc_root2.borrow().val => {
                Self::is_tree_equal(
                    &rc_root1.borrow().left,
                    &rc_root2.borrow().right ) && Self::is_tree_equal(
                    &rc_root1.borrow().right, &rc_root2.borrow().left)
            },
            (None, None) => true,
            _ => false
        }
    }
}
