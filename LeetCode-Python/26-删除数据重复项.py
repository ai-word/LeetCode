class Solution:
    def removeDuplicates(self, nums: object) -> int:
        if len(nums) < 2:
            return len(nums)
        count = 0
        arr = []
        for i in range(1,len(nums)):
            if nums[count] != nums[i]:
                count += 1
                nums[count] = nums[i]
                arr.append(nums[i])

        print(arr)
        return count+1

b = Solution.removeDuplicates(None,[1,1,12,45,45,89,99,99,99,99])
