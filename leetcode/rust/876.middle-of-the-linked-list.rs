// @leet start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast_option: Option<Box<ListNode>> = head.clone();
        while let Some(fast) = fast_option {
            fast_option = fast.next;
            if fast_option.is_none() {
                break;
            }
            head = head.unwrap().next;
            fast_option = fast_option.unwrap().next;
        }
        head
    }
}
// @leet end
