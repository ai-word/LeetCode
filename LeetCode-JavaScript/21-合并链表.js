function ListNode(val) {
    this.val = val;
    this.next = null;
}
var mergeTwoLists = function(l1, l2) {
    if (l1 == null) {
        return l2
    }
    if (l2 == null) {
        return l1
    }
    var result = ListNode(null);
    if (l1.val >= l2.val) {
        result = l2
        result.next = mergeTwoLists(l1,l2.next)
    } else {
        result = l1
        result.next = mergeTwoLists(l1.next,l2)
    }
    return result
};