package main

import (
	"fmt"
)

func main() {
	fmt.Println(lengthOfLongestSubstring("abcabcbb"))
}
func lengthOfLongestSubstring(s string) int {
	sum_str := []byte{}
	i := 0
	j := 0
	max := 0
	lens := len(s)
	for ;i < lens && j < lens; {
		if findIndex(s[j],sum_str) == -1 {
			sum_str = append(sum_str, s[j])
			j ++
		} else {
			i++
			j = i
			sum_str = []byte{}
		}
		if max < len(sum_str){
			max = len(sum_str)
		}
	}
	return  max
}
/**
查询在元素在切片的位置
*/
func findIndex(key byte,s []byte) int{
	for i,value := range s {
		if value == key {
			return i
		}
	}
	return -1
}


