class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        sum = 0
        max_sum = nums[0]
        for i in range(0, len(nums)):
            if sum < 0:
                sum = nums[i]
            else:
                sum += nums[i]
            if sum > max_sum:
                max_sum = sum
        return max_sum
