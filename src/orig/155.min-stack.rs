/*
 * @lc app=leetcode id=155 lang=rust
 *
 * [155] Min Stack
 *
 * https://leetcode.com/problems/min-stack/description/
 *
 * algorithms
 * Easy (38.25%)
 * Total Accepted:    358.9K
 * Total Submissions: 916.2K
 * Testcase Example:  '["MinStack","push","push","push","getMin","pop","top","getMin"]\n[[],[-2],[0],[-3],[],[],[],[]]'
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum
 * element in constant time.
 * 
 * 
 * push(x) -- Push element x onto stack.
 * pop() -- Removes the element on top of the stack.
 * top() -- Get the top element.
 * getMin() -- Retrieve the minimum element in the stack.
 * 
 * 
 * 
 * 
 * Example:
 * 
 * 
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin();   --> Returns -3.
 * minStack.pop();
 * minStack.top();      --> Returns 0.
 * minStack.getMin();   --> Returns -2.
 * 
 * 
 * 
 * 
 */
struct MinStack {
    min_arr: Vec<i32>,
    arr: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack{ min_arr: Vec::new(), arr: Vec::new() }
    }
    
    fn push(&mut self, x: i32) {
        let last = self.min_arr.last().unwrap_or(&std::i32::MIN).clone();
        if x < last {
            self.min_arr.push(self.arr.len() as i32);
        }
        self.arr.push(x);
    }
    
    fn pop(&mut self) {
        self.arr.pop();
        if self.arr.len() as i32 == self.min_arr.last().unwrap_or(&-1).clone() {
            self.min_arr.pop();
        }
    }
    
    fn top(&self) -> i32 {
        self.arr.last().unwrap_or(&-1).clone()
    }
    
    fn get_min(&self) -> i32 {
        self.min_arr.last().map_or_else(|| -1, |&x|{
            self.arr[x as usize]
        })
    }
}
