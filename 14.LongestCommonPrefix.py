class Solution:
    def longestCommonPrefix(strs : str):
        substring = strs[0]
        for word in strs:
            if len(word) < len(substring):
                word, substring = substring, word
            for i in range(len(substring)):
                if substring[i] != word[i]:
                    substring = substring[:i]
                    break
        return substring

# leetcode.com/problems/longest-common-prefix/