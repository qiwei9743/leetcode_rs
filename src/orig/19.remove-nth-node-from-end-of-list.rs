/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
 *
 * https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/
 *
 * algorithms
 * Medium (34.50%)
 * Total Accepted:    476.2K
 * Total Submissions: 1.4M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * Given a linked list, remove the n-th node from the end of list and return
 * its head.
 * 
 * Example:
 * 
 * 
 * Given linked list: 1->2->3->4->5, and n = 2.
 * 
 * After removing the second node from the end, the linked list becomes
 * 1->2->3->5.
 * 
 * 
 * Note:
 * 
 * Given n will always be valid.
 * 
 * Follow up:
 * 
 * Could you do this in one pass?
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
use crate::ListNode;

#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        dummy.next = head;

        let mut sdummy = Some(Box::new(dummy));
        let mut count = -1;
        let mut rdummy = &mut sdummy;
        while let Some(node) = rdummy {
            count += 1;
            rdummy = &mut node.next;
        }

        rdummy = &mut sdummy;
        for _ in 0..count-n {
            rdummy = &mut rdummy.as_mut().unwrap().next;
        }
        let next = rdummy.as_mut().unwrap().next.take();
        rdummy.as_mut().unwrap().next = next.unwrap().next.take();

        sdummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkedlist;
    #[test]
    fn test1() {

        assert_eq!(
            Solution::remove_nth_from_end(linkedlist![1,2,3,4,5], 2),
            linkedlist![1,2,3,5]);
    }
}
