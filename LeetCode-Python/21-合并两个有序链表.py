class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        if l1 == None:
            return l2
        if l2 == None:
            return l1
        result = ListNode(None)
        if l1.val >= l2.val:
            result = l2
            result.next = self.mergeTwoLists(l1,l2.next)
        else:
            result = l1
            result.next = self.mergeTwoLists(l1.next, l2)
            
        return result
