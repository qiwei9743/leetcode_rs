#
# @lc app=leetcode id=959 lang=python3
#
# [959] Regions Cut By Slashes
#
# https://leetcode.com/problems/regions-cut-by-slashes/description/
#
# algorithms
# Medium (64.39%)
# Likes:    520
# Dislikes: 107
# Total Accepted:    12.9K
# Total Submissions: 20.1K
# Testcase Example:  '[" /","/ "]'
#
# In a N x N grid composed of 1 x 1 squares, each 1 x 1 square consists of a /,
# \, or blank space.  These characters divide the square into contiguous
# regions.
# 
# (Note that backslash characters are escaped, so a \ is represented as "\\".)
# 
# Return the number of regions.
# 
# 
# 
# 
# 
# 
# 
# 
# 
# 
# 
# 
# 
# 
# 
# 
# 
# Example 1:
# 
# 
# Input:
# [
# " /",
# "/ "
# ]
# Output: 2
# Explanation: The 2x2 grid is as follows:
# 
# 
# 
# 
# Example 2:
# 
# 
# Input:
# [
# " /",
# "  "
# ]
# Output: 1
# Explanation: The 2x2 grid is as follows:
# 
# 
# 
# 
# Example 3:
# 
# 
# Input:
# [
# "\\/",
# "/\\"
# ]
# Output: 4
# Explanation: (Recall that because \ characters are escaped, "\\/" refers to
# \/, and "/\\" refers to /\.)
# The 2x2 grid is as follows:
# 
# 
# 
# 
# Example 4:
# 
# 
# Input:
# [
# "/\\",
# "\\/"
# ]
# Output: 5
# Explanation: (Recall that because \ characters are escaped, "/\\" refers to
# /\, and "\\/" refers to \/.)
# The 2x2 grid is as follows:
# 
# 
# 
# 
# Example 5:
# 
# 
# Input:
# [
# "//",
# "/ "
# ]
# Output: 3
# Explanation: The 2x2 grid is as follows:
# 
# 
# 
# 
# 
# Note:
# 
# 
# 1 <= grid.length == grid[0].length <= 30
# grid[i][j] is either '/', '\', or ' '.
# 
# 
# 
# 
# 
# 
#

# @lc code=start
class Solution:
    def regionsBySlashes(self, grid) -> int:
        N = max(len(grid), max(len(row) for row in grid))
        ufs = DUS(N*N*4)

        for idx in range(N * N):
            row = idx // N
            col = idx % N
            ch = self.grid_get(grid, row, col)
            if ch == '/':
                ufs.union(4*idx+0, 4*idx+3)
                ufs.union(4*idx+1, 4*idx+2)
            elif ch == '\\':
                ufs.union(4*idx+0, 4*idx+1)
                ufs.union(4*idx+2, 4*idx+3)
            else:
                ufs.union(4*idx+0, 4*idx+1)
                ufs.union(4*idx+1, 4*idx+2)
                ufs.union(4*idx+2, 4*idx+3)

            if col < N - 1:
                ufs.union(4*idx+1, 4*idx+4+3)

            if row < N - 1:
                ufs.union(4*idx+2, 4*idx+4*N+0)

        return ufs.size

    def grid_get(self, grid, row, col):
        if row < len(grid):
            return grid[row][col] if col < len(grid[row]) else ""
        else:
            return ""


class DUS:
    def __init__(self, n):
        self.parent = [i for i in range(n)]
        self.rank = [0] * n
        self.size = n
    def find(self, x):
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]
    def union(self, u, v):
        pu = self.find(u)
        pv = self.find(v)
        if pu == pv:
            return False
        if self.rank[pu] > self.rank[pv]:
            self.parent[pv] = pu
        elif self.rank[pu] < self.rank[pv]:
            self.parent[pu] = pv
        else:
            self.parent[pv] = pu
            self.rank[pu] += 1

        self.size -= 1
        return True
# @lc code=end

if __name__ == '__main__':
    so = Solution()
    r = so.regionsBySlashes([" /", "/ "])
    assert  r == 2, r
