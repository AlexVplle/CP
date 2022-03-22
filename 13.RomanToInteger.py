class Solution:
    def romanToInt(s : str):
        sumNumber, romanNumber = 0, {'CM' : 900, 'CD' : 400, 'XC' : 90, 'XL' : 40, 'IX' : 9, 'IV' : 4, 'M' : 1000, 'D' : 500, 'C' : 100, 'L' : 50, 'X' : 10, 'V' : 5, 'I' : 1}
        for key, values in romanNumber.items():
            while s.find(key) != -1:
                sumNumber += values
                s = s[:s.find(key)] + s[s.find(key) + len(key):]
        return sumNumber

# leetcode.com/problems/roman-to-integer/