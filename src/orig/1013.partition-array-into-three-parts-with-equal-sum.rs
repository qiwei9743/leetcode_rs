/*
 * @lc app=leetcode id=1013 lang=rust
 *
 * [1013] Partition Array Into Three Parts With Equal Sum
 *
 * https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/description/
 *
 * algorithms
 * Easy (55.17%)
 * Total Accepted:    12.4K
 * Total Submissions: 22.4K
 * Testcase Example:  '[0,2,1,-6,6,-7,9,1,2,0,1]'
 *
 * Given an array A of integers, return true if and only if we can partition
 * the array into three non-empty parts with equal sums.
 * 
 * Formally, we can partition the array if we can find indexes i+1 < j with
 * (A[0] + A[1] + ... + A[i] == A[i+1] + A[i+2] + ... + A[j-1] == A[j] + A[j-1]
 * + ... + A[A.length - 1])
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: [0,2,1,-6,6,-7,9,1,2,0,1]
 * Output: true
 * Explanation: 0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [0,2,1,-6,6,7,9,-1,2,0,1]
 * Output: false
 * 
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: [3,3,6,5,-2,2,5,1,-9,4]
 * Output: true
 * Explanation: 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
 * 
 * 
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 3 <= A.length <= 50000
 * -10000 <= A[i] <= 10000
 * 
 */
impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let _sum:i32 = a.iter().sum();
        let one_third = _sum / 3;
        let mut part = 0;
        let mut _cur = 0;
        for &v in a.iter() {
            _cur += v;
            if _cur == one_third {
                _cur = 0;
                part += 1;
            }
        }
        part > 0 && part % 3 == 0
    }
    // pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
    //     let _sum:i32 = a.iter().sum();
    //     let one_third:i32 = _sum / 3;
    //     if one_third * 3 != _sum {
    //         return false
    //     }


    //     let mut _sum1 = 0;
    //     for i in 0..a.len()-1 {
    //         _sum1 += a[i];
    //         if _sum1 != one_third {
    //             continue
    //         }
    //         let mut _sum2 = 0;
    //         for j in i+1..a.len()-1 {
    //             _sum2 += a[j];
    //             if _sum2 != one_third {
    //                 continue
    //             }

    //             let _sum3:i32 = a[j+1..].iter().sum();
    //             if _sum3 == one_third {
    //                 return true
    //             }
    //         }
    //     }
    //     return false
    // }

}
