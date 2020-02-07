/*
 * @lc app=leetcode id=889 lang=rust
 *
 * [889] Construct Binary Tree from Preorder and Postorder Traversal
 *
 * https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/description/
 *
 * algorithms
 * Medium (64.16%)
 * Likes:    615
 * Dislikes: 41
 * Total Accepted:    28.2K
 * Total Submissions: 43.9K
 * Testcase Example:  '[1,2,4,5,3,6,7]\n[4,5,2,6,7,3,1]'
 *
 * Return any binary tree that matches the given preorder and postorder
 * traversals.
 * 
 * Values in the traversals pre and post are distinct positive integers.
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: pre = [1,2,4,5,3,6,7], post = [4,5,2,6,7,3,1]
 * Output: [1,2,3,4,5,6,7]
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= pre.length == post.length <= 30
 * pre[] and post[] are both permutations of 1, 2, ..., pre.length.
 * It is guaranteed an answer exists. If there exists multiple answers, you can
 * return any of them.
 * 
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
use std::collections::HashMap;
impl Solution {
    pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct(&pre[..], &post[..])
    }
    fn construct(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if pre.is_empty() {
            return None;
        }
        let mut root = TreeNode::new(pre[0]);
        if pre.len() > 1 {
            let left_root = pre[1];
            let idx_in_post = post.iter().position(|&x| x == left_root).unwrap();
            root.left = Self::construct(&pre[1..=idx_in_post+1], &post[..=idx_in_post]);
            root.right = Self::construct(&pre[idx_in_post+1+1..], &post[idx_in_post+1..post.len()-1]);
        }

        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
