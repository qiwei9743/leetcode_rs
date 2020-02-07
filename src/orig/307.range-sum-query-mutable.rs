/*
 * @lc app=leetcode id=307 lang=rust
 *
 * [307] Range Sum Query - Mutable
 *
 * https://leetcode.com/problems/range-sum-query-mutable/description/
 *
 * algorithms
 * Medium (31.67%)
 * Likes:    987
 * Dislikes: 70
 * Total Accepted:    90.7K
 * Total Submissions: 285K
 * Testcase Example:  '["NumArray","sumRange","update","sumRange"]\n[[[1,3,5]],[0,2],[1,2],[0,2]]'
 *
 * Given an integer array nums, find the sum of the elements between indices i
 * and j (i ≤ j), inclusive.
 * 
 * The update(i, val) function modifies nums by updating the element at index i
 * to val.
 * 
 * Example:
 * 
 * 
 * Given nums = [1, 3, 5]
 * 
 * sumRange(0, 2) -> 9
 * update(1, 2)
 * sumRange(0, 2) -> 8
 * 
 * 
 * Note:
 * 
 * 
 * The array is only modifiable by the update function.
 * You may assume the number of calls to update and sumRange function is
 * distributed evenly.
 * 
 * 
 */

// @lc code=start
struct NumArray {
    st: SegTree
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        Self {
            st: SegTree::new(nums)
        }
    }
    
    fn update(&mut self, i: i32, val: i32) {
        self.st.update(i as usize, val);
    }
    
    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.st.query((i as usize..j as usize + 1))
    }
}

struct SegTree {
    arr: Vec<i32>,
    n: usize,
}

impl SegTree {
    pub fn new(nums: Vec<i32>) -> Self {
        let size = if nums.len() < 1 {
            0
        } else {
            2*2i32.pow((nums.len() as f64).log2().ceil() as u32) - 1
        };
//        let size = 4 * nums.len();
        let mut arr = vec![0; size as usize];
        if nums.len() > 0 {
            Self::_build((0..nums.len()), &mut arr, 0, &nums[..]);
        }
        println!("{} {:?}", size, arr);
        Self {
            arr,
            n: nums.len(),
        }
    }
    fn _build(root: std::ops::Range<usize>, arr: &mut [i32],arri: usize,
              input: &[i32]) -> i32 {
        if root.end - root.start == 1 {
            arr[arri] = input[root.start];
            return arr[arri];
        }
        let mid = Self::get_mid(&root);
        arr[arri] = Self::_build((root.start..mid),
                                 arr, arri*2+1, input)
            + Self::_build((mid..root.end),
                           arr, arri*2+2, input);
        arr[arri]
    }

    pub fn update(&mut self, i: usize, val: i32) {
        //println!("{:?}", self.arr);
        self._update((0..self.n), i, val, 0);
        //println!("{:?}", self.arr);
    }
    fn _update(&mut self, root: std::ops::Range<usize>, i: usize, val: i32, arri: usize) -> i32 {
        //println!("root range={:?}", root);
        if root.end - root.start == 1 {
            let diff = val - self.arr[arri];
            //println!("val={} old={} diff={}", val, self.arr[root.start], diff);
            self.arr[arri] = val;
            return diff
        }
        let mid = Self::get_mid(&root);
        let diff = if i < mid {
            self._update((root.start..mid), i, val, arri*2+1)
        } else {
            self._update((mid..root.end), i, val, arri*2+2)
        };
        self.arr[arri] += diff;
        diff
    }

    /*
    9
  1   8
 0 0 3 5
0



                       15
           3                      12
    1          2             3         9
   0  0       0 0          0  0       4  5



       2
   9       1
 0   9   5   10
0 0 0 0 0 0 0 73
    */
    pub fn query(&self, query: std::ops::Range<usize>) -> i32 {
        self._query((0..self.n), &query, 0)
    }
    fn _query(&self, root: std::ops::Range<usize>, query: &std::ops::Range<usize>,
              arri: usize) -> i32 {
        // println!("root={:?} query={:?}", root, query);
        if query.end <= root.start || root.end <= query.start {
            return 0;
        }
        if query.start <= root.start && root.end <= query.end {
            return self.arr[arri];
        }
        let mid = Self::get_mid(&root);
        self._query((root.start..mid), query, arri*2+1) +
            self._query((mid..root.end), query, arri*2+2)
    }

    fn get_mid(range: &std::ops::Range<usize>) -> usize {
        range.start + (range.end - range.start) / 2
    }
}


/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_sum_query() {
        let mut numarr = NumArray::new(vec![0,9,5,7,3]);
        numarr.sum_range(2, 4);
    }
}
