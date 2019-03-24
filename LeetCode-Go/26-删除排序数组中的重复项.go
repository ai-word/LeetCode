package main

func main1() {

}
func removeDuplicates(nums []int) int {
	if len(nums) < 2{
		return len(nums)
	}
	count := 0
	for i:=1;i<len(nums);i++{
		if nums[count] != nums[i]{
			count += 1
			nums[count] = nums[i]
		}
	}
	return count+1
}

