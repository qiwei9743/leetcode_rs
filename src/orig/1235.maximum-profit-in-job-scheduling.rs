/*
 * @lc app=leetcode id=1235 lang=rust
 *
 * [1235] Maximum Profit in Job Scheduling
 *
 * https://leetcode.com/problems/maximum-profit-in-job-scheduling/description/
 *
 * algorithms
 * Hard (44.34%)
 * Likes:    221
 * Dislikes: 2
 * Total Accepted:    5.6K
 * Total Submissions: 12.8K
 * Testcase Example:  '[1,2,3,3]\n[3,4,5,6]\n[50,10,40,70]'
 *
 * We have n jobs, where every job is scheduled to be done from startTime[i] to
 * endTime[i], obtaining a profit of profit[i].
 * 
 * You're given the startTime , endTime and profit arrays, you need to output
 * the maximum profit you can take such that there are no 2 jobs in the subset
 * with overlapping time range.
 * 
 * If you choose a job that ends at time X you will be able to start another
 * job that starts at time X.
 * 
 * 
 * Example 1:
 * 
 * 
 * 
 * 
 * Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
 * Output: 120
 * Explanation: The subset chosen is the first and fourth job. 
 * Time range [1-3]+[3-6] , we get profit of 120 = 50 + 70.
 * 
 * 
 * Example 2:
 * 
 * ⁠
 * 
 * 
 * 
 * Input: startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit =
 * [20,20,100,70,60]
 * Output: 150
 * Explanation: The subset chosen is the first, fourth and fifth job. 
 * Profit obtained 150 = 20 + 70 + 60.
 * 
 * 
 * Example 3:
 * 
 * 
 * 
 * 
 * Input: startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
 * Output: 6
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= startTime.length == endTime.length == profit.length <= 5 * 10^4
 * 1 <= startTime[i] < endTime[i] <= 10^9
 * 1 <= profit[i] <= 10^4
 * 
 * 
 */

// @lc code=start


#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>,
                          end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let (mut dp, mut arr) = (vec![0i32; start_time.len()], (0..start_time.len())
            .map(|i| (start_time[i], end_time[i], profit[i])).collect::<Vec<_>>());
        arr.sort_by_key(|x| x.1);

        for i in 0..start_time.len() {
            let extra = match arr[..i].binary_search_by(|probe| probe.1.cmp(&arr[i].0)) {
                Ok(mut pos) => {
                    while pos+1 < i && arr[pos+1].1 == arr[i].0 { pos += 1 }
                    dp[pos]
                },
                Err(pos) => if pos == 0 { 0 } else { dp[pos-1] },
            };
            dp[i] = std::cmp::max(if i > 0 { dp[i-1] } else { arr[i].2 },
                                  arr[i].2 + extra);
        }
        *dp.last().unwrap_or(&0)
    }


    pub fn job_scheduling_dirty(start_time: Vec<i32>,
                          end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut arr = (0..start_time.len()).map(|i| (start_time[i], end_time[i], profit[i])).collect::<Vec<_>>();
        arr.sort_by_key(|x| x.1);
        let mut dp = vec![0i32; start_time.len()];
        // println!("{:?}", arr);
        for i in 0..start_time.len() {
            // dp[i] = std::cmp::max(if i > 0 { dp[i-1] } else { arr[i].2 },
            //              arr[i].2 + arr[..i].iter().rposition(
            //                  |&x| x.1 <= arr[i].0).map(|x| dp[x]).unwrap_or(0) as i32);

            // let pos = Self::bs_first_less_or_equal(&arr[..i],
            //                               |a| { a.1 },
            //                               1);
            // let extra = let Some(pos) = arr[..i].binary_search_by(|probe| probe.1.cmp(&arr[i].0)).ok() {
            //     if pos == 0 { 0 } else { dp[pos-1] }
            // } else {
            //     0
            // };
            let extra = match arr[..i].binary_search_by(|probe| probe.1.cmp(&arr[i].0)) {
                Ok(mut pos) => {
                    while pos+1 < i && arr[pos+1].1 == arr[i].0 {
                        pos += 1
                    }
                    dp[pos]
                },
                Err(pos) => if pos == 0 { 0 } else { dp[pos-1] },
            };

            dp[i] = std::cmp::max(if i > 0 { dp[i-1] } else { arr[i].2 },
                                  arr[i].2 + extra);
                                  //arr[i].2 + pos.map_or(0, |x| dp[x]));

            // dp[i] = arr[i].2 + arr[..i].iter().enumerate()
            //     .filter_map(|(j, &s)| {
            //         if s.1 <= arr[i].0 { Some(dp[j]) } else { None }
            //     }).max().unwrap_or(0);
        }
        // println!("{:?}", dp);
        // *dp.iter().max().unwrap_or(&0)
        *dp.last().unwrap_or(&0)
    }
    // pub fn bs_first_less_or_equal<T>(arr: &[T], getter: fn(&T) -> i32, target: i32) -> Option<usize> {
    //     let (mut lo, mut hi) = (0, arr.len()-1);
    //     while lo < hi {
    //         let mid = lo + (hi-lo)/2;
    //         match getter(&arr[mid]).cmp(&target) {
    //             std::cmp::Ordering::Equal => hi = mid,
    //             std::cmp::Ordering::Less => lo = mid,
    //             std::cmp::Ordering::Greater => hi = mid - 1,
    //         }
    //     }
    //     if getter(&arr[lo]) <= target { Some(lo) } else { None }
    // }

}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_scheduling() {
         assert_eq!(Solution::job_scheduling(
            vec![1,2,3,3],
            vec![3,4,5,6],
             vec![50,10,40,70]), 120);

        assert_eq!(Solution::job_scheduling(
            vec![1,2,3,4,6],
            vec![3,5,10,6,9],
            vec![20,20,100,70,60]), 150);

        assert_eq!(Solution::job_scheduling(
            vec![1,1,1],
            vec![2,3,4],
            vec![5,6,4]), 6);

        assert_eq!(Solution::job_scheduling(
            vec![],
            vec![],
            vec![]), 0);
        assert_eq!(Solution::job_scheduling(
            vec![1],
            vec![2],
            vec![3]), 3);
        assert_eq!(Solution::job_scheduling(
            vec![6,15,7,11,1,3,16,2],
            vec![19,18,19,16,10,8,19,8],
            vec![2,9,1,19,5,7,3,19]), 41);

        assert_eq!(Solution::job_scheduling(
            vec![24,24,7,2,1,13,6,14,18,24],
            vec![27,27,20,7,14,22,20,24,19,27],
            vec![6,1,4,2,3,6,5,6,9,8]), 20);

        assert_eq!(Solution::job_scheduling(
            vec![7,2,6],
            vec![20,7,20],
            vec![4,2,5]), 6);
    }
    // #[test]
    // fn test_job_scheduling_bs() {
    //     assert_eq!(
    //         Solution::bs_first_less_or_equal(&vec![1,1,3,3,3][..],
    //                                 |x| *x,
    //                                 1),
    //         Some(0));

    //     assert_eq!(
    //         Solution::bs_first_less_or_equal(&vec![1,1,3,3,3][..],
    //                                          |x| *x,
    //                                          2),
    //         Some(1));
    // }
}

