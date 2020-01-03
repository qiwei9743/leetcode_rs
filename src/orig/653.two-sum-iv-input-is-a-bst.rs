/*
 * @lc app=leetcode id=653 lang=rust
 *
 * [653] Two Sum IV - Input is a BST
 *
 * https://leetcode.com/problems/two-sum-iv-input-is-a-bst/description/
 *
 * algorithms
 * Easy (53.09%)
 * Total Accepted:    110K
 * Total Submissions: 204.5K
 * Testcase Example:  '[5,3,6,2,4,null,7]\n9'
 *
 * Given a Binary Search Tree and a target number, return true if there exist
 * two elements in the BST such that their sum is equal to the given target.
 * 
 * Example 1:
 * 
 * 
 * Input: 
 * ⁠   5
 * ⁠  / \
 * ⁠ 3   6
 * ⁠/ \   \
 * 2   4   7
 * 
 * Target = 9
 * 
 * Output: True
 * 
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 
 * ⁠   5
 * ⁠  / \
 * ⁠ 3   6
 * ⁠/ \   \
 * 2   4   7
 * 
 * Target = 28
 * 
qo * Output: False
 *nn 
 * 
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

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut dummy = Rc::new(RefCell::new(TreeNode::new(-1)));
        dummy.as_ref().borrow_mut().left = root.clone();
        dummy.as_ref().borrow_mut().right = root.clone();
        let (mut ls, mut rs) = (vec![dummy.clone()], vec![dummy.clone()]);
        Self::next(&mut ls, true);
        Self::next(&mut rs, false);
        while !ls.is_empty() && !rs.is_empty() {
            if ls.last().unwrap().as_ptr() == rs.last().unwrap().as_ptr() { return false; }
            let t = ls.last().unwrap().borrow().val + rs.last().unwrap().borrow().val;
            match t.cmp(&k) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => Self::next(&mut ls, true),
                std::cmp::Ordering::Greater => Self::next(&mut rs, false),
            }
        }
        false
    }
    fn next(stack: &mut Vec<Rc<RefCell<TreeNode>>>, is_left: bool) {
        let mut root = if is_left { stack.pop().unwrap().borrow().right.clone() }
        else { stack.pop().unwrap().borrow().left.clone() };
        while let Some(rc_root) = root {
            stack.push(rc_root.clone());
            root = if is_left { rc_root.borrow().left.clone() } else { rc_root.borrow().right.clone() }
        }
    }
}
