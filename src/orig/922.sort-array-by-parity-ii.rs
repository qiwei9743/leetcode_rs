/*
 * @lc app=leetcode id=922 lang=rust
 *
 * [922] Sort Array By Parity II
 *
 * https://leetcode.com/problems/sort-array-by-parity-ii/description/
 *
 * algorithms
 * Easy (67.25%)
 * Total Accepted:    66.4K
 * Total Submissions: 98.5K
 * Testcase Example:  '[4,2,5,7]'
 *
 * Given an array A of non-negative integers, half of the integers in A are
 * odd, and half of the integers are even.
 * 
 * Sort the array so that whenever A[i] is odd, i is odd; and whenever A[i] is
 * even, i is even.
 * 
 * You may return any answer array that satisfies this condition.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: [4,2,5,7]
 * Output: [4,5,2,7]
 * Explanation: [4,7,2,5], [2,5,4,7], [2,7,4,5] would also have been
 * accepted.
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 2 <= A.length <= 20000
 * A.length % 2 == 0
 * 0 <= A[i] <= 1000
 * 
 * 
 * 
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn sort_array_by_parity_ii(mut a: Vec<i32>) -> Vec<i32> {
        let even = a.iter().enumerate().step_by(2).filter_map(|(i, &elem)| if elem % 2 != 0 { Some(i) } else { None });
        let odd = a.iter().enumerate().skip(1).step_by(2).filter_map(|(i, &elem)| if elem % 2 == 0 { Some(i) } else { None } );
        let swap = even.zip(odd).collect::<Vec<_>>();
        swap.iter().for_each(|&(i, j)| a.swap(i, j));
        a
    }
}
