/*
 * @lc app=leetcode id=1054 lang=rust
 *
 * [1054] Distant Barcodes
 *
 * https://leetcode.com/problems/distant-barcodes/description/
 *
 * algorithms
 * Medium (37.84%)
 * Total Accepted:    4.1K
 * Total Submissions: 10.7K
 * Testcase Example:  '[1,1,1,2,2,2]'
 *
 * In a warehouse, there is a row of barcodes, where the i-th barcode is
 * barcodes[i].
 * 
 * Rearrange the barcodes so that no two adjacent barcodes are equal.  You may
 * return any answer, and it is guaranteed an answer exists.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: [1,1,1,2,2,2]
 * Output: [2,1,2,1,2,1]
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [1,1,1,1,2,2,3,3]
 * Output: [1,3,1,3,2,1,2,1]
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= barcodes.length <= 10000
 * 1 <= barcodes[i] <= 10000
 * 
 * 
 * 
 * 
 * 
 */

use std::collections::HashMap;
impl Solution {
    pub fn rearrange_barcodes(mut barcodes: Vec<i32>) -> Vec<i32> {
        let mut hash:HashMap<i32, i32> = HashMap::new();
        for v in barcodes.iter() {
            if let Some(&cnt) = hash.get(v) {
                hash.insert(*v, cnt+1);
            } else {
                hash.insert(*v, 1);
            }
        }

        barcodes.sort_by_key(|&v| (hash.get(&v), v));

        let len = barcodes.len();
        let mut res = Vec::with_capacity(len);
        res.resize(len, 0);
        let mut i = 0;
        for j in (1..len).step_by(2) {
            res[j] = barcodes[i];
            i += 1;
        }
        for j in (0..len).step_by(2) {
            res[j] = barcodes[i];
            i += 1;
        }
        res
    }
}
