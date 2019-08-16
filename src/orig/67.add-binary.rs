/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 *
 * https://leetcode.com/problems/add-binary/description/
 *
 * algorithms
 * Easy (39.85%)
 * Total Accepted:    324K
 * Total Submissions: 810.2K
 * Testcase Example:  '"11"\n"1"'
 *
 * Given two binary strings, return their sum (also a binary string).
 * 
 * The input strings are both non-empty and contains only characters 1 or 0.
 * 
 * Example 1:
 * 
 * 
 * Input: a = "11", b = "1"
 * Output: "100"
 * 
 * Example 2:
 * 
 * 
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 * 
 */
struct Solution;

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        if a.len() > b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let mut cc = 0u8;
        let res = a.bytes()
            .rev()
            .zip(b.bytes().rev())
            .map(|(x1, x2)| (x1-b'0', x2-b'0'))
            .fold(String::from(""), |mut acc: String, x:(u8, u8)| -> String {
                let cr = x.0 + x.1 + cc;
                cc = if cr >= 2 { 1 } else { 0 };
                if cr % 2 != 0 {
                    acc.push('1');
                } else {
                    acc.push('0');
                }
                acc
            });
        let mut res = b.bytes()
            .rev()
            .skip(a.len())
            .map(|x| x - b'0')
            .fold(res, |mut acc: String, x:u8 | -> String {
                let cr = x + cc;
                cc = if cr >= 2 {
                    1
                } else {
                    0
                };
                if cr % 2 != 0 {
                    acc.push('1');
                } else {
                    acc.push('0');
                }
                acc.to_string()
        });
        if cc > 0 {
            res.push('1');
        }
        res.chars().rev().collect()
    }
}


#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::add_binary("1".into(),"1".into()), "10");
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::add_binary("110".into(), "1".into()), "111");
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::add_binary("111".into(), "1".into()), "1000");
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::add_binary("11".into(), "1".into()), "100");
    }
}
