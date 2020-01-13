/*
 * @lc app=leetcode id=264 lang=rust
 *
 * [264] Ugly Number II
 *
 * https://leetcode.com/problems/ugly-number-ii/description/
 *
 * algorithms
 * Medium (37.10%)
 * Total Accepted:    122K
 * Total Submissions: 321K
 * Testcase Example:  '10'
 *
 * Write a program to find the n-th ugly number.
 * 
 * Ugly numbers are positive numbers whose prime factors only include 2, 3,
 * 5. 
 * 
 * Example:
 * 
 * 
 * Input: n = 10
 * Output: 12
 * Explanation: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10
 * ugly numbers.
 * 
 * Note:  
 * 
 * 
 * 1 is typically treated as an ugly number.
 * n does not exceed 1690.
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn nth_ugly_number(n: i32) -> i32 {
        let (mut nums, mut indexes, mut factors) = (vec![1; n as usize], vec![0, 0, 0], vec![2, 3, 5]);
        for i in 1..n as usize {
            nums[i] = indexes.iter().zip(factors.iter()).map(|(&i, &j)| nums[i]*j).min().unwrap();
            indexes.iter_mut().zip(factors.iter_mut()).for_each(|(x, y)| if nums[*x]*(*y) == nums[i] { *x += 1 });
        }
        *nums.last().unwrap()
    }
    #[allow(dead_code)]
    pub fn nth_ugly_number_heap(n: i32) -> i32 {
        use std::cmp::Reverse;
        let mut hp:std::collections::BinaryHeap<Reverse<i32>> = vec![1].into_iter()
            .map(|i| Reverse(i)).collect::<std::collections::BinaryHeap<_>>();
        for i in 0..n-1 {
            let Reverse(top) = hp.pop().unwrap();
            while Reverse(top) == *hp.peek().unwrap_or(&Reverse(std::i32::MAX)) { hp.pop(); };
            [2, 3, 5].into_iter().for_each(|x| {
                if let (n, false) = top.overflowing_mul(*x) {
                    hp.push(Reverse(n));
                }
            });
        }
        hp.pop().unwrap().0
    }
    #[allow(dead_code)]
    pub fn nth_ugly_number_naive(n: i32) -> i32 {
        let mut ugset = vec![1,2,3,4,5,6];
        if n <= ugset.len() as i32 { return ugset[n as usize -1]; }
        let mut cn = ugset.last().unwrap() + 1;
        let mut i = ugset.len() + 1;
        while i <= n as usize {
            let mut num = cn;
            loop {
                num = if num % 2 == 0 { num / 2 }
                else if num % 3 == 0 { num / 3 }
                else if num % 5 == 0 { num / 5 }
                else { break; };
                if let Ok(_) = ugset.binary_search(&num) {
                    ugset.push(cn);
                    i += 1;
                    break;
                }
            }
            cn += 1;
        }
        cn - 1
    }
}
