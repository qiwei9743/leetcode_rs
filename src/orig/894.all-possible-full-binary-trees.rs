/*
 * @lc app=leetcode id=894 lang=rust
 *
 * [894] All Possible Full Binary Trees
 *
 * https://leetcode.com/problems/all-possible-full-binary-trees/description/
 *
 * algorithms
 * Medium (71.56%)
 * Total Accepted:    20.8K
 * Total Submissions: 29.1K
 * Testcase Example:  '7'
 *
 * A full binary tree is a binary tree where each node has exactly 0 or 2
 * children.
 * 
 * Return a list of all possible full binary trees with N nodes.  Each element
 * of the answer is the root node of one possible tree.
 * 
 * Each node of each tree in the answer must have node.val = 0.
 * 
 * You may return the final list of trees in any order.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: 7
 * Output:
 * [[0,0,0,null,null,0,0,null,null,0,0],[0,0,0,null,null,0,0,0,0],[0,0,0,0,0,0,0],[0,0,0,0,0,null,null,null,null,0,0],[0,0,0,0,0,null,null,0,0]]
 * Explanation:
 * 
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= N <= 20
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
use crate::TreeNode;

struct Solution;

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
        }
        let mut res = vec![];
        for i in 1..n-1 {
            for left in Self::all_possible_fbt(i).iter() {
                for right in Self::all_possible_fbt(n-1-i).iter() {

                    let mut root = TreeNode::new(0);
                    root.left = Some(left.as_ref().unwrap().clone());
                    root.right = Some(right.as_ref().unwrap().clone());

                    res.push(Some(Rc::new(RefCell::new(root))));
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        println!("hello");
    }
    #[test]
    fn test2() {

    }
}