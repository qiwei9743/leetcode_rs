/*
 * @lc app=leetcode id=707 lang=rust
 *
 * [707] Design Linked List
 *
 * https://leetcode.com/problems/design-linked-list/description/
 *
 * algorithms
 * Easy (21.26%)
 * Total Accepted:    31.4K
 * Total Submissions: 148.8K
 * Testcase Example:  '["MyLinkedList","addAtHead","addAtTail","addAtIndex","get","deleteAtIndex","get"]\n[[],[1],[3],[1,2],[1],[1],[1]]'
 *
 * Design your implementation of the linked list. You can choose to use the
 * singly linked list or the doubly linked list. A node in a singly linked list
 * should have two attributes: val and next. val is the value of the current
 * node, and next is a pointer/reference to the next node. If you want to use
 * the doubly linked list, you will need one more attribute prev to indicate
 * the previous node in the linked list. Assume all nodes in the linked list
 * are 0-indexed.
 * 
 * Implement these functions in your linked list class:
 * 
 * 
 * get(index) : Get the value of the index-th node in the linked list. If the
 * index is invalid, return -1.
 * addAtHead(val) : Add a node of value val before the first element of the
 * linked list. After the insertion, the new node will be the first node of the
 * linked list.
 * addAtTail(val) : Append a node of value val to the last element of the
 * linked list.
 * addAtIndex(index, val) : Add a node of value val before the index-th node in
 * the linked list. If index equals to the length of linked list, the node will
 * be appended to the end of linked list. If index is greater than the length,
 * the node will not be inserted. If index is negative, the node will be
 * inserted at the head of the list.
 * deleteAtIndex(index) : Delete the index-th node in the linked list, if the
 * index is valid.
 * 
 * 
 * Example:
 * 
 * 
 * MyLinkedList linkedList = new MyLinkedList();
 * linkedList.addAtHead(1);
 * linkedList.addAtTail(3);
 * linkedList.addAtIndex(1, 2);  // linked list becomes 1->2->3
 * linkedList.get(1);            // returns 2
 * linkedList.deleteAtIndex(1);  // now the linked list is 1->3
 * linkedList.get(1);            // returns 3
 * 
 * 
 * Note:
 * 
 * 
 * All values will be in the range of [1, 1000].
 * The number of operations will be in the range of [1, 1000].
 * Please do not use the built-in LinkedList library.
 * 
 * 
 */

use std::rc::{Rc, Weak};
use std::cell::{RefCell, Cell};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T: std::fmt::Debug> {
    pub elem: T,
    pub prev: Weak<RefCell<Node<T>>>,
    pub next: Link<T>,
}

impl<T: std::fmt::Debug> Drop for Node<T> {
    fn drop(&mut self) {
        println!("droping {:?}", self.elem);
    }
}

pub struct MyLinkedList {
    len: Cell<usize>,
    head: Link<i32>,
    tail: Link<i32>,
}
impl MyLinkedList {
    pub fn new() -> Self {
        Self {len: Cell::new(0), head: None, tail: None}
    }

    pub fn get(&self, index: i32) -> i32 {
        self.locate(index).map_or(-1, |x|
        x.as_ref().borrow().elem)
    }

