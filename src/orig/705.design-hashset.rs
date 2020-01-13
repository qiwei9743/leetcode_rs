/*
 * @lc app=leetcode id=705 lang=rust
 *
 * [705] Design HashSet
 *
 * https://leetcode.com/problems/design-hashset/description/
 *
 * algorithms
 * Easy (54.60%)
 * Total Accepted:    25.3K
 * Total Submissions: 46.1K
 * Testcase Example:  '["MyHashSet","add","add","contains","contains","add","contains","remove","contains"]\n[[],[1],[2],[1],[3],[2],[2],[2],[2]]'
 *
 * Design a HashSet without using any built-in hash table libraries.
 * 
 * To be specific, your design should include these functions:
 * 
 * 
 * add(value): Insert a value into the HashSet. 
 * contains(value) : Return whether the value exists in the HashSet or not.
 * remove(value): Remove a value in the HashSet. If the value does not exist in
 * the HashSet, do nothing.
 * 
 * 
 * 
 * Example:
 * 
 * 
 * MyHashSet hashSet = new MyHashSet();
 * hashSet.add(1);         
 * hashSet.add(2);         
 * hashSet.contains(1);    // returns true
 * hashSet.contains(3);    // returns false (not found)
 * hashSet.add(2);          
 * hashSet.contains(2);    // returns true
 * hashSet.remove(2);          
 * hashSet.contains(2);    // returns false (already removed)
 * 
 * 
 * 
 * Note:
 * 
 * 
 * All values will be in the range of [0, 1000000].
 * The number of operations will be in the range of [1, 10000].
 * Please do not use the built-in HashSet library.
 * 
 * 
 */

struct MyHashSet {
    arr: Vec<Slist<i32>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    /** Initialize your data structure here. */
    #[allow(dead_code)]
    fn new() -> Self {
        let cnt = 1024;
        let mut arr = Vec::with_capacity(cnt);
        for _ in 0..cnt {
            arr.push(Slist::new());
        }
        Self { arr }
    }
    #[allow(dead_code)]
    fn calc_index(arr_size: usize, key: i32) -> usize {
        key as usize & (arr_size - 1)
    }
    #[allow(dead_code)]
    fn add(&mut self, key: i32) {
        let index = Self::calc_index(self.arr.len(), key);
        let sl = &mut self.arr[index];
        sl.remove(|x| *x == key);
        sl.append(key);
    }
    #[allow(dead_code)]
    fn remove(&mut self, key: i32) {
        let index = Self::calc_index(self.arr.len(), key);
        let sl = &mut self.arr[index];
        sl.remove(|x| *x == key );
    }
    
    /** Returns true if this set contains the specified element */
    #[allow(dead_code)]
    fn contains(&self, key: i32) -> bool {
        let index = Self::calc_index(self.arr.len(), key);
        let sl = &self.arr[index];
        sl.iter().any(|x| *x == key)
    }
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Slist<T> {
    head: Link<T>
}

impl<T> Default for Slist<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Slist<T> {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { head: None }
    }
    #[allow(dead_code)]
    fn peek(&self) -> Option<&T> {
        if let Some(node) = self.head.as_ref() {
            return Some(&node.value)
        }
        None
    }
    #[allow(dead_code)]
    fn append(&mut self, elem: T) {
        let new_node = Box::new(
            Node{ value: elem, next: self.head.take() });
        self.head = Some(new_node);
    }
    #[allow(dead_code)]
    fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next;
        }
    }
    #[allow(dead_code)]
    fn remove<F>(&mut self, f: F) where
        F: Fn(&T) -> bool {

        let mut curr = &mut self.head;
        loop {
            match curr {
                None => return,
                Some(node) if f(&node.value) => {
                    *curr = node.next.take();
                    return
                }
                Some(node) => {
                    curr = &mut node.next;
                }
            }
        }
    }
    #[allow(dead_code)]
    fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|x| &**x)}
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next.take() {
            self.next = node.next.as_ref().map(|x| &**x);
            return Some(&node.value);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slist() {
        let mut sl = Slist::new();
        sl.append(3);
        assert_eq!(sl.peek(), Some(&3));
        sl.append(2);
        assert_eq!(sl.peek(), Some(&2));
        sl.append(4);
        assert_eq!(sl.peek(), Some(&4));
        sl.pop();
        assert_eq!(sl.peek(), Some(&2));
        sl.pop();
        assert_eq!(sl.peek(), Some(&3));
        sl.pop();
        assert_eq!(sl.peek(), None);
        sl.append(10);
        sl.append(12);
        sl.append(13);
        sl.append(14);
        sl.remove(|x| *x == 10);
        assert_eq!(sl.iter().find(|x| **x == 10), None);
    }

    #[test]
    fn test_hashset() {
        let mut hs = MyHashSet::new();
        hs.add(10);
        hs.add(10);
        hs.add(11);
        hs.add(12);
        hs.add(13);
        assert_eq!(hs.contains(10), true);
        assert_eq!(hs.contains(11), true);
        assert_eq!(hs.contains(12), true);
        assert_eq!(hs.contains(13), true);
        hs.remove(10);
        assert_eq!(hs.contains(10), false);
        assert_eq!(hs.contains(11), true);
        assert_eq!(hs.contains(12), true);
        assert_eq!(hs.contains(13), true);
    }
}
