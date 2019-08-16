/*
 * @lc app=leetcode id=1002 lang=rust
 *
 * [1002] Find Common Characters
 *
 * https://leetcode.com/problems/find-common-characters/description/
 *
 * algorithms
 * Easy (69.44%)
 * Total Accepted:    8.9K
 * Total Submissions: 13K
 * Testcase Example:  '["bella","label","roller"]'
 *
 * Given an array A of strings made only from lowercase letters, return a list
 * of all characters that show up in all strings within the list (including
 * duplicates).  For example, if a character occurs 3 times in all strings but
 * not 4 times, you need to include that character three times in the final
 * answer.
 * 
 * You may return the answer in any order.
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: ["bella","label","roller"]
 * Output: ["e","l","l"]
 * 
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: ["cool","lock","cook"]
 * Output: ["c","o"]
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= A.length <= 100
 * 1 <= A[i].length <= 100
 * A[i][j] is a lowercase letter
 * 
 * 
 * 
 */
use std::collections::HashMap;

//struct Solution;
impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let cnts_vect: Vec<HashMap<char, u32>> = a.iter().map(|x|{
            let mut hm = HashMap::new();
             for c in x.chars() {
                 let cnt = hm.entry(c).or_insert(0);
                 *cnt += 1;
             }
            hm
        }).collect();

        let mut res = Vec::new();

        for (ch, _) in cnts_vect[0].iter() {
            let min_cnt = cnts_vect.iter().map(|x| {
                x.get(ch).cloned().unwrap_or(0)
            }).min();

            let n = min_cnt.unwrap();
            for _ in 0..n {
                res.push(ch.to_string());
            }
        }

        res
        //Vec::new() as Vec<String>
    }
}

// fn main() {
//     // let v = vec![String::from("bella"),
//     //              String::from("label"),
//     //              String::from("roller")];

//     let v: Vec<String>  = vec!["cool","lock","cook"].iter().map(|x| String::from(*x)).collect();


//     println!("{:?}", Solution::common_chars(v));

// }
