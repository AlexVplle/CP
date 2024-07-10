class Solution:
    def isValid(self, s: str) -> bool:
        stack, dictionnary = [], {')' : '(', '}' : '{', ']' : '['}
        for character in s :
            if character in dictionnary :
                if not stack or dictionnary.get(character) != stack.pop() : return False
            else : stack.append(character)
        return not stack

#leetcode.com/problems/valid-parentheses/