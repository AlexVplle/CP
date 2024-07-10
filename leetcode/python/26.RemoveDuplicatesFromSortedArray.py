class Solution:
    def removeElement(self, nums : list, val: int) -> int:
        start, end = 0, len(nums) - 1
        while start <= end :
            if nums[start] != val : start += 1
            elif nums[end] == val : end -= 1
            else :
                nums[start] = nums[end]
                start += 1
                end -= 1
        return start

#leetcode.com/problems/remove-duplicates-from-sorted-array/