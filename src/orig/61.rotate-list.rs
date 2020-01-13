/*
 * @lc app=leetcode id=61 lang=rust
 *
 * [61] Rotate List
 *
 * https://leetcode.com/problems/rotate-list/description/
 *
 * algorithms
 * Medium (27.75%)
 * Total Accepted:    222K
 * Total Submissions: 785.2K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * Given a linked list, rotate the list to the right by k places, where k is
 * non-negative.
 * 
 * Example 1:
 * 
 * 
 * Input: 1->2->3->4->5->NULL, k = 2
 * Output: 4->5->1->2->3->NULL
 * Explanation:
 * rotate 1 steps to the right: 5->1->2->3->4->NULL
 * rotate 2 steps to the right: 4->5->1->2->3->NULL
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 0->1->2->NULL, k = 4
 * Output: 2->0->1->NULL
 * Explanation:
 * rotate 1 steps to the right: 2->0->1->NULL
 * rotate 2 steps to the right: 1->2->0->NULL
 * rotate 3 steps to the right: 0->1->2->NULL
 * rotate 4 steps to the right: 2->0->1->NULL
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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut cnt = 0;
        let mut th = &head;
        while th.is_some() {
            th = &th.as_ref().unwrap().next;
            cnt += 1;
        }
        let k = k % cnt;
        if k < 1 {
            return head;
        }
        let p = cnt - k;
        let mut th = &mut head;
        for _ in 0..p-1 {
            th = &mut th.as_mut().unwrap().next;
        }
        let mut new_head = th.as_mut().unwrap().next.take();
        let mut ref_new = &mut new_head;
        while ref_new.is_some() && ref_new.as_ref().unwrap().next.is_some() {
            ref_new = &mut ref_new.as_mut().unwrap().next;
        }
        //ref_new.as_mut().map(|x| x.next = head);
        if let Some(x) = ref_new.as_mut() { x.next = head }

        new_head
    }
}
