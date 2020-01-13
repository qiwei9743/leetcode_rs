/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 *
 * https://leetcode.com/problems/majority-element/description/
 *
 * algorithms
 * Easy (53.68%)
 * Total Accepted:    447.2K
 * Total Submissions: 821.2K
 * Testcase Example:  '[3,2,3]'
 *
 * Given an array of size n, find the majority element. The majority element is
 * the element that appears more than ⌊ n/2 ⌋ times.
 * 
 * You may assume that the array is non-empty and the majority element always
 * exist in the array.
 * 
 * Example 1:
 * 
 * 
 * Input: [3,2,3]
 * Output: 3
 * 
 * Example 2:
 * 
 * 
 * Input: [2,2,1,1,1,2,2]
 * Output: 2
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn majority_element2(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        for n in nums.iter() {
            let mut entry = counter.entry(n).or_insert(0);
            *entry += 1;
        }
        **counter.iter().map(|(k, v)| (v, k)).max().unwrap().1
    }
    #[allow(dead_code)]
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut np = -1;
        for i in nums.iter() {
            if cnt == 0 {
                cnt = 1;
                np = *i;
            } else if *i == np {
                cnt += 1
            } else {
                cnt -= 1;
            }
        }
        np
    }
}
