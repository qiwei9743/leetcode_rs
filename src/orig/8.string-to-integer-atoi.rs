/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 *
 * https://leetcode.com/problems/string-to-integer-atoi/description/
 *
 * algorithms
 * Medium (14.73%)
 * Total Accepted:    395.2K
 * Total Submissions: 2.7M
 * Testcase Example:  '"42"'
 *
 * Implement atoi which converts a string to an integer.
 * 
 * The function first discards as many whitespace characters as necessary until
 * the first non-whitespace character is found. Then, starting from this
 * character, takes an optional initial plus or minus sign followed by as many
 * numerical digits as possible, and interprets them as a numerical value.
 * 
 * The string can contain additional characters after those that form the
 * integral number, which are ignored and have no effect on the behavior of
 * this function.
 * 
 * If the first sequence of non-whitespace characters in str is not a valid
 * integral number, or if no such sequence exists because either str is empty
 * or it contains only whitespace characters, no conversion is performed.
 * 
 * If no valid conversion could be performed, a zero value is returned.
 * 
 * Note:
 * 
 * 
 * Only the space character ' ' is considered as whitespace character.
 * Assume we are dealing with an environment which could only store integers
 * within the 32-bit signed integer range: [−2^31,  2^31 − 1]. If the numerical
 * value is out of the range of representable values, INT_MAX (2^31 − 1) or
 * INT_MIN (−2^31) is returned.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: "42"
 * Output: 42
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "   -42"
 * Output: -42
 * Explanation: The first non-whitespace character is '-', which is the minus
 * sign.
 * Then take as many numerical digits as possible, which gets 42.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: "4193 with words"
 * Output: 4193
 * Explanation: Conversion stops at digit '3' as the next character is not a
 * numerical digit.
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: "words and 987"
 * Output: 0
 * Explanation: The first non-whitespace character is 'w', which is not a
 * numerical 
 * digit or a +/- sign. Therefore no valid conversion could be performed.
 * 
 * Example 5:
 * 
 * 
 * Input: "-91283472332"
 * Output: -2147483648
 * Explanation: The number "-91283472332" is out of the range of a 32-bit
 * signed integer.
 * Thefore INT_MIN (−2^31) is returned.
 * 
 */
impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let (n, s) = match str.chars().skip_while(|x| x.is_whitespace()).take(1).next() {
            Some('+') => (1, 1),
            Some(x) if x.is_digit(10) => (0, 1),
            Some('-') => (1, -1),
            _ => return 0,
        };
        let mut res = 0i32;
        let overflow = if s > 0 { std::i32::MAX } else { std::i32::MIN };
        for c in str.chars().skip_while(|x| x.is_whitespace()).skip(n)
            .take_while(|x| x.is_digit(10)) {
                let (r, o) = res.overflowing_mul(10);
                if o { return overflow; }
                let (r, o) = r.overflowing_add(s*(c as i32 - '0' as i32));
                if o { return overflow; }
                res = r;
        }
        res
    }









    // pub fn my_atoi2(str: String) -> i32 {
    //     let mut iter = str.chars()
    //         .skip_while(|x| x.is_ascii_whitespace());
    //     match iter.next().unwrap_or('x') {
    //         c @ '+' | c @ '-' | c @ '0'..='9' => {
    //             let mut res = (0i32, false);
    //             let positive = if c == '-' {
    //                 false
    //             } else if c == '+' {
    //                 true
    //             } else {
    //                 res = (c.to_digit(10).unwrap() as i32, false);
    //                 true
    //             };
    //             for cc in iter.take_while(|x| x.is_ascii_digit()) {
    //                 res = res.0.overflowing_mul(10);
    //                 if res.1 { break };
    //                 res = res.0.overflowing_add(cc.to_digit(10).unwrap() as i32);
    //             }
    //             if res.1 {
    //                 if positive { std::i32::MAX } else { std::i32::MIN }
    //             } else {
    //                 if positive { res.0 } else { -res.0 }
    //             }
    //         },
    //         _ => 0
    //     }
    // }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_my_atoi1() {
        assert_eq!(Solution::my_atoi(String::from("-42")), -42);
    }
    #[test]
    fn test_my_atoi2() {
        assert_eq!(Solution::my_atoi(String::from("  -42")), -42);
    }
    #[test]
    fn test_my_atoi3() {
        assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193);
    }

    #[test]
    fn test_my_atoi4() {
        assert_eq!(Solution::my_atoi(String::from("words and 987")), 0);
    }
    #[test]
    fn test_my_atoi5() {
        assert_eq!(Solution::my_atoi(String::from("-91283472332")), -2147483648);
    }
}
