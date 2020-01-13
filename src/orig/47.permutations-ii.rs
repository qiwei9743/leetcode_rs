/*
 * @lc app=leetcode id=47 lang=rust
 *
 * [47] Permutations II
 *
 * https://leetcode.com/problems/permutations-ii/description/
 *
 * algorithms
 * Medium (41.67%)
 * Total Accepted:    286.3K
 * Total Submissions: 667.1K
 * Testcase Example:  '[1,1,2]'
 *
 * Given a collection of numbers that might contain duplicates, return all
 * possible unique permutations.
 * 
 * Example:
 * 
 * 
 * Input: [1,1,2]
 * Output:
 * [
 * ⁠ [1,1,2],
 * ⁠ [1,2,1],
 * ⁠ [2,1,1]
 * ]
 * 
 * 
 */

#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;


impl Solution {
    #[allow(dead_code)]
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        std::iter::successors(Some(nums.clone()), |n| {
            let mut cur = n.clone();
            if let Some(prev) = (0..cur.len()-1).rposition(|x| cur[x] < cur[x+1]) {
                let j = cur.iter().rposition(|&x| x > cur[prev]).unwrap();
                cur.swap(prev, j);
                cur[prev+1..].reverse();
            } else { cur.reverse(); }
            if cur == nums { None } else { Some(cur) }
        }).collect()
    }

    #[allow(dead_code)]
    pub fn permute_unique_old(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![nums];
        loop {
            let cur = Self::next(res.last().unwrap().clone());
            if cur == res[0] {
                return res;
            }
            res.push(cur);
        }
    }

    fn next(mut nums: Vec<i32>) -> Vec<i32> {
        if let Some(prev) = (0..nums.len()-1).rposition(|x| nums[x] < nums[x+1]) {
            let j = nums.iter().rposition(|&x| x > nums[prev]).unwrap();
            nums.swap(prev, j);
            nums[prev+1..].reverse();
        } else { nums.reverse(); }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::permute_unique(vec![1,1,2]),
                   vec![vec![1,1,2],vec![1,2,1],vec![2,1,1]]);
    }
}
