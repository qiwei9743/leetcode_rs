/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
 *
 * https://leetcode.com/problems/minimum-depth-of-binary-tree/description/
 *
 * algorithms
 * Easy (36.49%)
 * Likes:    1006
 * Dislikes: 581
 * Total Accepted:    362K
 * Total Submissions: 991.6K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * Given a binary tree, find its minimum depth.
 * 
 * The minimum depth is the number of nodes along the shortest path from the
 * root node down to the nearest leaf node.
 * 
 * Note: A leaf is a node with no children.
 * 
 * Example:
 * 
 * Given binary tree [3,9,20,null,null,15,7],
 * 
 * 
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 * 
 * return its minimum depth = 2.
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(ref root) = root {
            let mut res = std::i32::MAX;
            Self::dfs(root, &mut res, 1);
            res
        } else { 0 }
    }
    fn dfs(root: &Rc<RefCell<TreeNode>>, res: &mut i32, depth: i32) {
        match (&root.borrow().left, &root.borrow().right) {
            (Some(ref left), Some(ref right)) => {
                Self::dfs(left, res, depth+1);
                Self::dfs(right, res, depth+1);
            },
            (Some(ref left), None) => {
                Self::dfs(left, res, depth+1);
            },
            (None, Some(ref right)) => {
                Self::dfs(right, res, depth+1);
            },
            _ => *res = std::cmp::min(*res, depth),
        }
    }
}
// @lc code=end