    pub fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    pub fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.len.get() as i32, val);
    }

    pub fn add_at_index(&mut self, mut index: i32, val: i32) {
        if index < 0 {
            index = 0;
        }
        if index > self.len.get() as i32 {
            return
        }

        let next = self.locate(index);
        let prev = if next.is_some() {
            next.as_ref().map(|x| x.borrow().prev.upgrade()).unwrap_or(None)
        } else {
            self.locate(index-1)
        };
        let mut new_node = Rc::new(
            RefCell::new(Node{elem: val, prev: Weak::new(), next: None}));

        if let Some(rc_prev) = prev {
            new_node.as_ref().borrow_mut().prev = Rc::downgrade(&rc_prev);
            rc_prev.as_ref().borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }

        if let Some(rc_next) = next {
            new_node.as_ref().borrow_mut().next = Some(rc_next.clone());
            rc_next.as_ref().borrow_mut().prev = Rc::downgrade(&new_node);
        } else {
            self.tail = Some(new_node.clone());
        }

        self.len.set(self.len.get() + 1);
    }

    pub fn delete_at_index(&mut self, index: i32) {
        let deleted = self.locate(index);
        if deleted.is_none() {
            return
        }
        let deleted = deleted.unwrap();
        let prev = deleted.as_ref().borrow_mut().prev.upgrade();
        let next = &deleted.as_ref().borrow_mut().next;

        if let Some(rc_prev) = &prev {
            rc_prev.as_ref().borrow_mut().next = next.as_ref().map(|x| x.clone());
        } else {
            self.head = next.as_ref().map(|x| x.clone());
        }

        if let Some(rc_next) = next {
            rc_next.as_ref().borrow_mut().prev = prev.as_ref().map(
                |x| Rc::downgrade(x)).unwrap_or(Weak::new());
        } else {
            self.tail = prev.as_ref().map(|x| x.clone());
        }

        self.len.set(self.len.get() - 1);
    }
    fn locate(&self, index: i32) -> Option<Rc<RefCell<Node<i32>>>> {
        if index < 0 || index >= self.len.get() as i32 {
            return None;
        }
        let mid = self.len.get() / 2;
        if index < mid as i32 {
            let mut cur = self.head.clone();
            let mut i = 0;
            while let Some(node) = cur.clone() {
                if i == index {
                    break
                }
                cur = node.as_ref().borrow().next.as_ref().map(|x| Rc::clone(x));
                i += 1;
            }
            cur
        } else {
            let mut cur = self.tail.clone();
            let mut i = self.len.get() - 1;
            while let Some(node) = cur.clone() {
                if i == index as usize {
                    break
                }
                cur = node.as_ref().borrow().prev.upgrade();
                //cur = node.as_ref().borrow().prev.upgrade().unwrap().as_ref().map(|x| Rc::clone(x));
                i -= 1;
            }
            cur
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len.get() == 0
    }
}

//impl Drop for MyLinkedList {
//    fn drop(&mut self) {
//        while self.len.get() > 0 {
//            self.delete_at_index(0);
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::MyLinkedList;

    #[test]
    fn test_add_head_delete0() {
        let mut ll = MyLinkedList::new();
        assert_eq!(ll.get(1), -1);
        ll.add_at_head(12);
        assert_eq!(ll.get(0), 12);
        ll.add_at_head(11);
        ll.add_at_head(10);
        ll.delete_at_index(0);
        ll.delete_at_index(0);
        ll.delete_at_index(0);
        ll.delete_at_index(0);
        //assert!(ll.is_empty());
    }
    #[test]
    fn test_add_tail() {
        let mut ll = MyLinkedList::new();
        ll.add_at_tail(1);
        ll.add_at_tail(2);
        assert_eq!(ll.get(0), 1);
        assert_eq!(ll.get(1), 2);
    }
    #[test]
    fn test_delete_at_index() {
        let mut ll = MyLinkedList::new();
        ll.add_at_head(1);
        ll.add_at_tail(3);
        ll.add_at_index(1, 2);
        assert_eq!(ll.get(0), 1);
        assert_eq!(ll.get(1), 2);
        assert_eq!(ll.get(2), 3);
        ll.delete_at_index(1);
        assert_eq!(ll.get(0), 1);
        assert_eq!(ll.get(1), 3);
    }
    #[test]
    fn test_lc4(){
        let mut ll = MyLinkedList::new();
        /*
          ✘ testcase: '["MyLinkedList","addAtHead","addAtHead","deleteAtIndex","addAtIndex","addAtHead","addAtHead","addAtHead","get","addAtTail","addAtIndex","addAtHead"]
                                      [[],[5],[2],[1],[1,9],[4],[9],[8],[3],[1],[3,6],[3]]'
        */
        ll.add_at_head(5);
        ll.add_at_head(2);
        ll.delete_at_index(1);
        ll.add_at_index(1, 9);
        ll.add_at_head(4);
        ll.add_at_head(9);
        ll.add_at_head(8);
        ll.get(3);
        ll.add_at_tail(1);
        ll.add_at_index(3, 6);
        ll.add_at_head(3)
    }
    #[test]
    fn test_lc60() {
        /*
        '["MyLinkedList","addAtIndex","get","deleteAtIndex"]\n[[],[-1,0],[0],[-1]]'
        */
        let mut ll = MyLinkedList::new();
        ll.add_at_index(-1, 0);
        ll.get(0);
        ll.delete_at_index(-1);
    }

    #[test]
    fn test_drop() {
        let mut ll = MyLinkedList::new();
        for i in 1..10 {
            ll.add_at_tail(i);
        }
    }
}
