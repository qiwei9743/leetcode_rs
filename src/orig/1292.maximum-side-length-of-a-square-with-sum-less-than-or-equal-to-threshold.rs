/*
 * @lc app=leetcode id=1292 lang=rust
 *
 * [1292] Maximum Side Length of a Square with Sum Less than or Equal to Threshold
 *
 * https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/description/
 *
 * algorithms
 * Medium (42.99%)
 * Likes:    133
 * Dislikes: 4
 * Total Accepted:    5.2K
 * Total Submissions: 12K
 * Testcase Example:  '[[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]]\r\n4\r'
 *
 * Given a m x n matrix mat and an integer threshold. Return the maximum
 * side-length of a square with a sum less than or equal to threshold or return
 * 0 if there is no such square.
 *
 *
 * Example 1:
 *
 *
 * Input: mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]], threshold =
 * 4
 * Output: 2
 * Explanation: The maximum side length of square with sum less than 4 is 2 as
 * shown.
 *
 *
 * Example 2:
 *
 *
 * Input: mat = [[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2]],
 * threshold = 1
 * Output: 0
 *
 *
 * Example 3:
 *
 *
 * Input: mat = [[1,1,1,1],[1,0,0,0],[1,0,0,0],[1,0,0,0]], threshold = 6
 * Output: 3
 *
 *
 * Example 4:
 *
 *
 * Input: mat = [[18,70],[61,1],[25,85],[14,40],[11,96],[97,96],[63,45]],
 * threshold = 40184
 * Output: 2
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= m, n <= 300
 * m == mat.length
 * n == mat[i].length
 * 0 <= mat[i][j] <= 10000
 * 0 <= threshold <= 10^5
 *
 *
 */

#[cfg(feature = "local")]
struct Solution;

// @lc code=start
impl Solution {
    pub fn max_side_length_other_ones(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let h = mat.len();
        let w = if h == 0 { 0 } else { mat[0].len() };
        let mut best = 0;
        'outer: for len in 1..=std::cmp::min(w, h) {
            for x in 0..=w - len {
                for y in 0..=h - len {
                    let sum: i32 = mat[y..y + len]
                        .iter()
                        .map(|r: &Vec<i32>| r[x..x + len].iter())
                        .flatten()
                        .sum();
                    if sum <= threshold {
                        best = len;
                        continue 'outer;
                    }
                }
            }
            break; // nothing found
        }
        best as i32
    }
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let mut res = 0;
        'outer: for m in (1..=std::cmp::min(mat.len(), if mat.is_empty() { 0 } else { mat[0].len() }) + 1) {
            for row in 0..mat.len() - m + 1 {
                for col in 0..mat[row].len() - m + 1 {
                    if mat[row..row + m]
                        .iter()
                        .map(|x| x[col..col + m].iter())
                        .flatten()
                        .sum::<i32>()
                        <= threshold
                    {
                        res = m;
                        continue 'outer;
                    }
                }
            }
            break
        }
        res as i32
    }

    pub fn max_side_length2(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let (mut l, mut r) = (
            0,
            std::cmp::min(mat.len(), mat.first().map_or(0, |x| x.len())) + 1,
        );

        while l < r {
            let m = l + (r - l) / 2;
            let mut found = false;
            //println!("l={} r={} m={}", l, r, m);
            if m > 0 {
                // 'first_for: for row in 0..mat.len() - m + 1 {
                //     for col in 0..mat[row].len() - m + 1 {
                //         let sum = mat[row..row + m]
                //             .iter()
                //             .map(|rowx| rowx[col..col + m].iter().sum::<i32>())
                //             .sum::<i32>();

                //         if sum <= threshold {
                //             found = true;
                //             break 'first_for
                //         }
                //     }
                // }

                'first_for: for row in 0..mat.len() - m + 1 {
                    for col in 0..mat[row].len() - m + 1 {
                        if mat[row..row + m]
                            .iter()
                            .map(|x| x[col..col + m].iter())
                            .flatten()
                            .sum::<i32>()
                            <= threshold
                        {
                            found = true;
                            break 'first_for;
                        }
                    }
                }
            }

            if !found {
                r = m;
            } else {
                l = m + 1;
            }
        }
        std::cmp::max(l as i32 - 1, 0)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_equal_to_threadhold() {
        Solution::max_side_length(vec![vec![2, 2], vec![2, 2]], 1);
    }
}
