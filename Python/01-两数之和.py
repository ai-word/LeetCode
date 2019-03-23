class Solution:
   def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        result_nums = []
        length = len(nums)
        for x in range(length):
            for y in range(x+1,length):
                if nums[x] + nums[y] == target:
                    result_nums.append(x)
                    result_nums.append(y)
                        
        return result_nums
