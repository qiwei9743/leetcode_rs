/*
 * @lc app=leetcode id=684 lang=rust
 *
 * [684] Redundant Connection
 *
 * https://leetcode.com/problems/redundant-connection/description/
 *
 * algorithms
 * Medium (54.85%)
 * Likes:    933
 * Dislikes: 216
 * Total Accepted:    73K
 * Total Submissions: 132.9K
 * Testcase Example:  '[[1,2],[1,3],[2,3]]'
 *
 * 
 * In this problem, a tree is an undirected graph that is connected and has no
 * cycles.
 * 
 * The given input is a graph that started as a tree with N nodes (with
 * distinct values 1, 2, ..., N), with one additional edge added.  The added
 * edge has two different vertices chosen from 1 to N, and was not an edge that
 * already existed.
 * 
 * The resulting graph is given as a 2D-array of edges.  Each element of edges
 * is a pair [u, v] with u < v, that represents an undirected edge connecting
 * nodes u and v.
 * 
 * Return an edge that can be removed so that the resulting graph is a tree of
 * N nodes.  If there are multiple answers, return the answer that occurs last
 * in the given 2D-array.  The answer edge [u, v] should be in the same format,
 * with u < v.
 * Example 1:
 * 
 * Input: [[1,2], [1,3], [2,3]]
 * Output: [2,3]
. * Explanation: The given undirected graph will be like this:
 * ⁠ 1
 * ⁠/ \
 * 2 - 3
 * 
 * 
 * Example 2:
 * 
 * Input: [[1,2], [2,3], [3,4], [1,4], [1,5]]
 * Output: [1,4]
 * Explanation: The given undirected graph will be like this:
 * 5 - 1 - 2
 * ⁠   |   |
 * ⁠   4 - 3
 * 
 * 
 * Note:
 * The size of the input 2D-array will be between 3 and 1000.
 * Every integer represented in the 2D-array will be between 1 and N, where N
 * is the size of the input array.
 * 
 * 
 * 
 * 
 * 
 * Update (2017-09-26):
 * We have overhauled the problem description + test cases and specified
 * clearly the graph is an undirected graph. For the directed graph follow up
 * please see Redundant Connection II). We apologize for any inconvenience
 * caused.
 * 
 */

// @lc code=start
struct UnionFindSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFindSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (px, py) = (self.find(x), self.find(y));
        if px == py { return false; }
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Greater => self.parent[py] = px,
            std::cmp::Ordering::Less => self.parent[px] = py,
            std::cmp::Ordering::Equal => {
                self.parent[py] = px;
                self.rank[px] += 1;
            }
        }
        true
    }
}

#[cfg(feature = "local")]
struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFindSet::new(edges.len() + 1);
        for ed in edges {
            if !uf.union(ed[0] as usize, ed[1] as usize) {
                return ed;
            }
        }
        vec![]
    }
}
// @lc code=end


