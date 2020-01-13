/*
 * @lc app=leetcode id=25 lang=rust
 *
 * [25] Reverse Nodes in k-Group
 *
 * https://leetcode.com/problems/reverse-nodes-in-k-group/description/
 *
 * algorithms
 * Hard (37.48%)
 * Total Accepted:    215.8K
 * Total Submissions: 561K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * Given a linked list, reverse the nodes of a linked list k at a time and
 * return its modified list.
 * 
 * k is a positive integer and is less than or equal to the length of the
 * linked list. If the number of nodes is not a multiple of k then left-out
 * nodes in the end should remain as it is.
 * 
 * 
 * 
 * 
 * Example:
 * 
 * Given this linked list: 1->2->3->4->5
 * 
 * For k = 2, you should return: 2->1->4->3->5
 * 
 * For k = 3, you should return: 3->2->1->4->5
 * 
 * Note:
 * 
 * 
 * Only constant extra memory is allowed.
 * You may not alter the values in the list's nodes, only nodes itself may be
 * changed.
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
use crate::{ListNode};

#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    fn find_last<'a>(mut x: &'a mut Option<Box<ListNode>>, mut cnt: Option<&mut i32>) -> &'a mut Option<Box<ListNode>> {
        while x.as_ref().unwrap().next.is_some() {
            x = &mut x.as_mut().unwrap().next;
            //cnt.as_mut().map(|x| **x += 1);
            if let Some(x) = cnt.as_mut() { **x += 1; }
        }
        x
    }
    #[allow(dead_code)]
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut total_cnt = 1;
        Self::find_last(&mut head, Some(&mut total_cnt));
        total_cnt = total_cnt / k * k;


        let mut dummy = Some(Box::new(ListNode::new(-1)));
        let mut h = &mut dummy;
        let mut cnt = 0;

        while let Some(mut node) = head {
            if cnt == total_cnt {
                h = Self::find_last(h, None);
                h.as_mut().unwrap().next = Some(node);
                break;
            }
            head = node.next.take();

            if cnt % k == 0 {
                h = Self::find_last(h, None);
            }
            node.next = h.as_mut().unwrap().next.take();
            h.as_mut().unwrap().next = Some(node);
            cnt += 1;
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_k_group() {
        use crate::linkedlist;
        assert_eq!(
            Solution::reverse_k_group(linkedlist![1,2,3,4], 2),
            linkedlist![2,1,4,3]);
    }
}
