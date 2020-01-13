/*
 * @lc app=leetcode id=82 lang=rust
 *
 * [82] Remove Duplicates from Sorted List II
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/description/
 *
 * algorithms
 * Medium (33.80%)
 * Total Accepted:    208.2K
 * Total Submissions: 603.1K
 * Testcase Example:  '[1,2,3,3,4,4,5]'
 *
 * Given a sorted linked list, delete all nodes that have duplicate numbers,
 * leaving only distinct numbers from the original list.
 * 
 * Example 1:
 * 
 * 
 * Input: 1->2->3->3->4->4->5
 * Output: 1->2->5
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 1->1->1->2->3
 * Output: 2->3
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

use crate::ListNode;
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut delete_current = false;
        let mut mh = &mut head;

        while mh.as_ref().unwrap().next.is_some() {
            if mh.as_ref().unwrap().val == mh.as_ref().unwrap().next.as_ref().unwrap().val {
                delete_current = true;
                *mh = mh.as_mut().unwrap().next.take();
            } else if delete_current {
                *mh = mh.as_mut().unwrap().next.take();
                delete_current = false;
            } else {
                mh = &mut mh.as_mut().unwrap().next;
            }
        }

        if delete_current {
            *mh = mh.as_mut().unwrap().next.take();
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        use crate::linkedlist;

        assert_eq!(Solution::delete_duplicates(linkedlist![1,1,1]),
                   linkedlist![]);

        assert_eq!(Solution::delete_duplicates(linkedlist![1,2,2,3]),
                   linkedlist![1,3]);

        assert_eq!(Solution::delete_duplicates(linkedlist![]),
                   linkedlist![]);

        assert_eq!(Solution::delete_duplicates(linkedlist![1,1,2]),
                   linkedlist![2]);
    }
}
