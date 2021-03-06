/*
 * @lc app=leetcode id=225 lang=rust
 *
 * [225] Implement Stack using Queues
 *
 * https://leetcode.com/problems/implement-stack-using-queues/description/
 *
 * algorithms
 * Easy (40.36%)
 * Total Accepted:    141.3K
 * Total Submissions: 347.4K
 * Testcase Example:  '["MyStack","push","push","top","pop","empty"]\n[[],[1],[2],[],[],[]]'
 *
 * Implement the following operations of a stack using queues.
 * 
 * 
 * push(x) -- Push element x onto stack.
 * pop() -- Removes the element on top of the stack.
 * top() -- Get the top element.
 * empty() -- Return whether the stack is empty.
 * 
 * 
 * Example:
 * 
 * 
 * MyStack stack = new MyStack();
 * 
 * stack.push(1);
 * stack.push(2);  
 * stack.top();   // returns 2
 * stack.pop();   // returns 2
 * stack.empty(); // returns false
 * 
 * Notes:
 * 
 * 
 * You must use only standard operations of a queue -- which means only push to
 * back, peek/pop from front, size, and is empty operations are valid.
 * Depending on your language, queue may not be supported natively. You may
 * simulate a queue by using a list or deque (double-ended queue), as long as
 * you use only standard operations of a queue.
 * You may assume that all operations are valid (for example, no pop or top
 * operations will be called on an empty stack).
 * 
 * 
 */

struct MyStack {
    q: std::collections::VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    #[allow(dead_code)]
    fn new() -> Self {
        Self{q: std::collections::VecDeque::new()}
    }
    
    /** Push element x onto stack. */
    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.q.push_back(x);
        for _ in 0..self.q.len()-1 {
            let x = self.q.pop_front().unwrap();
            self.q.push_back(x);
        }
    }
    
    /** Removes the element on top of the stack and returns that element. */
    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }
    
    /** Get the top element. */
    #[allow(dead_code)]
    fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }
    
    /** Returns whether the stack is empty. */
    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}
