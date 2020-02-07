/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
 *
 * https://leetcode.com/problems/validate-binary-search-tree/description/
 *
 * algorithms
 * Medium (26.99%)
 * Likes:    2989
 * Dislikes: 430
 * Total Accepted:    565.9K
 * Total Submissions: 2.1M
 * Testcase Example:  '[2,1,3]'
 *
 * Given a binary tree, determine if it is a valid binary search tree (BST).
 * 
 * Assume a BST is defined as follows:
 * 
 * 
 * The left subtree of a node contains only nodes with keys less than the
 * node's key.
 * The right subtree of a node contains only nodes with keys greater than the
 * node's key.
 * Both the left and right subtrees must also be binary search trees.
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * ⁠   2
 * ⁠  / \
 * ⁠ 1   3
 * 
 * Input: [2,1,3]
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * ⁠   5
 * ⁠  / \
 * ⁠ 1   4
 * / \
 * 3   6
 * 
 * Input: [5,1,4,null,null,3,6]
 * Output: false
 * Explanation: The root node's value is 5 but its right child's value is 4.
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
type OptionNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_valid_bst(root: OptionNode) -> bool {
        Self::helper(&root, std::i64::MIN, std::i64::MAX)
    }

    fn helper(root: &OptionNode, min: i64, max: i64) -> bool {
        if let Some(ref rc_root) = root {
            let val = rc_root.borrow().val as i64;
            if !(min < val && val < max) { return false; }
            Self::helper(&rc_root.borrow().left, min, val) &&
                Self::helper(&rc_root.borrow().right, val, max)
        } else { true }
    }

    pub fn is_valid_bst2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(ref rc_root) = root {
            Self::is_valid(rc_root).0
        } else { true }
    }

    fn is_valid(rc_root: &Rc<RefCell<TreeNode>>) -> (bool, i32, i32) {
        let val = rc_root.borrow().val;
        let left = if let Some(ref rc_left) = rc_root.borrow().left {
            let p = Self::is_valid(rc_left);
            if !p.0 || !(p.2 < val) { return (false, 0, 0); }
            p.1
        } else {
            val
        };
        let right = if let Some(ref rc_right) = rc_root.borrow().right {
            let p = Self::is_valid(rc_right);
            if !p.0 || !(val < p.1) { return (false, 0, 0); }
            p.2
        } else {
            val
        };

        (true, left, right)
    }
}
// @lc code=end
