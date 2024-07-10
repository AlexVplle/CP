class Solution:
    def twoSum(nums : list, target : int):
        hashmap = {}
        for index, value in enumerate(nums):
            complement = target - value
            if complement in hashmap:
                return [index, hashmap[complement]]
            hashmap[value] = index

# leetcode.com/problems/two-sum/