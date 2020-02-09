/*
 * @lc app=leetcode id=109 lang=rust
 *
 * [109] Convert Sorted List to Binary Search Tree
 *
 * https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/description/
 *
 * algorithms
 * Medium (44.57%)
 * Likes:    1529
 * Dislikes: 79
 * Total Accepted:    215.1K
 * Total Submissions: 481.5K
 * Testcase Example:  '[-10,-3,0,5,9]'
 *
 * Given a singly linked list where elements are sorted in ascending order,
 * convert it to a height balanced BST.
 * 
 * For this problem, a height-balanced binary tree is defined as a binary tree
 * in which the depth of the two subtrees of every node never differ by more
 * than 1.
 * 
 * Example:
 * 
 * 
 * Given the sorted linked list: [-10,-3,0,5,9],
 * 
 * One possible answer is: [0,-3,9,-10,null,5], which represents the following
 * height balanced BST:
 * 
 * ⁠     0
 * ⁠    / \
 * ⁠  -3   9
 * ⁠  /   /
 * ⁠-10  5
 * 
 * 
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
type OptListNode = Option<Box<ListNode>>;
type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;

struct OptListNodeIterator<'a>(&'a OptListNode);
impl<'a> Iterator for OptListNodeIterator<'a> {
    type Item = &'a Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_ref().map(|ln| { self.0 = &ln.next; ln })
        // if let Some(ref ln) = self.0 {
        //     self.0 = &ln.next;
        //     Some(ln)
        // } else { None }
    }
}

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_new(&mut OptListNodeIterator(&head), OptListNodeIterator(&head).count())
    }
    fn construct_new(iter: &mut OptListNodeIterator, size: usize) -> OptTreeNode {
        if size == 0 { return None; }
        let left = Self::construct_new(iter, size / 2);
        let mut root = TreeNode::new(iter.next().unwrap().val);
        let right = Self::construct_new(iter, size - size / 2 - 1);
        root.left = left;
        root.right = right;
        Some(Rc::new(RefCell::new(root)))
    }



    // pub fn sorted_list_to_bst2(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     Self::construct(&head, Self::count_node(&head))
    // }
    // fn construct(head: &OptListNode, size: usize) -> OptTreeNode {
    //     if head.is_none() || size == 0 { return None; }
    //     let mid = Self::find_nth(head, size / 2);
    //     let mut root = TreeNode::new(mid.as_ref().unwrap().val);
    //     root.left = Self::construct(head, size / 2);
    //     root.right = Self::construct(&mid.as_ref().unwrap().next, size - size / 2 - 1);
    //     Some(Rc::new(RefCell::new(root)))
    // }
    // fn count_node(head: &OptListNode) -> usize {
    //     let mut p = head;
    //     let mut cnt = 0;
    //     while p.is_some() {
    //         p = &p.as_ref().unwrap().next;
    //         cnt += 1;
    //     }
    //     cnt
    // }
    // // n: [0, n)
    // fn find_nth(head: &OptListNode, n: usize) -> &OptListNode {
    //     let mut p = head;
    //     for _ in 0..n {
    //         p = &p.as_ref().unwrap().next;
    //     }
    //     p
    // }
}
// @lc code=end
