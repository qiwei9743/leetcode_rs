#
# @lc app=leetcode id=653 lang=python3
#
# [653] Two Sum IV - Input is a BST
#
# https://leetcode.com/problems/two-sum-iv-input-is-a-bst/description/
#
# algorithms
# Easy (53.76%)
# Likes:    1085
# Dislikes: 121
# Total Accepted:    111.9K
# Total Submissions: 207.7K
# Testcase Example:  '[5,3,6,2,4,null,7]\n9'
#
# Given a Binary Search Tree and a target number, return true if there exist
# two elements in the BST such that their sum is equal to the given target.
# 
# Example 1:
# 
# 
# Input: 
# ⁠   5
# ⁠  / \
# ⁠ 3   6
# ⁠/ \   \
# 2   4   7
# 
# Target = 9
# 
# Output: True
# 
# 
# 
# 
# Example 2:
# 
# 
# Input: 
# ⁠   5
# ⁠  / \
# ⁠ 3   6
# ⁠/ \   \
# 2   4   7
# 
# Target = 28
# 
# Output: False
# 
# 
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
    def findTarget(self, root: TreeNode, k: int) -> bool:
        dummy = TreeNode(-1)
        dummy.left = root
        dummy.right = root
        ls, rs = ([dummy], [dummy])
        self.next_item(ls, True)
        self.next_item(rs, False)
        while len(ls) > 0 and len(rs) > 0:
            if ls[-1] is rs[-1]:
                return False
            t = ls[-1].val + rs[-1].val
            if t == k:
                return True
            elif t < k:
                self.next_item(ls, True)
            else:
                self.next_item(rs, False)
        return False

    def next_item(self, stack, is_left):
        root = stack.pop().right if is_left else stack.pop().left
        while root:
            stack.append(root)
            root = root.left if is_left else root.right

    # def lr_next(self, stack):
    #     root = stack.pop().right
    #     while root:
    #         stack.append(root)
    #         root = root.left

    # def rl_next(self, stack):
    #     root = stack.pop().left
    #     while root:
    #         stack.append(root)
    #         root = root.right
    
# @lc code=end
