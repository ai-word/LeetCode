package main

import "fmt"

func main() {
 a := []int{1,2,3,4,5,6}
 fmt.Println(searchInsert(a,5))
}

func searchInsert(nums []int, target int) int {
	low := 0
	high := len(nums)
	for ;low<high; {
		mid := low + (high - low)/2
		if nums[mid] > target{
			high = mid
		} else if nums[mid] < target {
			low = mid + 1
		} else {
			return mid
		}
	}
	return low
}
