#
# @lc app=leetcode id=721 lang=python3
#
# [721] Accounts Merge
#
# https://leetcode.com/problems/accounts-merge/description/
#
# algorithms
# Medium (44.78%)
# Likes:    956
# Dislikes: 244
# Total Accepted:    56.8K
# Total Submissions: 126.2K
# Testcase Example:  '[["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]'
#
# Given a list accounts, each element accounts[i] is a list of strings, where
# the first element accounts[i][0] is a name, and the rest of the elements are
# emails representing emails of the account.
# 
# Now, we would like to merge these accounts.  Two accounts definitely belong
# to the same person if there is some email that is common to both accounts.
# Note that even if two accounts have the same name, they may belong to
# different people as people could have the same name.  A person can have any
# number of accounts initially, but all of their accounts definitely have the
# same name.
# 
# After merging the accounts, return the accounts in the following format: the
# first element of each account is the name, and the rest of the elements are
# emails in sorted order.  The accounts themselves can be returned in any
# order.
# 
# Example 1:
# 
# Input: 
# accounts = [["John", "johnsmith@mail.com", "john00@mail.com"], ["John",
# "johnnybravo@mail.com"], ["John", "johnsmith@mail.com",
# "john_newyork@mail.com"], ["Mary", "mary@mail.com"]]
# Output: [["John", 'john00@mail.com', 'john_newyork@mail.com',
# 'johnsmith@mail.com'],  ["John", "johnnybravo@mail.com"], ["Mary",
# "mary@mail.com"]]
# Explanation: 
# The first and third John's are the same person as they have the common email
# "johnsmith@mail.com".
# The second John and Mary are different people as none of their email
# addresses are used by other accounts.
# We could return these lists in any order, for example the answer [['Mary',
# 'mary@mail.com'], ['John', 'johnnybravo@mail.com'], 
# ['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']]
# would still be accepted.
# 
# 
# 
# Note:
# The length of accounts will be in the range [1, 1000].
# The length of accounts[i] will be in the range [1, 10].
# The length of accounts[i][j] will be in the range [1, 30].
# 
#

# @lc code=start
class Solution:
    def accountsMerge(self, accounts: List[List[str]]) -> List[List[str]]:
        import bisect
        ufs = UnionFindSet()
        for acc in accounts:
            for p, n in zip(acc[1:], acc[2:]):
                ufs.union(p, n)

        group = {}
        for acc in accounts:
            name = acc[0]
            for email in acc[1:]:
                # print(f"find {email}'s parent {ufs.find(email)}")
                # group.setdefault(ufs.find(email), [name, set()])[1].add(email)
                arr = group.setdefault(ufs.find(email), [name])
                ti = bisect.bisect_left(arr, email, 1, len(arr))
                if not (ti < len(arr) and arr[ti] == email):
                    arr.insert(ti, email)

        # print(group)
        # return [[name]+sorted(list(values)) for name, values in group.values()]
        return list(group.values())

class UnionFindSet:
    def __init__(self):
        self.parent = {}
        self.rank = {}

    def find(self, x):
        if x not in self.parent:
            self.parent[x] = x
            self.rank[x] = 0

        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, u, v):
        # print(u, v)
        pu = self.find(u)
        pv = self.find(v)
        if pu == pv:
            return False
        ru = self.rank[pu]
        rv = self.rank[pv]
        # print(f"ru={ru} rv={rv}")
        if ru > rv:
            self.parent[pv] = pu
        elif ru < rv:
            self.parent[pu] = pv
        else:
            self.parent[pv] = pu
            self.rank[pu] += 1

        # print(f"pu={pu} pv={pv}")
        # print(self.parent, self.rank)
        return True

# @lc code=end

