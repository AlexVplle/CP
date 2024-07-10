class Solution:
    def searchInsert(self, nums: list, target: int) -> int:
        end = len(nums) - 1
        mid = end // 2
        if not nums : return 0
        valueMid = nums[mid]
        if valueMid == target: return mid
        if len(nums) == 1 : 
            if nums[0] < target : return 1
            return 0
        if valueMid > target : return self.searchInsert(nums[:mid], target)
        else : return mid + 1 + self.searchInsert(nums[mid+1:], target)