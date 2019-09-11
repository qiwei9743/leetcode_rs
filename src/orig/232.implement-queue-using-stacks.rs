/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 *
 * https://leetcode.com/problems/implement-queue-using-stacks/description/
 *
 * algorithms
 * Easy (44.43%)
 * Total Accepted:    163.9K
 * Total Submissions: 366.3K
 * Testcase Example:  '["MyQueue","push","push","peek","pop","empty"]\n[[],[1],[2],[],[],[]]'
 *
 * Implement the following operations of a queue using stacks.
 * 
 * 
 * push(x) -- Push element x to the back of queue.
 * pop() -- Removes the element from in front of queue.
 * peek() -- Get the front element.
 * empty() -- Return whether the queue is empty.
 * 
 * 
 * Example:
 * 
 * 
 * MyQueue queue = new MyQueue();
 * 
 * queue.push(1);
 * queue.push(2);  
 * queue.peek();  // returns 1
 * queue.pop();   // returns 1
 * queue.empty(); // returns false
 * 
 * Notes:
 * 
 * 
 * You must use only standard operations of a stack -- which means only push to
 * top, peek/pop from top, size, and is empty operations are valid.
 * Depending on your language, stack may not be supported natively. You may
 * simulate a stack by using a list or deque (double-ended queue), as long as
 * you use only standard operations of a stack.
 * You may assume that all operations are valid (for example, no pop or peek
 * operations will be called on an empty queue).
 * 
 * 
 */
struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{ s1: Vec::new(), s2: Vec::new() }
    }
    
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }
    
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        self.s2.pop().unwrap_or_else(|| {
            while let Some(x) = self.s1.pop() {
                self.s2.push(x);
            }
            self.s2.pop().unwrap()
        })
    }
    
    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if self.s2.is_empty() {
            while let Some(x) = self.s1.pop() {
                self.s2.push(x);
            }
        }
        self.s2.last().unwrap().clone()
    }
    
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}

