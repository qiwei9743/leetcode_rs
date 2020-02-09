#
# @lc app=leetcode id=130 lang=python3
#
# [130] Surrounded Regions
#
# https://leetcode.com/problems/surrounded-regions/description/
#
# algorithms
# Medium (25.15%)
# Likes:    1158
# Dislikes: 534
# Total Accepted:    182.8K
# Total Submissions: 725.6K
# Testcase Example:  '[["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]'
#
# Given a 2D board containing 'X' and 'O' (the letter O), capture all regions
# surrounded by 'X'.
# 
# A region is captured by flipping all 'O's into 'X's in that surrounded
# region.
# 
# Example:
# 
# 
# X X X X
# X O O X
# X X O X
# X O X X
# 
# 
# After running your function, the board should be:
# 
# 
# X X X X
# X X X X
# X X X X
# X O X X
# 
# 
# Explanation:
# 
# Surrounded regions shouldn’t be on the border, which means that any 'O' on
# the border of the board are not flipped to 'X'. Any 'O' that is not on the
# border and it is not connected to an 'O' on the border will be flipped to
# 'X'. Two cells are connected if they are adjacent cells connected
# horizontally or vertically.
# 
#

# @lc code=start
class Solution:
    def solve(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        def dfs(coordinate_i):
            x, y = i2xy(coordinate_i)
            if board[x][y] == 'X':
                return
            if keep0set[coordinate_i]:
                return
            keep0set[coordinate_i] = True
            for direction in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                new_cor = (x + direction[0],
                           y + direction[1])
                new_cor_i = xy2i(*new_cor)
                if (0 <= new_cor[0] < M and 0 <= new_cor[1] < N
                    and not visited[new_cor_i]):

                    visited[new_cor_i] = True
                    dfs(new_cor_i)
                    visited[new_cor_i] = False

        def xy2i(x, y):
            return x * N + y

        def i2xy(i):
            return (i // N, i % N)

        M = len(board)
        N = len(board[0]) if M > 0 else 0
        keep0set = [False] * (M * N)
        visited = [False] * (M * N)

        for j in range(N):
            dfs(xy2i(0, j))
            dfs(xy2i(M - 1, j))

        for i in range(M):
            dfs(xy2i(i, 0))
            dfs(xy2i(i, N-1))

        for i in range(M):
            for j in range(N):
                cor = (i, j)
                cor_i = xy2i(i, j)
                if board[i][j] != 'X' and not keep0set[cor_i]:
                    board[i][j] = 'X'


# @lc code=end
