/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (30.63%)
 * Total Accepted:    790.9K
 * Total Submissions: 2.6M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order and each of their nodes
 * contain a single digit. Add the two numbers and return it as a linked list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 * 
 * Example:
 * 
 * 
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 * 
 * 
 */
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0i32;
        let mut new_head = Some(Box::new(ListNode::new(-1)));
        let mut tail = &mut new_head;
        loop {
            match (l1, l2) {
                (None, None) => break,
                (b1, b2) => {
                    let x = if let Some(boxed) = b1 {
                        let ret = boxed.val;
                        l1 = boxed.next;
                        ret
                    } else { l1 = None; 0 };
                    let y = if let Some(boxed) = b2 {
                        let ret = boxed.val;
                        l2 = boxed.next;
                        ret
                    } else { l2 = None; 0 };

                    let mut r = x + y + carry;
                    if r > 9 {
                        carry = 1;
                        r -= 10;
                    } else {
                        carry = 0;
                    }
                    let new_node = Some(Box::new(ListNode::new(r)));
                    tail.as_mut().unwrap().next = new_node;
                    tail = &mut tail.as_mut().unwrap().next;
                }
            }
        }
        if carry > 0 {
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
        }
        new_head.unwrap().next
    }

}
