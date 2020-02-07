/*
 * @lc app=leetcode id=911 lang=rust
 *
 * [911] Online Election
 *
 * https://leetcode.com/problems/online-election/description/
 *
 * algorithms
 * Medium (48.94%)
 * Likes:    254
 * Dislikes: 201
 * Total Accepted:    18.2K
 * Total Submissions: 37.3K
 * Testcase Example:  '["TopVotedCandidate","q","q","q","q","q","q"]\n' +
  '[[[0,1,1,0,0,1,0],[0,5,10,15,20,25,30]],[3],[12],[25],[15],[24],[8]]'
 *
 * In an election, the i-th vote was cast for persons[i] at time times[i].
 * 
 * Now, we would like to implement the following query function:
 * TopVotedCandidate.q(int t) will return the number of the person that was
 * leading the election at time t.  
 * 
 * Votes cast at time t will count towards our query.  In the case of a tie,
 * the most recent vote (among tied candidates) wins.
 * 
 * 
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: ["TopVotedCandidate","q","q","q","q","q","q"],
 * [[[0,1,1,0,0,1,0],[0,5,10,15,20,25,30]],[3],[12],[25],[15],[24],[8]]
 * Output: [null,0,1,1,0,0,1]
 * Explanation: 
 * At time 3, the votes are [0], and 0 is leading.
 * At time 12, the votes are [0,1,1], and 1 is leading.
 * At time 25, the votes are [0,1,1,0,0,1], and 1 is leading (as ties go to the
 * most recent vote.)
 * This continues for 3 more queries at time 15, 24, and 8.
 * 
 * 
 * 
 * 
 * Note:
 * 
 * 
 * 1 <= persons.length = times.length <= 5000
 * 0 <= persons[i] <= persons.length
 * times is a strictly increasing array with all elements in [0, 10^9].
 * TopVotedCandidate.q is called at most 10000 times per test case.
 * TopVotedCandidate.q(int t) is always called with t >= times[0].
 * 
 * 
 * 
 */

// @lc code=start
struct TopVotedCandidate {
    times: Vec<i32>,
    time_based_rank: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {

    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut person_votes = std::collections::HashMap::new();
        let mut time_based_rank = vec![0; times.len()];
        let mut leader = -1;

        for (i, p) in persons.iter().enumerate() {
            let mut entry = person_votes.entry(*p).or_insert(0);
            *entry += 1;
            if *entry >= *person_votes.get(&leader).unwrap_or(&0) {
                leader = *p;
            }

            time_based_rank[i] = leader;
        }

        //println!("{:?}\n{:?}", times, time_based_rank);
        Self {
            times,
            time_based_rank
        }
    }
    
    fn q(&self, t: i32) -> i32 {
        let (mut l, mut r) = (0, self.times.len());
        while l < r {
            let m = l + (r - l) / 2;
            if self.times[m] > t {
                r = m;
            } else {
                l = m + 1;
            }
        }

        if l > 0 { self.time_based_rank[l - 1] } else { 0 }
    }
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
// @lc code=end
