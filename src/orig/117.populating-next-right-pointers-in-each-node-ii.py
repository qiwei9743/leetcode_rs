#
# @lc app=leetcode id=117 lang=python3
#
# [117] Populating Next Right Pointers in Each Node II
#
# https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/description/
#
# algorithms
# Medium (36.85%)
# Likes:    1239
# Dislikes: 167
# Total Accepted:    222.1K
# Total Submissions: 601.8K
# Testcase Example:  '[1,2,3,4,5,null,7]'
#
# Given a binary tree
# 
# 
# struct Node {
# ⁠ int val;
# ⁠ Node *left;
# ⁠ Node *right;
# ⁠ Node *next;
# }
# 
# 
# Populate each next pointer to point to its next right node. If there is no
# next right node, the next pointer should be set to NULL.
# 
# Initially, all next pointers are set to NULL.
# 
# 
# 
# Follow up:
# 
# 
# You may only use constant extra space.
# Recursive approach is fine, you may assume implicit stack space does not
# count as extra space for this problem.
# 
# 
# 
# Example 1:
# 
# 
# 
# 
# Input: root = [1,2,3,4,5,null,7]
# Output: [1,#,2,3,#,4,5,7,#]
# Explanation: Given the above binary tree (Figure A), your function should
# populate each next pointer to point to its next right node, just like in
# Figure B. The serialized output is in level order as connected by the next
# pointers, with '#' signifying the end of each level.
# 
# 
# 
# Constraints:
# 
# 
# The number of nodes in the given tree is less than 6000.
# -100 <= node.val <= 100
# 
# 
#

# @lc code=start
"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""
class Solution:
    def connect(self, root: 'Node') -> 'Node':
        if root is None:
            return None

        if root.left:
            if root.right:
                root.left.next = root.right
            else:
                root.left.next = self.find_next_child(root.next)

        if root.right:
            root.right.next = self.find_next_child(root.next)


        self.connect(root.right)
        self.connect(root.left)
        return root

    def find_next_child(self, root):
        while root:
            if root.left:
                return root.left
            if root.right:
                return root.right
            root = root.next
        return None

    def connect_incorrect(self, root: 'Node') -> 'Node':
        self.root = root

        def dfs(root, level, idx):
            if root is None:
                return
            root.next = self.find(level, idx+1)
            dfs(root.left, level+1, 2*idx)
            dfs(root.right, level+1, 2*idx+1)

        dfs(root, 0, 0)
        return root

    def find(self, target_level, target_index):
        root = self.root
        level = target_level
        index = target_index
        index_info = []
        while level > -1:
            index_info.append(index)
            index //= 2
            level -= 1

        if index_info.pop() > 0:
            return None
        cur_idx = 0
        for idx in index_info[::-1]:
            if root is None:
                return None
            if cur_idx * 2 == idx:
                root = root.left
            elif cur_idx * 2 + 1 == idx:
                root = root.right
        return root

# @lc code=end
