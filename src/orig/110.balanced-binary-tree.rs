/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
 *
 * https://leetcode.com/problems/balanced-binary-tree/description/
 *
 * algorithms
 * Easy (42.44%)
 * Likes:    1734
 * Dislikes: 146
 * Total Accepted:    394.9K
 * Total Submissions: 930.1K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, determine if it is height-balanced.
 * 
 * For this problem, a height-balanced binary tree is defined as:
 * 
 * 
 * a binary tree in which the left and right subtrees of every node differ in
 * height by no more than 1.
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * Given the following tree [3,9,20,null,null,15,7]:
 * 
 * 
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 * 
 * Return true.
 * 
 * Example 2:
 * 
 * Given the following tree [1,2,2,3,3,null,null,4,4]:
 * 
 * 
 * ⁠      1
 * ⁠     / \
 * ⁠    2   2
 * ⁠   / \
 * ⁠  3   3
 * ⁠ / \
 * ⁠4   4
 * 
 * 
 * Return false.
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
type OptNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(rc_root) = root { Self::check_balance(&rc_root).0 } else { true }
    }
    fn check_balance(root: &Rc<RefCell<TreeNode>>) -> (bool, i32) {
        let left = if let Some(ref child) = root.borrow().left {
            let ret = Self::check_balance(child);
            if !ret.0 {return (false, 0);}
            ret.1
        } else { 0 };
        let right = if let Some(ref child) = root.borrow().right {
            let ret = Self::check_balance(child);
            if !ret.0 {return (false, 0);}
            ret.1
        } else { 0 };
        if (left - right).abs() > 1 {
            (false, 0)
        } else {
            (true, std::cmp::max(left, right) + 1)
        }
    }
}
// @lc code=end
