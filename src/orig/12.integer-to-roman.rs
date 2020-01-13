/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 *
 * https://leetcode.com/problems/integer-to-roman/description/
 *
 * algorithms
 * Medium (51.79%)
 * Total Accepted:    261.1K
 * Total Submissions: 501.7K
 * Testcase Example:  '3'
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D
 * and M.
 * 
 * 
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * 
 * For example, two is written as II in Roman numeral, just two one's added
 * together. Twelve is written as, XII, which is simply X + II. The number
 * twenty seven is written as XXVII, which is XX + V + II.
 * 
 * Roman numerals are usually written largest to smallest from left to right.
 * However, the numeral for four is not IIII. Instead, the number four is
 * written as IV. Because the one is before the five we subtract it making
 * four. The same principle applies to the number nine, which is written as IX.
 * There are six instances where subtraction is used:
 * 
 * 
 * I can be placed before V (5) and X (10) to make 4 and 9. 
 * X can be placed before L (50) and C (100) to make 40 and 90. 
 * C can be placed before D (500) and M (1000) to make 400 and 900.
 * 
 * 
 * Given an integer, convert it to a roman numeral. Input is guaranteed to be
 * within the range from 1 to 3999.
 * 
 * Example 1:
 * 
 * 
 * Input: 3
 * Output: "III"
 * 
 * Example 2:
 * 
 * 
 * Input: 4
 * Output: "IV"
 * 
 * Example 3:
 * 
 * 
 * Input: 9
 * Output: "IX"
 * 
 * Example 4:
 * 
 * 
 * Input: 58
 * Output: "LVIII"
 * Explanation: L = 50, V = 5, III = 3.
 * 
 * 
 * Example 5:
 * 
 * 
 * Input: 1994
 * Output: "MCMXCIV"
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 q* 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn int_to_roman(mut num: i32) -> String {
        let divec = vec![(1, "I"), (4, "IV"), (5, "V"), (9, "IX"),
                         (10, "X"), (40, "XL"), (50, "L"), (90, "XC"),
                         (100, "C"), (400, "CD"), (500, "D"), (900, "CM"),
                         (1000, "M")];
        let mut res = String::new();
        while num > 0 {
            let (score, symbol) = divec.iter().rev().find(|(score, _)| *score <= num).unwrap();
            res.push_str(symbol);
            num -= score;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(3), String::from("III"));
        assert_eq!(Solution::int_to_roman(4), String::from("IV"));
        assert_eq!(Solution::int_to_roman(9), String::from("IX"));
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
    }
}
