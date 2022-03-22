class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        if not len(needle) : return 0
        return haystack.find(needle)

#https://leetcode.com/problems/implement-strstr/