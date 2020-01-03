/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] ZigZag Conversion
 *
 * https://leetcode.com/problems/zigzag-conversion/description/
 *
 * algorithms
 * Medium (32.59%)
 * Total Accepted:    343.6K
 * Total Submissions: 1.1M
 * Testcase Example:  '"PAYPALISHIRING"\n3'
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number
 * of rows like this: (you may want to display this pattern in a fixed font for
 * better legibility)
 * 
 * 
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * 
 * 
 * And then read line by line: "PAHNAPLSIIGYIR"
 * 
 * Write the code that will take a string and make this conversion given a
 * number of rows:
 * 
 * 
 * string convert(string s, int numRows);
 * 
 * Example 1:
 * 
 * 
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * 
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 * 
 */
struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s
        }
        let mut direction = 1;
        let mut row = 0i32;
        let mut res = vec![vec![]; num_rows as usize];
        for c in s.chars() {
            res[row as usize].push(c);
            row += direction;
            if row == 0 || row == num_rows - 1 {
                direction *= -1;
            }
        }
        res.into_iter().flatten().collect()
    }
    pub fn convert2(s: String, num_rows: i32) -> String {
        let s: Vec<_> = s.chars().collect();
        let mut pos = vec![vec![]; num_rows as usize];
        let (mut x, mut y) = (0, 0);
        let mut idx = 0;
        let mut down = true;
        while idx < s.len() {
            pos[y].push(s[idx]);

            if down {
                if y as i32 == num_rows-1 && y > 0 {
                    x += 1;
                    y -= 1;
                    down = false;
                } else if y + 1 < num_rows as usize {
                    y += 1
                }
            } else if y == 0 && y < num_rows as usize {
                    down = true;
                    y += 1;
            } else if y > 0 {
                    x += 1;
                    y -= 1;
            }

            idx += 1;
        }
        pos.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            Solution::convert(
                String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
        assert_eq!(
            Solution::convert(String::from("AB"), 1),
            String::from("AB")
        );
        assert_eq!(
            Solution::convert(String::from("ABC"), 1),
            String::from("ABC")
        );
    }
}
