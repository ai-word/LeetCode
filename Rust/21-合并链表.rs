#[derive(PartialEq, Eq, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }
 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }
struct Solution{}
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let  l11 = l1.as_ref().unwrap();
        let  l22 = l2.as_ref().unwrap();
         if let Some(L1) = l1.as_ref() {
             println!("{:?}",L1);
         } else {
            return l2;
         }
         if let Some(L2) = l2.as_ref() {
             println!("{:?}",L2);
         } else {
            return l1;
         }
        let mut reslut = Some(Box::new(ListNode::new(-1)));

        if l11.val >= l22.val {
            reslut = l2;
            let mut res = reslut.as_ref().unwrap();
            let b = Solution::merge_two_lists(l1,l22.next);
            let mut  n = &res.next;
            n = &b;
//            println!("{:?}", &b);
//            println!("{:?}", n);
//            res.next = Solution::merge_two_lists(l1,l22.next);
        } else {
            reslut =l1;
//            let mut res = reslut.as_ref().unwrap();
//            res.next = Solution::merge_two_lists(l11.next,l2);
        }
        return reslut
    }
}

fn main() {
    let mut l1 = ListNode::new(12);
    let mut l11 = ListNode::new(30);
    l1.next = Some(Box::new(l11));
    let mut l3 = ListNode::new(12);
    let mut l33 = ListNode::new(50);
    l3.next = Some(Box::new(l33));
    Solution::merge_two_lists(Some(Box::new(l1)),Some(Box::new(l3)));
}


