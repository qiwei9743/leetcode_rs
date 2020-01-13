/*
 * @lc app=leetcode id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
 *
 * https://leetcode.com/problems/swap-nodes-in-pairs/description/
 *
 * algorithms
 * Medium (45.56%)
 * Total Accepted:    338.7K
 * Total Submissions: 741.2K
 * Testcase Example:  '[1,2,3,4]'
 *
 * Given a linked list, swap every two adjacent nodes and return its head.
 * 
 * You may not modify the values in the list's nodes, only nodes itself may be
 * changed.
 * 
 * 
 * 
 * Example:
 * 
 * 
 * Given 1->2->3->4, you should return the list as 2->1->4->3.
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

#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
use crate::ListNode;

impl Solution {
    #[allow(dead_code)]
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut i, n) = (0, 2);
        let mut dummy = Some(Box::new(ListNode::new(-1)));
        let mut refdummy = &mut dummy;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = refdummy.as_mut().unwrap().next.take();
            refdummy.as_mut().unwrap().next = Some(node);
            i += 1;
            if i == n {
                while refdummy.as_ref().unwrap().next.is_some() {
                    refdummy = &mut refdummy.as_mut().unwrap().next;
                }
                i = 0;
            }
        }

        dummy.unwrap().next
    }
}
