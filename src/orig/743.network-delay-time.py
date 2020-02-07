#
# @lc app=leetcode id=743 lang=python3
#
# [743] Network Delay Time
#
# https://leetcode.com/problems/network-delay-time/description/
#
# algorithms
# Medium (45.02%)
# Likes:    981
# Dislikes: 198
# Total Accepted:    63.2K
# Total Submissions: 140K
# Testcase Example:  '[[2,1,1],[2,3,1],[3,4,1]]\n4\n2'
#
# There are N network nodes, labelled 1 to N.
# 
# Given times, a list of travel times as directed edges times[i] = (u, v, w),
# where u is the source node, v is the target node, and w is the time it takes
# for a signal to travel from source to target.
# 
# Now, we send a signal from a certain node K. How long will it take for all
# nodes to receive the signal? If it is impossible, return -1.
# 
# 
# 
# Example 1:
# 
# 
# 
# 
# Input: times = [[2,1,1],[2,3,1],[3,4,1]], N = 4, K = 2
# Output: 2
# 
# 
# 
# 
# Note:
# 
# 
# N will be in the range [1, 100].
# K will be in the range [1, N].
# The length of times will be in the range [1, 6000].
# All edges times[i] = (u, v, w) will have 1 <= u, v <= N and 0 <= w <= 100.
# 
# 
#

# @lc code=start
class Solution:
    def networkDelayTime(self, times: List[List[int]], N: int, K: int) -> int:
        import heapq
        import collections
        graph = collections.defaultdict(dict)
        for u, v, c in times:
            graph[u][v] = c
        seen = set()
        parent = {K: None}
        distanceFromK = {i: float('inf') for i in range(1, N+1)}
        distanceFromK[K] = 0


        hp = [(0, K)]
        while hp:
            K2u, u = heapq.heappop(hp)
            seen.add(u)
            for v, u2v in graph[u].items():
                if v not in seen and K2u + u2v < distanceFromK[v]:
                    parent[v] = u
                    distanceFromK[v] = K2u + u2v
                    heapq.heappush(hp, (K2u+u2v, v))

        r = max(distanceFromK.values())
        return -1 if r == float('inf') else r
































#     def networkDelayTime4(self, times: List[List[int]], N: int, K: int) -> int:
#         dp1 = [float('inf')] * (N + 1)
#         dp2 = [float('inf')] * (N + 1)
#         dp1[K] = 0
#         dp2[K] = 0
#         for _ in range(N-1):
#             for _from, to, t in times:
#                 dp2[to] = min(dp2[to], dp1[_from] + t)
#             dp1,dp2 = dp2, dp1
#         r = max(dp1[1:])
#         return -1 if r == float('inf') else r





#     def networkDelayTime3(self, times: List[List[int]], N: int, K: int) -> int:
#         import collections
#         import heapq
#         seen = set()
#         graph = collections.defaultdict(dict)
#         for _from, to, time in times:
#             graph[_from][to] = time
#         parent = {K: None}
#         distance_record = init_distance(N, K)
#         hp = [(0, K)]
#         while hp:
#             distance, vertex = heapq.heappop(hp)
#             seen.add(vertex)
#             for v, vertex2v in graph[vertex].items():
#                 if v not in seen and distance + vertex2v < distance_record[v]:
#                     heapq.heappush(hp, (distance + vertex2v, v))
#                     distance_record[v] = distance + vertex2v
#                     parent[v] = vertex
#         r = max(distance_record.values())
#         return -1 if r == float('inf') else r





#     def networkDelayTime2(self, times: List[List[int]], N: int, K: int) -> int:
#         import heapq
#         import collections
#         graph = collections.defaultdict(dict)
#         for u, v, w in times:
#             graph[u][v] = w

#         pq = [(0, K)]
#         parent_records = {K: None}
#         distance_records = init_distance(N, K)
#         seen = set()
#         # print("before while")
#         while pq:
#             # print(f"pq={pq}")
#             distance, vertex = heapq.heappop(pq)
#             seen.add(vertex)
#             # print(f"before for")
#             for v, vertex2v_dis in graph[vertex].items():
#                 #print(f"before if not seen")
#                 if v not in seen:
#                     new_distance = distance + vertex2v_dis
#                     if new_distance < distance_records[v]:
#                         heapq.heappush(pq, (new_distance, v))
#                         parent_records[v] = vertex
#                         distance_records[v] = new_distance
#         r = max(distance_records.values())
#         #print(f"parent={parent_records}")
#         return -1 if r == float('inf') else r


# def init_distance(N, K):
#     distance = {}
#     for n in range(1, N+1):
#         distance[n] = float('inf')
#     distance[K] = 0
#     return distance

# @lc code=end
