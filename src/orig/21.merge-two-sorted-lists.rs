/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 *
 * https://leetcode.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (46.29%)
 * Total Accepted:    584.6K
 * Total Submissions: 1.2M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * Merge two sorted linked lists and return it as a new list. The new list
 * should be made by splicing together the nodes of the first two lists.
 * 
 * Example:
 * 
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
 * 
 * 
 */
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

struct Solution;
use crate::ListNode;

impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//        let mut head = Some(Box::new(ListNode::new(-1)));
//        let mut tail = &mut head;
        let mut head = Box::new(ListNode::new(-1));
        let mut tail = &mut head;
        loop {
            match (l1, l2) {
                (Some(mut n1), Some(mut n2)) => {
                    tail.next = if n1.val < n2.val {
                        l1 = n1.next.take();
                        l2 = Some(n2);
                        Some(n1)
                    } else {
                        l1 = Some(n1);
                        l2 = n2.next.take();
                        Some(n2)
                    };
                    tail = tail.next.as_mut().unwrap();
                },
                (Some(mut n1), None) => {
                    l1 = n1.next.take();
                    l2 = None;
                    tail.next = Some(n1);
                    tail = tail.next.as_mut().unwrap();
                }
                (None, Some(mut n2)) => {
                    l1 = None;
                    l2 = n2.next.take();
                    tail.next = Some(n2);
                    tail = tail.next.as_mut().unwrap();
                },
                (None, None) => break,
            }
        }
        head.next
    }
}
