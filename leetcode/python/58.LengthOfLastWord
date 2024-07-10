class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        lengthString, result = len(s) - 1, 0
        while lengthString >= 0 :
            if s[lengthString] != " " : result += 1
            elif result > 0 : return result
            lengthString -= 1
        return result
