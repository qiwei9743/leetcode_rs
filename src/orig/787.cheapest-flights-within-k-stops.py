#
# @lc app=leetcode id=787 lang=python3
#
# [787] Cheapest Flights Within K Stops
#
# https://leetcode.com/problems/cheapest-flights-within-k-stops/description/
#
# algorithms
# Medium (37.01%)
# Likes:    1179
# Dislikes: 43
# Total Accepted:    64.7K
# Total Submissions: 174.4K
# Testcase Example:  '3\n[[0,1,100],[1,2,100],[0,2,500]]\n0\n2\n1'
#
# There are n cities connected by m flights. Each fight starts from city u and
# arrives at v with a price w.
# 
# Now given all the cities and flights, together with starting city src and the
# destination dst, your task is to find the cheapest price from src to dst with
# up to k stops. If there is no such route, output -1.
# 
# 
# Example 1:
# Input: 
# n = 3, edges = [[0,1,100],[1,2,100],[0,2,500]]
# src = 0, dst = 2, k = 1
# Output: 200
# Explanation: 
# The graph looks like this:
# 
# 
# The cheapest price from city 0 to city 2 with at most 1 stop costs 200, as
# marked red in the picture.
# 
# 
# Example 2:
# Input: 
# n = 3, edges = [[0,1,100],[1,2,100],[0,2,500]]
# src = 0, dst = 2, k = 0
# Output: 500
# Explanation: 
# The graph looks like this:
# 
# 
# The cheapest price from city 0 to city 2 with at most 0 stop costs 500, as
# marked blue in the picture.
# 
# Note:
# 
# 
# The number of nodes n will be in range [1, 100], with nodes labeled from 0 to
# n - 1.
# The size of flights will be in range [0, n * (n - 1) / 2].
# The format of each flight will be (src, dst, price).
# The price of each flight will be in the range [1, 10000].
# k is in the range of [0, n - 1].
# There will not be any duplicated flights or self cycles.
# 
# 
#

# @lc code=start
class Solution:
    def findCheapestPrice(self, n: int, flights: List[List[int]],
                          src: int, dst: int, K: int) -> int:
        dp1 = [float('inf')] * n
        dp2 = [float('inf')] * n
        dp1[src] = 0
        dp2[src] = 0
        for _ in range(1, K+2):
            for _from, to, price in flights:
                dp2[to] = min(dp2[to], dp1[_from] + price)
            dp1, dp2 = dp2, dp1

        return -1 if dp1[dst] == float('inf') else dp1[dst]
    def findCheapestPrice3(self, n: int, flights: List[List[int]],
                          src: int, dst: int, K: int) -> int:
        dp = [[float('inf')] * n for i in range(K+2)]
        dp[0][src] = 0;
        for i in range(1, K+2):
            dp[i][src] = 0
            for _from, to, price in flights:
                dp[i][to] = min(dp[i][to], dp[i-1][_from] + price)

        return -1 if dp[-1][dst] == float('inf') else dp[-1][dst]


    def findCheapestPrice2(self, n: int, flights: List[List[int]], src: int, dst: int, K: int) -> int:
        import collections
        import heapq
        graph = collections.defaultdict(dict)
        for s, d, p in flights:
            graph[s][d] = p

        seen = set()
        parent = {src: None}
        distance_record = init_distance(n, src)
        # distance, stops, destination
        hp = [(0, -1, src)]
        while hp:
            distance, stops, vertex = heapq.heappop(hp)
            if vertex == dst:
                return distance
            seen.add(vertex)
            # print(f"mid vertex = {vertex} to_vertex={distance} stops={stops} seen={seen} distance={distance_record}")
            for v, vertex2v in graph[vertex].items():
                # print(f"v={v} vertex2v={vertex2v}")
                # if v not in seen and distance + vertex2v < distance_record[v] and stops+1 <= K:
                if stops+1 <= K:
                    heapq.heappush(hp, (distance+vertex2v, stops+1, v))
                    parent[v] = vertex
                    distance_record[v] = distance + vertex2v

        # print(f"distance_record={distance_record}")
        # print(f"parent={parent}")
        return distance_record[dst] if distance_record[dst] != float('inf') else -1

def init_distance(n, src):
    distance = {}
    for i in range(n+1):
        distance[i] = float('inf')
    distance[src] = 0
    return distance

# @lc code=end
