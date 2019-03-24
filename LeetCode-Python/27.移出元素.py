class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        if len(nums) == 0:
            return 0
        num = 0
        for i in range(len(nums)):
            if nums[i] != val:
                nums[num] = nums[i]
                num += 1
        return num
