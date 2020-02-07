#
# @lc app=leetcode id=146 lang=python3
#
# [146] LRU Cache
#
# https://leetcode.com/problems/lru-cache/description/
#
# algorithms
# Medium (29.29%)
# Likes:    4437
# Dislikes: 190
# Total Accepted:    422.7K
# Total Submissions: 1.4M
# Testcase Example:  '["LRUCache","put","put","get","put","get","put","get","get","get"]\n' +
  '[[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]'
#
# Design and implement a data structure for Least Recently Used (LRU) cache. It
# should support the following operations: get and put.
# 
# get(key) - Get the value (will always be positive) of the key if the key
# exists in the cache, otherwise return -1.
# put(key, value) - Set or insert the value if the key is not already present.
# When the cache reached its capacity, it should invalidate the least recently
# used item before inserting a new item.
# 
# The cache is initialized with a positive capacity.
# 
# Follow up:
# Could you do both operations in O(1) time complexity?
# 
# Example:
# 
# 
# LRUCache cache = new LRUCache( 2 /* capacity */ );
# 
# cache.put(1, 1);
# cache.put(2, 2);
# cache.get(1);       // returns 1
# cache.put(3, 3);    // evicts key 2
# cache.get(2);       // returns -1 (not found)
# cache.put(4, 4);    // evicts key 1
# cache.get(1);       // returns -1 (not found)
# cache.get(3);       // returns 3
# cache.get(4);       // returns 4
# 
# 
# 
# 
#

# @lc code=start

class DNode:
    def __init__(self, key, val):
        self.key = key
        self.val = val
        self.prev = None
        self.next = None

    def remove(self):
        if self.prev and self.next:
            self.prev.next = self.next
            self.next.prev = self.prev
            self.prev = self.next = None

class DList:
    def __init__(self):
        self.head = DNode(-1, -1)
        self.head.prev = self.head.next = self.head
        self.size = 0

    def pop_head(self):
        n = self.head.next
        self.remove_node(n)
        return n


    def append_tail(self, new):
        new.next = self.head
        new.prev = self.head.prev

        self.head.prev.next = new
        self.head.prev = new

        self.size += 1

    def remove_node(self, n):
        n.remove()
        self.size -= 1


class LRUCache:

    def __init__(self, capacity: int):
        self.map = {}
        self.capacity = capacity
        self.dl = DList()

    def get(self, key: int) -> int:
        if key in self.map:
            node = self.map[key]
            self.dl.remove_node(node)
            self.dl.append_tail(node)
            return node.val
        else:
            return -1

    def put(self, key: int, value: int) -> None:
        if key in self.map:
            node = self.map[key]
            self.dl.remove_node(node)
            self.dl.append_tail(node)
            node.val = value
        elif self.capacity > 0:
            if self.dl.size == self.capacity:
                node = self.dl.pop_head()
                del self.map[node.key]
                node.key = key
                node.val = value
            else:
                node = DNode(key, value)

            self.map[node.key] = node
            self.dl.append_tail(node)


# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)
# @lc code=end
