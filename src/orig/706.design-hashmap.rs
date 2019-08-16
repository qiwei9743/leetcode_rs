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
use std::cell::RefCell;

#[derive(Copy, Clone)]
struct Element(i32, i32);


struct MyHashMap {
    arr: RefCell<Box<[Element; 10000]>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{arr: RefCell::new(Box::new([Element(-1, -1); 10000]))}
    }
    
    /** value will always be non-negative. */
    fn put(&self, key: i32, value: i32) {
        //self.arr.borrow_mut().insert(key as usize, value);
        let elem_num = self.arr.borrow().len();
        for off in 0..elem_num {
            let index = (key as usize + off) % elem_num;
            let mut arr = self.arr.borrow_mut();
            if arr[index].0 == -1 || arr[index].0 == key {
                arr[index] = Element(key, value);
                break;
            }
        }
    }
    
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        if let Some(index) = self._search(key) {
            return self.arr.borrow()[index].1;
        }
        -1
    }

    fn _search(&self, key:i32) -> Option<usize> {
        let elem_num = self.arr.borrow().len();
        let arr = self.arr.borrow();
        for off in 0..elem_num {
            let index = (key as usize + off) % elem_num;
            if arr[index].0 == key {
                return Some(index);
            }
            if arr[index].0 == -1 {
                break
            }
        }
        None
    }
    
    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&self, key: i32) {
        if let Some(index) = self._search(key) {
            self.arr.borrow_mut()[index].0 = -1;
        }
    }
}


#[cfg(test)]
mod test {
    use super::MyHashMap;

    #[test]
    fn test1() {
        let mut mhm = MyHashMap::new();
        mhm.put(10, 30);
        assert_eq!(mhm.get(10), 30);
        mhm.remove(10);

        assert_eq!(mhm.get(11), -1);
        assert_eq!(mhm.get(10), -1);
    }
}