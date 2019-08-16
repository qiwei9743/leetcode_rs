/*
 * @lc app=leetcode id=415 lang=rust
 *
 * [415] Add Strings
 *
 * https://leetcode.com/problems/add-strings/description/
 *
 * algorithms
 * Easy (44.16%)
 * Total Accepted:    107.2K
 * Total Submissions: 242.3K
 * Testcase Example:  '"0"\n"0"'
 *
 * Given two non-negative integers num1 and num2 represented as string, return
 * the sum of num1 and num2.
 * 
 * Note:
 * 
 * The length of both num1 and num2 is < 5100.
 * Both num1 and num2 contains only digits 0-9.
 * Both num1 and num2 does not contain any leading zero.
 * You must not use any built-in BigInteger library or convert the inputs to
 * integer directly.
 * 
 * 
 */
struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (mut iter1, mut iter2) = (num1.bytes().rev(), num2.bytes().rev());
        let mut res = vec![];
        let mut carry:u8 = 0;
        loop {
            match (iter1.next(),iter2.next()) {
                (None, None) => break,
                (a, b) => {
                    let mut n = a.unwrap_or(b'0') + b.unwrap_or(b'0') - b'0' + carry;
                    if n > b'9' {
                        carry = 1;
                        n -= 10;
                    } else {
                        carry = 0;
                    }
                    res.push(n as char);
                }
            }
        }
        if carry > 0 {
            res.push('1');
        }
        res.iter().rev().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::add_strings("10".into(), "1".into()),
                   "11".to_string());
    }
}