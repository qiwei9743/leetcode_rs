/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 *
 * https://leetcode.com/problems/permutations/description/
 *
 * algorithms
 * Medium (56.74%)
 * Total Accepted:    462.6K
 * Total Submissions: 792K
 * Testcase Example:  '[1,2,3]'
 *
 * Given a collection of distinct integers, return all possible permutations.
 * 
 * Example:
 * 
 * 
 * Input: [1,2,3]
 * Output:
 * [
 * ⁠ [1,2,3],
 * ⁠ [1,3,2],
 * ⁠ [2,1,3],
 * ⁠ [2,3,1],
 * ⁠ [3,1,2],
 * ⁠ [3,2,1]
 * ]
 * 
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 2 {
            return vec![nums];
        }

        let count:usize = (1..=nums.len()).product();

        let mut res = vec![nums.clone()];
        for _ in 0..count-1 {
            nums = Self::next(nums.clone());
            res.push(nums.clone());
        }
        res
    }

    fn next(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = nums.len()-2;
        let mut j = nums.len()-1;
        if nums[i] > nums[j] {
            if let Some(prev) = (0..nums.len()-1).rposition(|x| nums[x] < nums[x+1] ) {
                i = prev;
                j = nums.iter().rposition(|&x| x > nums[prev] ).unwrap();
                nums.swap(i, j);
                nums[i+1..].reverse();
            } else {
                nums.reverse();
            }
            return nums;
        }

        nums.swap(i, j);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::next(vec![1,2,3]), vec![1,3,2]);
        assert_eq!(Solution::next(vec![1,3,2]), vec![2,1,3]);
        assert_eq!(Solution::next(vec![2,1,3]), vec![2,3,1]);
        assert_eq!(Solution::next(vec![2, 3, 1]), vec![3,1,2]);
        assert_eq!(Solution::next(vec![2, 3, 1]), vec![3,1,2]);
        assert_eq!(Solution::next(vec![3,1,2]), vec![3,2,1]);
        assert_eq!(Solution::next(vec![3,2,1]), vec![1,2,3]);
    }
    #[test]
    fn test2() {
        let actual = Solution::permute(vec![1,2,3]);
        let expected = vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,1,2], vec![3,2,1]];
        assert_eq!(actual, expected);
    }
}
