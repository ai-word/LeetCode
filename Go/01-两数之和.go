package main
func twoSum(nums []int, target int) []int {
	var res = []int{-1,-2}
	for i:=0;i<len(nums);i++ {
		temp := target - nums[i]
		for j:=i+1;j<len(nums);j++ {
			if temp == nums[j]{
				res[0]=j
				res[1]=i
				return res
			}
		}
	}
	return  res
}

