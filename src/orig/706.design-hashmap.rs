/*
 * @lc app=leetcode id=706 lang=rust
 *
 * [706] Design HashMap
 *
 * https://leetcode.com/problems/design-hashmap/description/
 *
 * algorithms
 * Easy (56.67%)
 * Total Accepted:    41.7K
 * Total Submissions: 73.5K
 * Testcase Example:  '["MyHashMap","put","put","get","get","put","get", "remove", "get"]\n[[],[1,1],[2,2],[1],[3],[2,1],[2],[2],[2]]'
 *
 * Design a HashMap without using any built-in hash table libraries.
 * 
 * To be specific, your design should include these functions:
 * 
 * 
 * put(key, value) : Insert a (key, value) pair into the HashMap. If the value
 * already exists in the HashMap, update the value.
 * get(key): Returns the value to which the specified key is mapped, or -1 if
 * this map contains no mapping for the key.
 * remove(key) : Remove the mapping for the value key if this map contains the
 * mapping for the key.
 * 
 * 
 * 
 * Example:
 * 
 * 
 * MyHashMap hashMap = new MyHashMap();
 * hashMap.put(1, 1);          
 * hashMap.put(2, 2);         
 * hashMap.get(1);            // returns 1
 * hashMap.get(3);            // returns -1 (not found)
 * hashMap.put(2, 1);          // update the existing value
 * hashMap.get(2);            // returns 1 
 * hashMap.remove(2);          // remove the mapping for 2
 * hashMap.get(2);            // returns -1 (not found) 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * All keys and values will be in the range of [0, 1000000].
 * The number of operations will be in the range of [1, 10000].
 * Please do not use the built-in HashMap library.
 * 
 * 
 */

struct MyHashMap {
    arr: Vec<SingleList<Element>>,
}

struct Element(i32, i32);

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    /** Initialize your data structure here. */
    #[allow(dead_code)]
    fn new() -> Self {
        let mut array = vec![];
        for _ in 0..1024 {
            array.push(SingleList::new());
            //array[i] = SingleList::new();
        }
        Self { arr: array }
    }
    
    /** value will always be non-negative. */
    #[allow(dead_code)]
    fn put(&mut self, key: i32, value: i32) {
        //if sl.iter().find(|x| x.0 == key).is_none() {
        self.remove(key);

        let sl = self.get_mut_bucket(key);
        sl.push(Element(key, value));
        //}
    }
    
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    #[allow(dead_code)]
    fn get(&mut self, key: i32) -> i32 {
        let sl = self.get_mut_bucket(key);
        if let Some(t) = sl.iter().find(|x| x.0 == key) {
            return t.1;
        }
        -1
    }
    
    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    #[allow(dead_code)]
    fn remove(&mut self, key: i32) {
        let sl = self.get_mut_bucket(key);
        let mut current = &mut sl.head;
        loop {
            match current {
                None => return,
                Some(node) if node.value.0 == key => {
                    *current = node.next.take();
                    return
                },
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }
    #[allow(dead_code)]
    fn get_bucket(&self, key: i32) -> &SingleList<Element> {
        let pos = key as usize & (self.arr.len()-1);
        &self.arr[pos]
    }

    #[allow(dead_code)]
    fn get_mut_bucket(&mut self, key: i32) -> &mut SingleList<Element> {
        let pos = key as usize & (self.arr.len()-1);
        &mut self.arr[pos]
    }
}

struct SingleList<T> {
    pub head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    pub value: T,
    pub next: Link<T>
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node{ value, next: None }
    }
}


impl<T> SingleList<T> {
    #[allow(dead_code)]
    fn new() -> Self{
        SingleList{head: None}
    }
    #[allow(dead_code)]
    fn push(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }
    #[allow(dead_code)]
    fn pop(&mut self) -> Option<T> {
        if let Some(box_node) = self.head.take() {
            self.head = box_node.next;
            return Some(box_node.value);
        }
        None
    }
    #[allow(dead_code)]
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.value)
    }
    #[allow(dead_code)]
    fn remove(&mut self) {
        unimplemented!()
    }
    #[allow(dead_code)]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    #[allow(dead_code)]
    fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|x| &**x) }
    }
    #[allow(dead_code)]
    fn itermut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|x| &mut **x)}
    }
}

impl<T> Drop for SingleList<T> {
    fn drop(&mut self) {
        while let Some(current) = self.head.take() {
            self.head = current.next;
        }
    }
}

struct IntoIter<T>(SingleList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|x|{
            self.next = x.next.as_ref().map(|x| &**x);
            &x.value
        })
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|x| {
            self.next = x.next.as_mut().map(|node| &mut **node);
            &mut x.value
        })
    }
}

#[cfg(test)]
mod test {
    use super::MyHashMap;
    use crate::design_hashmap::SingleList;

    #[test]
    fn test1() {
        let mut mhm = MyHashMap::new();
        mhm.put(10, 30);
        assert_eq!(mhm.get(10), 30);
        mhm.remove(10);

        assert_eq!(mhm.get(11), -1);
        assert_eq!(mhm.get(10), -1);
        mhm.put(10, 30);
        assert_eq!(mhm.get(10), 30);
        mhm.put(10, 31);
        assert_eq!(mhm.get(10), 31);
        mhm.remove(10);
        assert_eq!(mhm.get(10), -1);
    }

    #[test]
    fn test_myhashmap() {
        /*
        ["MyHashMap","remove","get","put","put","put","get","put","put","put","put"]
[[],[14],[4],[7,3],[11,1],[12,1],[7],[1,19],[0,3],[1,8],[2,6]]
        */
        let mut mhm = MyHashMap::new();
        mhm.remove(14);
        assert_eq!(mhm.get(4), -1);
        mhm.put(7, 3);
        mhm.put(11, 1);
        mhm.put(12, 1);
        //assert_eq!(mhm.get(7), 3);
        assert_eq!(mhm.get(16384), -1);
    }
    #[test]
    fn test_list() {
        let mut list = SingleList::new();
        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        list.push(4);
        assert_eq!(list.peek(), Some(&4));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.peek(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = SingleList::new();
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
    }
    #[test]
    fn test_itermut() {
        let mut list = SingleList::new();
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        let mut itermut = list.itermut();
        assert_eq!(itermut.next(), Some(&mut 5));
        assert_eq!(itermut.next(), Some(&mut 4));
        assert_eq!(itermut.next(), Some(&mut 3));
        assert_eq!(itermut.next(), Some(&mut 2));
    }
}
