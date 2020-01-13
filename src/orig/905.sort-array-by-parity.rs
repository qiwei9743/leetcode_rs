/*
 * @lc app=leetcode id=905 lang=rust
 *
 * [905] Sort Array By Parity
 *
 * https://leetcode.com/problems/sort-array-by-parity/description/
 *
 * algorithms
 * Easy (72.65%)
 * Total Accepted:    142.9K
 * Total Submissions: 195.4K
 * Testcase Example:  '[3,1,2,4]'
 *
 * Given an array A of non-negative integers, return an array consisting of all
 * the even elements of A, followed by all the odd elements of A.
 * 
 * You may return any answer array that satisfies this condition.
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: [3,1,2,4]
 * Output: [2,4,3,1]
 * The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= A.length <= 5000
 * 0 <= A[i] <= 5000
 * 
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        if a.is_empty() { return a; }
        let (mut low, mut high) = (0, a.len()-1);
        while low < high {
            while low < high && a[high] % 2 != 0 { high -= 1; }
            while low < high && a[low] % 2 == 0 { low += 1; }
            if low < high { a.swap(low, high); }
        }
        a
    }
}
