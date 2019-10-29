/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
 *
 * https://leetcode.com/problems/merge-k-sorted-lists/description/
 *
 * algorithms
 * Hard (35.57%)
 * Total Accepted:    469.4K
 * Total Submissions: 1.3M
 * Testcase Example:  '[[1,4,5],[1,3,4],[2,6]]'
 *
 * Merge k sorted linked lists and return it as one sorted list. Analyze and
 * describe its complexity.
 * 
 * Example:
 * 
 * 
 * Input:
 * [
 * 1->4->5,
 * 1->3->4,
 * 2->6
 * ]
 * Output: 1->1->2->3->4->4->5->6
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

use std::collections::BinaryHeap;
#[derive(Eq, PartialEq)]
struct OrdNode(ListNode);

impl PartialOrd for OrdNode {
    fn partial_cmp(&self, other: &OrdNode) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdNode {
    fn cmp(&self, other: &OrdNode) -> std::cmp::Ordering {
        self.0.val.cmp(&other.0.val).reverse()
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = lists.into_iter()
            .filter_map(|x| x.map(|x| OrdNode(*x)))
            .collect::<BinaryHeap<_>>();
        let mut head = Some(Box::new(ListNode::new(-1)));
        let mut tail = &mut head;
        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.0.next.take() {
                heap.push(OrdNode(*next));
            }
            tail.as_mut().unwrap().next = Some(Box::new(node.0));
            tail = &mut tail.as_mut().unwrap().next;
        }

        head.unwrap().next
    }
}
