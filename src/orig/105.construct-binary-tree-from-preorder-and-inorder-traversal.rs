/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
 *
 * algorithms
 * Medium (45.32%)
 * Likes:    2527
 * Dislikes: 71
 * Total Accepted:    296.3K
 * Total Submissions: 653K
 * Testcase Example:  '[3,9,20,15,7]\n[9,3,15,20,7]'
 *
 * Given preorder and inorder traversal of a tree, construct the binary tree.
 * 
 * Note:
 * You may assume that duplicates do not exist in the tree.
 * 
 * For example, given
 * 
 * 
 * preorder = [3,9,20,15,7]
 * inorder = [9,3,15,20,7]
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct(&preorder, &mut 0, &inorder[..])
    }
    fn construct(preorder: &Vec<i32>, pi: &mut usize, inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if *pi >= preorder.len() || inorder.is_empty() { return None; }
        let root_num = preorder[*pi];
        let mut root = TreeNode::new(root_num);
        let root_idx_inorder = inorder.iter().position(|&x| x == root_num).unwrap();
        *pi += 1;

        root.left = Self::construct(preorder, pi, &inorder[..root_idx_inorder]);
        root.right = Self::construct(preorder, pi, &inorder[root_idx_inorder+1..]);
        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
