/*
 * @lc app=leetcode id=166 lang=rust
 *
 * [166] Fraction to Recurring Decimal
 *
 * https://leetcode.com/problems/fraction-to-recurring-decimal/description/
 *
 * algorithms
 * Medium (19.79%)
 * Total Accepted:    94.8K
 * Total Submissions: 476.2K
 * Testcase Example:  '1\n2'
 *
 * Given two integers representing the numerator and denominator of a fraction,
 * return the fraction in string format.
 * 
 * If the fractional part is repeating, enclose the repeating part in
 * parentheses.
 * 
 * Example 1:
 * 
 * 
 * Input: numerator = 1, denominator = 2
 * Output: "0.5"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: numerator = 2, denominator = 1
 * Output: "2"
 * 
 * Example 3:
 * 
 * 
 * Input: numerator = 2, denominator = 3
 * Output: "0.(6)"
 * 
 * 
 */

struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        use std::collections::HashMap;
        let mut did_reminders = HashMap::new();
        let numerator = numerator as i64;
        let denominator = denominator as i64;
        
        let mut n = (numerator.abs() / denominator.abs()) as i64;
        let mut res = if numerator * denominator < 0 {
            vec![String::from("-"), n.to_string(), String::from(".")]
        } else {
            vec![n.to_string(), String::from(".")]
        };
        let mut numerator = numerator.abs();
        let mut denominator = denominator.abs();
        let mut reminder = numerator % denominator;
        loop {
            if did_reminders.contains_key(&reminder) {
                break
            }
            did_reminders.insert(reminder, res.len());

            reminder *= 10;
            n = reminder / denominator;
            reminder = reminder % denominator;
            res.push(n.to_string());
        }
        if reminder == 0 {
            res.pop();
            if res.last().unwrap() == &String::from(".") {
                res.pop();
            }
        } else {
            let last_n = (reminder * 10) / denominator;
            //let idx = res.iter().enumerate().rev().find(|(idx, x)| **x == last_n.to_string()).unwrap().0;
            let idx = did_reminders[&reminder];
            res.insert(idx, String::from("("));
            res.push(String::from(")"));
        }
        res.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_to_decimal() {
        assert_eq!(Solution::fraction_to_decimal(2, 3), String::from("0.(6)"));
    }
    #[test]
    fn test_fraction_to_decimal2() {
        assert_eq!(Solution::fraction_to_decimal(1, 3), String::from("0.(3)"));
    }
    #[test]
    fn test_fraction_to_decimal3() {
        assert_eq!(Solution::fraction_to_decimal(2, 7), String::from("0.(285714)"));
    }
    #[test]
    fn test_fraction_to_decimal4() {
        assert_eq!(Solution::fraction_to_decimal(3, 7), String::from("0.(428571)"));
    }
    #[test]
    fn test_fraction_to_decimal5() {
        assert_eq!(Solution::fraction_to_decimal(4, 7), String::from("0.(571428)"));
    }
    #[test]
    fn test_fraction_to_decimal6() {
        assert_eq!(Solution::fraction_to_decimal(5, 7), String::from("0.(714285)"));
    }
    #[test]
    fn test_fraction_to_decimal7() {
        assert_eq!(Solution::fraction_to_decimal(6, 7), String::from("0.(857142)"));
    }
    #[test]
    fn test_fraction_to_decimal8() {
        assert_eq!(Solution::fraction_to_decimal(1, 8), String::from("0.125"));
    }
    #[test]
    fn test_fraction_to_decimal9() {
        assert_eq!(Solution::fraction_to_decimal(3, 41), String::from("0.(07317)"));
    }
    #[test]
    fn test_fraction_to_decimal10() {
        assert_eq!(Solution::fraction_to_decimal(0, 41), String::from("0"));
    }
    #[test]
    fn test_fraction_to_decimal11() {
        assert_eq!(Solution::fraction_to_decimal(40, 41), String::from("0.(97560)"));
    }
    #[test]
    fn test_fraction_to_decimal12() {
        assert_eq!(Solution::fraction_to_decimal(1, 6), String::from("0.1(6)"));
    }
    #[test]
    fn test_fraction_to_decimal13() {
        assert_eq!(Solution::fraction_to_decimal(1, 333), String::from("0.(003)"));
    }
    #[test]
    fn test_fraction_to_decimal14() {
        assert_eq!(Solution::fraction_to_decimal(-1, -2147483648),
                   String::from("0.0000000004656612873077392578125"));
    }
}
