/*
 * @lc app=leetcode id=1128 lang=rust
 *
 * [1128] Number of Equivalent Domino Pairs
 *
 * https://leetcode.com/problems/number-of-equivalent-domino-pairs/description/
 *
 * algorithms
 * Easy (42.42%)
 * Total Accepted:    6.2K
 * Total Submissions: 14.6K
 * Testcase Example:  '[[1,2],[2,1],[3,4],[5,6]]'
 *
 * Given a list of dominoes, dominoes[i] = [a, b] is equivalent to dominoes[j]
 * = [c, d] if and only if either (a==c and b==d), or (a==d and b==c) - that
 * is, one domino can be rotated to be equal to another domino.
 * 
 * Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length,
 * and dominoes[i] is equivalent to dominoes[j].
 * 
 * 
 * Example 1:
 * Input: dominoes = [[1,2],[2,1],[3,4],[5,6]]
 * Output: 1
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= dominoes.length <= 40000
 * 1 <= dominoes[i][j] <= 9
 * 
 */
use std::collections::HashMap;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut count_map: HashMap<(i32, i32), i32> = HashMap::new();
        for i in 0..dominoes.len() {
            let dom = (dominoes[i][0], dominoes[i][1]);
            let bro = (dom.1, dom.0);
            if let Some(cnt) = count_map.get(&dom) {
                res += cnt;
            }
            if dom != bro {
                if let Some(cnt)= count_map.get(&bro) {
                    res += cnt;
                }
            }


            let mut cnt_entry = count_map.entry(dom).or_insert(0);
            *cnt_entry += 1;
        }
        res

    }
}
