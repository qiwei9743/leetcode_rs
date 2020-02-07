#
# @lc app=leetcode id=222 lang=python3
#
# [222] Count Complete Tree Nodes
#
# https://leetcode.com/problems/count-complete-tree-nodes/description/
#
# algorithms
# Medium (39.94%)
# Likes:    1508
# Dislikes: 165
# Total Accepted:    171.9K
# Total Submissions: 429K
# Testcase Example:  '[1,2,3,4,5,6]'
#
# Given a complete binary tree, count the number of nodes.
# 
# Note: 
# 
# Definition of a complete binary tree from Wikipedia:
# In a complete binary tree every level, except possibly the last, is
# completely filled, and all nodes in the last level are as far left as
# possible. It can have between 1 and 2^h nodes inclusive at the last level h.
# 
# Example:
# 
# 
# Input: 
# ⁠   1
# ⁠  / \
# ⁠ 2   3
# ⁠/ \  /
# 4  5 6
# 
# Output: 6
# 
#

# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def countNodes(self, root: TreeNode) -> int:
        res = 0
        if root:
            left = self.count_left(root.left)
        while root:
            right = self.count_left(root.right)
            if left == right:
                res += self.calc_nodes(left) + 1
                root = root.right
                left = right - 1
            elif left > right:
                res += self.calc_nodes(right) + 1
                root = root.left
                left = left - 1
            else:
                assert False

        return res

    def calc_nodes(self, height):
        return 2**height - 1

    def count_left(self, root):
        cnt = 0
        while root:
            cnt += 1
            root = root.left
        return cnt

# @lc code=end
