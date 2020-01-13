/*
 * @lc app=leetcode id=38 lang=rust
 *
 * [38] Count and Say
 *
 * https://leetcode.com/problems/count-and-say/description/
 *
 * algorithms
 * Easy (39.59%)
 * Total Accepted:    263.7K
 * Total Submissions: 664.4K
 * Testcase Example:  '1'
 *
 * The count-and-say sequence is the sequence of integers with the first five
 * terms as following:
 * 
 * 
 * 1.     1
 * 2.     11
 * 3.     21
 * 4.     1211
 * 5.     111221
 * 
 * 
 * 1 is read off as "one 1" or 11.
 * 11 is read off as "two 1s" or 21.
 * 21 is read off as "one 2, then one 1" or 1211.
 * 
 * Given an integer n where 1 ≤ n ≤ 30, generate the n^th term of the
 * count-and-say sequence.
 * 
 * Note: Each term of the sequence of integers will be represented as a
 * string.
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: 1
 * Output: "1"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: 4
 * Output: "1211"
 * 
 */
#[allow(dead_code)]
#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut fx = vec![b'1'];
        if n == 1 {
            return String::from_utf8(fx).unwrap();
        }
        for _i in 2..=n {
            let mut cnt = 0;
            let mut prev = b'3';
            let mut res = vec![];
            for c in fx.into_iter() {
                if prev == c {
                    cnt += 1;
                } else  {
                    if cnt > 0 {
                        res.push(cnt as u8 + b'0');
                        res.push(prev);
                    }
                    cnt = 1;
                }
                prev = c;
            }
            if cnt > 0 {
                res.push(cnt as u8 + b'0');
                res.push(prev);
            }
            fx = res;
        }
        String::from_utf8(fx).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(Solution::count_and_say(1), String::from("1"));
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::count_and_say(2), String::from("11"));
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::count_and_say(3), String::from("21"));
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::count_and_say(4), String::from("1211"))
    }
}
