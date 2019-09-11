/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 *
 * https://leetcode.com/problems/roman-to-integer/description/
 *
 * algorithms
 * Easy (51.84%)
 * Total Accepted:    416.5K
 * Total Submissions: 795.3K
 * Testcase Example:  '"III"'
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
 * Given a roman numeral, convert it to an integer. Input is guaranteed to be
 * within the range from 1 to 3999.
 * 
 * Example 1:
 * 
 * 
 * Input: "III"
 * Output: 3
 * 
 * Example 2:
 * 
 * 
 * Input: "IV"
 * Output: 4
 * 
 * Example 3:
 * 
 * 
 * Input: "IX"
 * Output: 9
 * 
 * Example 4:
 * 
 * 
 * Input: "LVIII"
 * Output: 58
 * Explanation: L = 50, V= 5, III = 3.
 * 
 * 
 * Example 5:
 * 
 * 
 * Input: "MCMXCIV"
 * Output: 1994
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 * 
 */
struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut divec = vec![(1, "I"), (4, "IV"), (5, "V"), (9, "IX"),
                         (10, "X"), (40, "XL"), (50, "L"), (90, "XC"),
                         (100, "C"), (400, "CD"), (500, "D"), (900, "CM"),
                         (1000, "M")];
        divec.sort_by_key(|(_, symbol)| symbol.len());
        let mut res = 0i32;
        let mut i = 0usize;
        //for i in 0..s.len() {
        while i < s.len() {
            let (score, symbol) = divec.iter().rev()
                .find(|(score, symbol)| symbol == &&s[i..std::cmp::min(i+symbol.len(), s.len())])
                .unwrap();
            res += score;
            i += symbol.len();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_integer() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);

    }
}
