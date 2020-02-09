/*
 * @lc app=leetcode id=222 lang=rust
 *
 * [222] Count Complete Tree Nodes
 *
 * https://leetcode.com/problems/count-complete-tree-nodes/description/
 *
 * algorithms
 * Medium (39.94%)
 * Likes:    1508
 * Dislikes: 165
 * Total Accepted:    171.9K
 * Total Submissions: 429K
 * Testcase Example:  '[1,2,3,4,5,6]'
 *
 * Given a complete binary tree, count the number of nodes.
 * 
 * Note: 
 * 
 * Definition of a complete binary tree from Wikipedia:
 * In a complete binary tree every level, except possibly the last, is
 * completely filled, and all nodes in the last level are as far left as
 * possible. It can have between 1 and 2^h nodes inclusive at the last level
 * h.
 * 
 * Example:
 * 
 * 
 * Input: 
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   3
 * ⁠/ \  /
 * 4  5 6
 * 
 * Output: 6
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
    pub fn count_nodes(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cnt = 0;
        while let Some(rc) = root {
            let left_cnt = Self::count_left(rc.borrow().left.clone());
            let right_cnt = Self::count_left(rc.borrow().right.clone());
            cnt += 1 + match left_cnt.cmp(&right_cnt) {
                std::cmp::Ordering::Equal => {
                    root = rc.borrow_mut().right.take();
                    2i32.pow(left_cnt) - 1
                },
                std::cmp::Ordering::Greater => {
                    root = rc.borrow_mut().left.take();
                    2i32.pow(right_cnt) - 1
                },
                _ => unreachable!("left_cnt < right_cnt"),
            }
        }
        cnt
    }
    fn count_left(mut root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
        let mut cnt = 0;
        while let Some(rc) = root {
            cnt += 1;
            root = rc.borrow_mut().left.clone();
        }
        cnt
    }


    pub fn count_nodes3(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.as_ref()
            .map(|r| Self::count_nodes(r.borrow().left.clone()) + Self::count_nodes(r.borrow().right.clone()) + 1)
            .unwrap_or(0)
    }
    pub fn count_nodes2(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut left = Self::count_left_children(&root.as_ref().unwrap().borrow().left);
        let mut right = 0;
        let mut res = 0;
        while let Some(r) = root {
            right = Self::count_left_children(&r.borrow().right);
            match left.cmp(&right) {
                std::cmp::Ordering::Equal => {
                    res += 1 + 2i32.pow(left as u32) - 1;
                    left = right - 1;
                    root = r.borrow().right.clone();
                },
                std::cmp::Ordering::Greater => {
                    res += 1 + 2i32.pow(right as u32) - 1;
                    left -= 1;
                    root = r.borrow().left.clone();
                },
                std::cmp::Ordering::Less => panic!("invalid complete binary tree"),
            }
        }
        res
    }
    fn count_left_children(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.as_ref().map(|r| Self::count_left_children(&r.borrow().left) + 1).unwrap_or(0)
    }
}
// @lc code=end
