package main

import "fmt"

func main1() {
l := removeElement([]int{3,2,2,3},3)
 fmt.Println(l)
}
func removeElement(nums []int, val int) int {
	if len(nums) == 0 {
		return 0
	}
	num := 0
	for i:=0;i<len(nums);i++{
		if nums[i] != val {
			nums[num] = nums[i]
			num ++
		}
	}
	return num
}
//func removeElement(nums []int, val int) int {
//	if len(nums) == 0 {
//		return 0
//	}
//	for i:=0;i<=len(nums)-1;{
//		if nums[i] == val {
//			nums = append(nums[:i],nums[i+1:]...)
//		} else {
//			i++
//		}
//	}
//	fmt.Println(nums)
//	return len(nums)
//}


