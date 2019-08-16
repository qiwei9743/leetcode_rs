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
// pub struct Solution;
impl Solution {
    pub fn merge_two_lists1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rl1 = &l1;
        let mut rl2 = &l2;

        let mut res = Some(Box::new(ListNode::new(0)));
        let mut refp = &mut res;
        while rl1.is_some() && rl2.is_some() {
            refp.as_mut().unwrap().next =
                if rl1.as_ref().unwrap().val < rl2.as_ref().unwrap().val {

                    let r = Some(rl1.as_ref().unwrap().clone());
                    rl1 = &rl1.as_ref().unwrap().next;
                    r
            } else {
                    let r = Some(rl2.as_ref().unwrap().clone());
                    rl2 = &rl2.as_ref().unwrap().next;
                    r
            };

            refp = &mut refp.as_mut().unwrap().next;
            //println!("{:?}", refp.as_ref().unwrap().val);
            refp.as_mut().unwrap().next = None;
        }

        while rl1.is_some() {
            refp.as_mut().unwrap().next = Some(rl1.as_ref().unwrap().clone());
            refp = &mut refp.as_mut().unwrap().next;
            rl1 = &rl1.as_ref().unwrap().next;
        }

        while rl2.is_some() {
            refp.as_mut().unwrap().next = Some(rl2.as_ref().unwrap().clone());
            refp = &mut refp.as_mut().unwrap().next;
            rl2 = &rl2.as_ref().unwrap().next;
        }

        res.unwrap().next
    }

    pub fn merge_two_lists2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = Some(Box::new(ListNode{val:0, next:None}));
        let mut tail = &mut res;
        let mut iter1 = ListNodeIterator(l1);
        let mut iter2 = ListNodeIterator(l2);
        let mut oln1: Option<Box<ListNode>> = None;
        let mut oln2: Option<Box<ListNode>> = None;
        loop {
            if oln1.is_none() {
                oln1 = iter1.next();
            }
            if oln2.is_none() {
                oln2 = iter2.next();
            }
            if oln1.is_some() && oln2.is_some() {
                if oln1.as_ref().unwrap().val < oln2.as_ref().unwrap().val {
                    tail.as_mut().unwrap().next = oln1.take();
                } else {
                    tail.as_mut().unwrap().next = oln2.take();
                }
                tail = &mut tail.as_mut().unwrap().next;
            } else {
                break
            }
            // if let (Some(ref ln1), Some(ref ln2)) = (oln1, oln2) {
            //     if ln1.val < ln2.val {
            //         tail.as_mut().unwrap().next = oln1.take();
            //     } else {
            //         tail.as_mut().unwrap().next = oln2.take();
            //     }
            // } else {
            //     break
            // }
        }

        while let Some(ln) = oln1 {
            //println!("{:?}", ln);
            tail.as_mut().unwrap().next = Some(ln);
            tail = &mut tail.as_mut().unwrap().next;
            oln1 = iter1.next();
        }
        while let Some(ln) = oln2 {
            //println!("{:?}", ln);
            tail.as_mut().unwrap().next = Some(ln);
            tail = &mut tail.as_mut().unwrap().next;
            oln2 = iter2.next();
        }

        res.unwrap().next
    }

    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        if let Some(mut n1) = l1 {
            if let Some(mut n2) = l2 {
                if n1.val < n2.val {
                    n1.next = Solution::merge_two_lists(n1.next, Some(n2));
                    return Some(n1)
                } else {
                    n2.next = Solution::merge_two_lists(Some(n1), n2.next);
                    return Some(n2)
                }
            }
            Some(n1)
        } else {
            l2
        }
    }
}

impl Iterator for ListNodeIterator {
    type Item = Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut ret) = self.0.take() {
            self.0 = ret.next.take();
            return Some(ret);
        }
        None
    }
}

struct ListNodeIterator(Option<Box<ListNode>>);
