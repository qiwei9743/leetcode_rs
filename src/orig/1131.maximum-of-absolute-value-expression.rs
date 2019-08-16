/*
 * @lc app=leetcode id=1131 lang=rust
 *
 * [1131] Maximum of Absolute Value Expression
 *
 * https://leetcode.com/problems/maximum-of-absolute-value-expression/description/
 *
 * algorithms
 * Medium (49.55%)
 * Total Accepted:    2.4K
 * Total Submissions: 4.7K
 * Testcase Example:  '[1,2,3,4]\r\n[-1,4,5,6]\r'
 *
 * Given two arrays of integers with equal lengths, return the maximum value
 * of:
 * 
 * |arr1[i] - arr1[j]| + |arr2[i] - arr2[j]| + |i - j|
 * 
 * where the maximum is taken over all 0 <= i, j < arr1.length.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: arr1 = [1,2,3,4], arr2 = [-1,4,5,6]
 * Output: 13
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: arr1 = [1,-2,-5,0,10], arr2 = [0,-2,-1,-7,-4]
 * Output: 20
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 2 <= arr1.length == arr2.length <= 40000
 * -10^6 <= arr1[i], arr2[i] <= 10^6
 * 
 */
impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let iter = || {
            arr1.iter().zip(arr2.iter()).enumerate()
        };

        // let max1 = arr1.iter().zip(arr2.iter()).enumerate().map(|(i, a)| a.0+a.1+i as i32).max().unwrap();
        // let min1 = arr1.iter().zip(arr2.iter()).enumerate().map(|(i, a)| a.0+a.1+i as i32).min().unwrap();

        // let max2 = arr1.iter().zip(arr2.iter()).enumerate().map(|(i, a)| a.0-a.1+i as i32).max().unwrap();
        // let min2 = arr1.iter().zip(arr2.iter()).enumerate().map(|(i, a)| a.0-a.1+i as i32).min().unwrap();

        // let max3 = arr1.iter().zip(arr2.iter()).enumerate().map(|(i, a)| a.0+a.1-i as i32).max().unwrap();
        // let min3 = arr1.iter().zip(arr2.iter()).enumerate().map(|(i, a)| a.0+a.1-i as i32).min().unwrap();

        // let max4 = arr1.iter().zip(arr2.iter()).enumerate().map(|(i, a)| a.0-a.1-i as i32).max().unwrap();
        // let min4 = arr1.iter().zip(arr2.iter()).enumerate().map(|(i, a)| a.0-a.1-i as i32).min().unwrap();

        let max1 = iter().map(|(i, a)| a.0+a.1+i as i32).max().unwrap();
        let min1 = iter().map(|(i, a)| a.0+a.1+i as i32).min().unwrap();

        let max2 = iter().map(|(i, a)| a.0-a.1+i as i32).max().unwrap();
        let min2 = iter().map(|(i, a)| a.0-a.1+i as i32).min().unwrap();

        let max3 = iter().map(|(i, a)| a.0+a.1-i as i32).max().unwrap();
        let min3 = iter().map(|(i, a)| a.0+a.1-i as i32).min().unwrap();

        let max4 = iter().map(|(i, a)| a.0-a.1-i as i32).max().unwrap();
        let min4 = iter().map(|(i, a)| a.0-a.1-i as i32).min().unwrap();
        *vec![max1-min1, max2-min2, max3-min3, max4-min4].iter().max().unwrap()
    }
}
