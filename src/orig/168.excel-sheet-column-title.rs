/*
 * @lc app=leetcode id=168 lang=rust
 *
 * [168] Excel Sheet Column Title
 *
 * https://leetcode.com/problems/excel-sheet-column-title/description/
 *
 * algorithms
 * Easy (29.37%)
 * Total Accepted:    181.2K
 * Total Submissions: 616.2K
 * Testcase Example:  '1'
 *
 * Given a positive integer, return its corresponding column title as appear in
 * an Excel sheet.
 * 
 * For example:
 * 
 * 
 * ⁠   1 -> A
 * ⁠   2 -> B
 * ⁠   3 -> C
 * ⁠   ...
 * ⁠   26 -> Z
 * ⁠   27 -> AA
 * ⁠   28 -> AB 
 * ⁠   ...
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: 1
 * Output: "A"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 28
 * Output: "AB"
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: 701
 * Output: "ZY"
 * 
 */
struct Solution;
impl Solution {
    pub fn convert_to_title(mut n: i32) -> String {
        let mut res = String::new();
        while n != 0 {
            n -= 1;
            let ch = b'A' + (n % 26) as u8;
            res.push(ch as char);
            n /= 26;
        }
        res.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(Solution::convert_to_title(1), String::from("A"));
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::convert_to_title(2), String::from("B"));
    }
    #[test]
    fn test3() {
        //assert_eq!(Solution::convert_to_title(28), String::from("AB"));
        assert_eq!(Solution::convert_to_title(701), String::from("ZY"));
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::convert_to_title(26), String::from("Z"));
    }
    #[test]
    fn test_array() {
        let arr = (0..26).map(|x| b'A' + x).collect::<Vec<_>>();
        println!("{:?}", String::from_utf8(arr).unwrap());
    }
}
