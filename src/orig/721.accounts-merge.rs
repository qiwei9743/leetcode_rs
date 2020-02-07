/*
 * @lc app=leetcode id=721 lang=rust
 *
 * [721] Accounts Merge
 *
 * https://leetcode.com/problems/accounts-merge/description/
 *
 * algorithms
 * Medium (44.78%)
 * Likes:    953
 * Dislikes: 243
 * Total Accepted:    56.6K
 * Total Submissions: 125.8K
 * Testcase Example:  '[["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]'
 *
 * Given a list accounts, each element accounts[i] is a list of strings, where
 * the first element accounts[i][0] is a name, and the rest of the elements are
 * emails representing emails of the account.
 * 
 * Now, we would like to merge these accounts.  Two accounts definitely belong
 * to the same person if there is some email that is common to both accounts.
 * Note that even if two accounts have the same name, they may belong to
 * different people as people could have the same name.  A person can have any
 * number of accounts initially, but all of their accounts definitely have the
 * same name.
 * 
 * After merging the accounts, return the accounts in the following format: the
 * first element of each account is the name, and the rest of the elements are
 * emails in sorted order.  The accounts themselves can be returned in any
 * order.
 * 
 * Example 1:
 * 
 * Input: 
 * accounts = [["John", "johnsmith@mail.com", "john00@mail.com"], ["John",
 * "johnnybravo@mail.com"], ["John", "johnsmith@mail.com",
 * "john_newyork@mail.com"], ["Mary", "mary@mail.com"]]
 * Output: [["John", 'john00@mail.com', 'john_newyork@mail.com',
 * 'johnsmith@mail.com'],  ["John", "johnnybravo@mail.com"], ["Mary",
 * "mary@mail.com"]]
 * Explanation: 
 * The first and third John's are the same person as they have the common email
 * "johnsmith@mail.com".
 * The second John and Mary are different people as none of their email
 * addresses are used by other accounts.
 * We could return these lists in any order, for example the answer [['Mary',
 * 'mary@mail.com'], ['John', 'johnnybravo@mail.com'], 
 * ['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']]
 * would still be accepted.
 * 
 * 
 * 
 * Note:
 * The length of accounts will be in the range [1, 1000].
 * The length of accounts[i] will be in the range [1, 10].
 * The length of accounts[i][j] will be in the range [1, 30].
 * 
 */

#[cfg(feature = "local")]
struct Solution;

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // init ufs
        let mut ufs = UnionFindSet::new();
        for acc in &accounts {
            for (mail1, mail2) in acc.iter().skip(1).zip(acc.iter().skip(2)) {
                ufs.union(&mail1[..], &mail2[..]);
            }
        }
        // create map group => mail list
        let mut group: HashMap<&str, Vec<&str>> = HashMap::new();
        for acc in &accounts {
            let mut acc_iter = acc.into_iter();
            let name = acc_iter.next().unwrap();
            for mail in acc_iter {
                let group_mail = ufs.find(&mail);
                let entry = group.entry(group_mail)
                    .or_insert_with(|| vec![name]);

                if let Err(i) = (*entry)[1..].binary_search(&&mail[..]) {
                    (*entry).insert(i+1, mail);
                }
            }
        }
        // base on owner, group => mail list  to owner=> mail list
        group.into_iter()
            .map(|(_, v)| v.into_iter().map(|v| v.to_string() ).collect::<Vec<_>>() )
            .collect()
    }
}


struct UnionFindSet<'a> {
    parent: HashMap<&'a str, &'a str>,
    rank: HashMap<&'a str, usize>,
}

impl<'a> UnionFindSet<'a> {
    fn new() -> Self {
        Self {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn find(&mut self, t: &'a str) -> &'a str {
        let parent = self.parent.get(t).unwrap_or(&t);
        if self.parent.get(t).unwrap_or(&t) != &t {
            let group = self.find(parent);
            self.parent.insert(t, group);
        }
        self.parent.get(t).unwrap_or(&t)
    }

    fn union(&mut self, u: &'a str, v: &'a str) -> bool {
        let pu = self.find(u);
        let pv = self.find(v);
        if pu == pv {
            return false;
        }
        let ru = *self.rank.get(u).unwrap_or(&0);
        let rv = *self.rank.get(v).unwrap_or(&0);
        match ru.cmp(&rv) {
            std::cmp::Ordering::Less => self.parent.insert(pu, pv), //self.parent[&pu] = pv,
            std::cmp::Ordering::Greater => self.parent.insert(pv, pu), // self.parent[&pv] = pu,
            std::cmp::Ordering::Equal => {
                let mut entry = self.rank.entry(pv).or_insert(1);
                *entry += 1;
                self.parent.insert(pu, pv)
            }
        };
        true
    }
}































// mod space1 {

// struct UnionFindSet {
//     parent: HashMap<String, String>,
//     rank: HashMap<String, usize>,
// }

// impl UnionFindSet {
//     fn new(cnt: usize) {
//         Self {
//             parent: HashMap::new(),
//             rank: HashMap::new(),
//         }
//     }

//     fn find(&mut self, x: &str) -> &str {
//         let entry = self.parent.entry(x.clone()).or_insert(x.clone());
//         if *entry != *x {
//             self.parent[x] = self.find(&(*entry));
//         }
//         entry
//     }

//     fn union(&mut self, u: &str, v: &str) -> bool {
//         let pu = self.find(u);
//         let pv = self.find(v);
//         if pu == pv {
//             return false;
//         }
//         match self.rank.get(pu).unwrap_or(0).cmp(self.rank.get(pv).unwrap_or(&0)) {
//             std::cmp::Ordering::Less => self.parent[pu] = pv,
//             std::cmp::Ordering::Greater => self.parent[pv] = pu,
//             std::cmp::Ordering::Equal => {
//                 self.parent[pu] = pv;
//                 let mut entry = self.rank.entry(pv).or_insert();
//                 *entry += 1;
//             },
//         }
// }
// }
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounts_merge() {
        let arr = vec![vec!["John".to_string(),"johnsmith@mail.com".to_string(),"john_newyork@mail.com".to_string()],
                       vec!["John".to_string(),"johnsmith@mail.com".to_string(),"john00@mail.com".to_string()],
                       vec!["Mary".to_string(),"mary@mail.com".to_string()],
                       vec!["John".to_string(),"johnnybravo@mail.com".to_string()]];
        Solution::accounts_merge(arr);
    }
}
