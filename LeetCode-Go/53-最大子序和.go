package main

import "fmt"

func main() {
 fmt.Println(maxSubArray([]int{-2,1,-3,4,-1,2,1,-5,4}))
}
func maxSubArray(nums []int) int {
	sum := 0
	max_sum := nums[0]
	for i:=0;i < len(nums); i++ {
		if sum < 0{
			sum = nums[i]
		} else {
			sum += nums[i]
		}
		if sum > max_sum{
			max_sum = sum
		}
	}
	return max_sum
}

