package main

func main1() {


}
/**
strconv
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
type ListNode struct {
	Val int
	Next *ListNode
}
func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
	if l1 == nil {
		return  l2
	}
	if l2 == nil {
		return l1
	}
	var result *ListNode
	if l1.Val >= l2.Val {
		result = l2
		result.Next = mergeTwoLists(l1,l2.Next)
	}
	if l2.Val >= l1.Val {
		result = l1
		result.Next = mergeTwoLists(l1.Next,l2)
	}
	return result
}
