/*
i * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 *
 * https://leetcode.com/problems/subsets/description/
 *
 * algorithms
 * Medium (54.39%)
 * Total Accepted:    446K
 * Total Submissions: 791.4K
 * Testcase Example:  '[1,2,3]'
 *
 * Given a set of distinct integers, nums, return all possible subsets (the
 * power set).
 * 
 * Note: The solution set must not contain duplicate subsets.
 * 
 * Example:
 * 
 * 
 * Input: nums = [1,2,3]
 * Output:
 * [
 * ⁠ [3],
 * [1],
 * [2],
 * [1,2,3],
 * [1,3],
 * [2,3],
 * [1,2],
 * []
 * ]
 * 
 */
//struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::dfs(&nums[..], 0, &mut vec![], &mut res);
        res
    }
    pub fn dfs(nums: &[i32], idx: usize, cur: &mut Vec<i32>,
               res: &mut Vec<Vec<i32>>) {
        res.push(cur.clone());
        (idx..nums.len()).for_each(|j| {
            cur.push(nums[j]);
            Self::dfs(nums, j+1, cur, res);
            cur.pop();
        })
    }

    pub fn subsets_i(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];
        (1..=nums.len()).for_each(|i| Self::dfs_i(&nums[..], 0, i, &mut vec![], &mut res));
        res
    }
    pub fn dfs_i(nums: &[i32], idx: usize, cnt: usize,
               cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if idx == cnt {
            res.push(cur.clone());
            return
        }
        (idx..nums.len()).for_each(|j| {
            cur.push(nums[j]);
            Self::dfs_i(nums, j+1, cnt, cur, res);
            cur.pop();
        })
    }

    pub fn subsets_hash_sort(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = std::collections::HashSet::new();
        res.insert(vec![]);
        let mut access = vec![false; nums.len()];
        for i in 1..=nums.len() {
            Self::dfs_hash_sort(&nums[..], 0, i, &mut vec![], &mut res, &mut access);
        }
        res.into_iter().collect()
    }
    pub fn dfs_hash_sort(nums: &[i32], idx: usize, cnt: usize,
               cur: &mut Vec<i32>, res: &mut std::collections::HashSet<Vec<i32>>,
               access: &mut Vec<bool>) {
        if idx == cnt {
            let mut t = cur.clone();
            t.sort();
            res.insert(t);
            return
        }
        for j in 0..nums.len() {
            if access[j] == true {
                continue
            }
            cur.push(nums[j]);
            access[j] = true;
            Self::dfs_hash_sort(nums, j+1, cnt, cur, res, access);
            access[j] = false;
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        println!("subsets={:?}", Solution::subsets(vec![1,2,3]));
    }
}
