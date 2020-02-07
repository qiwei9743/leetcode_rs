/*
 * @lc app=leetcode id=106 lang=rust
 *
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
 *
 * https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/description/
 *
 * algorithms
 * Medium (42.78%)
 * Likes:    1236
 * Dislikes: 26
 * Total Accepted:    188.9K
 * Total Submissions: 441K
 * Testcase Example:  '[9,3,15,20,7]\n[9,15,7,20,3]'
 *
 * Given inorder and postorder traversal of a tree, construct the binary tree.
 * 
 * Note:
 * You may assume that duplicates do not exist in the tree.
 * 
 * For example, given
 * 
 * 
 * inorder = [9,3,15,20,7]
 * postorder = [9,15,7,20,3]
 * 
 * Return the following binary tree:
 * 
 * 
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
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
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut idx = postorder.len() as i32 - 1;

        Self::construct(&postorder, &mut idx, &inorder[..])
    }
    fn construct(postorder: &Vec<i32>, pi: &mut i32, inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if *pi < 0 || inorder.is_empty() { return None; }
        let root_num = postorder[*pi as usize];
        let mut root = TreeNode::new(root_num);
        let root_idx = inorder.iter().position(|&x| x == root_num).unwrap();
        *pi -= 1;
        root.right = Self::construct(postorder, pi, &inorder[root_idx+1..]);
        root.left = Self::construct(postorder, pi, &inorder[..root_idx]);
        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
