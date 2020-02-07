#
# @lc app=leetcode id=95 lang=python3
#
# [95] Unique Binary Search Trees II
#
# https://leetcode.com/problems/unique-binary-search-trees-ii/description/
#
# algorithms
# Medium (38.27%)
# Likes:    1717
# Dislikes: 138
# Total Accepted:    168.1K
# Total Submissions: 438.8K
# Testcase Example:  '3'
#
# Given an integer n, generate all structurally unique BST's (binary search
# trees) that store values 1 ... n.
# 
# Example:
# 
# 
# Input: 3
# Output:
# [
# [1,null,3,2],
# [3,2,null,1],
# [3,1,null,null,2],
# [2,1,3],
# [1,null,2,null,3]
# ]
# Explanation:
# The above output corresponds to the 5 unique BST's shown below:
# 
# ⁠  1         3     3      2      1
# ⁠   \       /     /      / \      \
# ⁠    3     2     1      1   3      2
# ⁠   /     /       \                 \
# ⁠  2     1         2                 3
# 
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
    def generateTrees(self, n: int) -> List[TreeNode]:
        return [] if n == 0 else self.helper(1, n+1)

    def helper(self, _from, _to):
        if _from == _to:
            return [None]
        if _from + 1 == _to:
            return [TreeNode(_from)]
        res = []

        for i in range(_from, _to):
            lefts = self.helper(_from, i)
            rights = self.helper(i+1, _to)
            self.product(i, lefts, rights, res)

        return res

    def product(self, root_num, lefts, rights, res):
        for l in lefts:
            for r in rights:
                root = TreeNode(root_num)
                root.left = l
                root.right = r
                res.append(root)

# @lc code=end
