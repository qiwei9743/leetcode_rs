/*
 * @lc app=leetcode id=315 lang=rust
 *
 * [315] Count of Smaller Numbers After Self
 *
 * https://leetcode.com/problems/count-of-smaller-numbers-after-self/description/
 *
 * algorithms
 * Hard (40.32%)
 * Likes:    1700
 * Dislikes: 67
 * Total Accepted:    104.8K
 * Total Submissions: 258.8K
 * Testcase Example:  '[5,2,6,1]'
 *
 * You are given an integer array nums and you have to return a new counts
 * array. The counts array has the property where counts[i] is the number of
 * smaller elements to the right of nums[i].
 * 
 * Example:
 * 
 * 
 * Input: [5,2,6,1]
 * Output: [2,1,1,0] 
 * Explanation:
 * To the right of 5 there are 2 smaller elements (2 and 1).
 * To the right of 2 there is only 1 smaller element (1).
 * To the right of 6 there is 1 smaller element (1).
 * To the right of 1 there is 0 smaller element.
 * 
 */

// @lc code=start

#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums.into_iter().map(|_| 0).collect();
        }
        let mut res = vec![];
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        //let mut st = SegTree::new(*min, (max-min+1) as usize);
        let mut st = BinaryIndexedTree::new(*min, (max-min+1) as usize);

        for n in nums.iter().rev() {
            res.push(st.query_count_of_lessthan(*n));
            st.update(*n);
        }
        res.into_iter().rev().collect()
    }
}

struct SegTree {
    arr: Vec<i32>,
    min: i32,
    leaf_cnt: usize,
}

impl SegTree {
    fn new(min: i32, n: usize) -> Self {
        let size = 2*2i32.pow((n as f64).log2().ceil() as u32) - 1;
        //println!("min={}, n={}, size={}", min, n, size);

        Self {
            arr: vec![0; size as usize],
            min,
            leaf_cnt: n,
        }
    }
    fn query_count_of_lessthan(&self, num: i32) -> i32 {
        if num == self.min {
            0
        } else {
            let ret = self._query((0..self.leaf_cnt), 0,
                                  &(0..(num-self.min) as usize));
            ret
        }
    }
    fn _query(&self, root: std::ops::Range<usize>, arri: usize,
              query: &std::ops::Range<usize>) -> i32 {
        //println!("start root={:?} query ={:?}, arri={}", root, query, arri);

        if query.end <= root.start || root.end <= query.start {
            return 0;
        }
        if query.start <= root.start && root.end <= query.end {
            return self.arr[arri];
        }
        let mid = self.get_mid(&root);
        self._query((root.start..mid), 2*arri+1, query)
            + self._query((mid..root.end), 2*arri+2, query)
    }
    fn update(&mut self, n: i32) {
        self._update((0..self.leaf_cnt), 0, (n - self.min) as usize);
    }
    fn _update(&mut self, root: std::ops::Range<usize>, arri: usize, i: usize) {
        if root.end - root.start == 1 {
            self.arr[arri] += 1;
            return;
        }

        let mid = self.get_mid(&root);
        if i < mid {
            self._update((root.start..mid), arri*2+1, i);
        } else {
            self._update((mid..root.end), arri*2+2, i);
        }
        self.arr[arri] += 1;
    }
    fn get_mid(&self, range: &std::ops::Range<usize>) -> usize {
        range.start + (range.end - range.start) / 2
    }
}

struct BinaryIndexedTree {
    arr: Vec<i32>,
    min: i32,
}

impl BinaryIndexedTree {
    fn new(min: i32, size: usize) -> Self {
        Self {
            arr: vec![0; size + 1],
            min,
        }
    }
    fn query_count_of_lessthan(&self, n: i32) -> i32 {
        if n == self.min {
            0
        } else {
            let ret = self.query_sum(n-self.min);
            ret
        }
    }

    fn query_sum(&self, mut i: i32) -> i32 {
        let mut res = 0;
        while i > 0 {
            res += self.arr[i as usize];
            i -= self.lowbit(i);
        }
        res
    }
    fn update(&mut self, n: i32) {
        let mut i = (n - self.min + 1) as i32;
        while i < self.arr.len() as i32 {
            self.arr[i as usize] += 1;
            i += self.lowbit(i);
        }
    }
    fn lowbit(&self, n: i32) -> i32 {
        n & -n
    }
}


// @lc code=end
