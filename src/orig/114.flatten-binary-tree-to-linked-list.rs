/*
 * @lc app=leetcode id=114 lang=rust
 *
 * [114] Flatten Binary Tree to Linked List
 *
 * https://leetcode.com/problems/flatten-binary-tree-to-linked-list/description/
 *
 * algorithms
 * Medium (43.61%)
 * Total Accepted:    256.6K
 * Total Submissions: 587.3K
 * Testcase Example:  '[1,2,5,3,4,null,6]'
 *
 * Given a binary tree, flatten it to a linked list in-place.
 * 
 * For example, given the following tree:
 * 
 * 
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   5
 * ⁠/ \   \
 * 3   4   6
 * 
 * 
 * The flattened tree should look like:
 * 
 * 
 * 1
 * ⁠\
 * ⁠ 2
 * ⁠  \
 * ⁠   3
 * ⁠    \
 * ⁠     4
 * ⁠      \
 * ⁠       5
 * ⁠        \
 * ⁠         6
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
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
use crate::TreeNode;


use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Clone)]
enum Rotation {
    LEFT,
    RIGHT,
}


impl Rotation {
    #[allow(dead_code)]
    fn opposite(&self) -> Self {
        match self {
            Rotation::LEFT => Rotation::RIGHT,
            Rotation::RIGHT => Rotation::LEFT,
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root.clone();
        while root.is_some() {
            let mut lright_most = root.as_ref().unwrap().borrow_mut().left.take();
            if lright_most.is_some() {
                let right = root.as_ref().unwrap().borrow_mut().right.take();
                root.as_ref().unwrap().borrow_mut().right = lright_most.clone();
                while lright_most.as_ref().unwrap().borrow().right.is_some() {
                    let rt = lright_most.as_ref().unwrap().borrow().right.clone();
                    lright_most = rt;
                }
                lright_most.as_ref().unwrap().borrow_mut().right = right;
            }
            let right = root.as_ref().unwrap().borrow().right.clone();
            root = right;
        }
    }
    #[allow(dead_code)]
    pub fn flatten_misunderstand_by_order(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        *root = Self::helper(root).0;
    }
    fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>)
              -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {  (None, None) }
        else {
            let (ll, lr) = Self::helper(&mut root.as_ref().unwrap().borrow_mut().left);
            let (rl, rr) = Self::helper(&mut root.as_ref().unwrap().borrow_mut().right);
            if lr.is_some() { lr.as_ref().unwrap().borrow_mut().right = root.clone(); }
            root.as_ref().unwrap().borrow_mut().left = None;
            root.as_ref().unwrap().borrow_mut().right = rl.clone();
            (ll.or(root.clone()), rr.or(root.clone()))
        }
    }
    #[allow(dead_code)]
    pub fn flatten_loop1(mut root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root.clone();

        while let Some(ref mut rt) = root.clone() {
            let mut rt_bow = rt.borrow_mut();
            if let Some(left) = rt_bow.left.take() {
                let mut right_most = Some(left.clone());
                while let Some(mut right) = right_most.clone().unwrap().borrow_mut().right.clone() {
                    right_most = Some(right);
                }
                right_most.as_ref().unwrap().borrow_mut().right = rt_bow.right.take();
                rt_bow.right = Some(left.clone());
            }
            root = rt_bow.right.clone();
        }


        // Solution 2, but not accepted becaure incorrect node order.
        // if root.is_some() {
        //     Self::_flatten(root, Rotation::RIGHT);
        // }
    }
    // fn _flatten(root: &mut Option<Rc<RefCell<TreeNode>>>, rot: Rotation) {
    //     if root.as_ref().unwrap().as_ref().borrow_mut().left.is_some() {
    //         Self::_flatten(&mut root.as_ref().unwrap().as_ref().borrow_mut().left, Rotation::LEFT);
    //     }
    //     if root.as_ref().unwrap().as_ref().borrow_mut().right.is_some() {
    //         Self::_flatten(&mut root.as_ref().unwrap().as_ref().borrow_mut().right, Rotation::RIGHT);
    //     }
    //     let rotate_fn: fn (root: &mut Option<Rc<RefCell<TreeNode>>>) = match rot {
    //         Rotation::LEFT => Self::rotate_left,
    //         Rotation::RIGHT => Self::rotate_right,
    //     };
    //     while match rot {
    //         Rotation::LEFT => root.as_ref().unwrap().borrow().right.is_some(),
    //         Rotation::RIGHT => root.as_ref().unwrap().borrow().left.is_some(),
    //     } {
    //         rotate_fn(root);
    //     }
    // }

    // fn rotate_left(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    //     if root.as_ref().unwrap().as_ref().borrow_mut().right.is_some() {
    //         let old_right = root.as_ref().unwrap().as_ref().borrow_mut().right.take();
    //         let old_root = std::mem::replace(root, old_right);
    //         old_root.as_ref().unwrap().as_ref().borrow_mut().right = root.as_ref().unwrap().as_ref().borrow_mut().left.take();
    //         root.as_ref().unwrap().as_ref().borrow_mut().left = old_root;
    //     }
    // }
    // fn rotate_right(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    //     if root.as_ref().unwrap().as_ref().borrow_mut().left.is_some() {
    //         let old_left = root.as_ref().unwrap().as_ref().borrow_mut().left.take();
    //         let old_root = std::mem::replace(root, old_left);
    //         old_root.as_ref().unwrap().as_ref().borrow_mut().left = root.as_ref().unwrap().as_ref().borrow_mut().right.take();
    //         root.as_ref().unwrap().as_ref().borrow_mut().right = old_root;
    //     }
    // }
}
