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

impl solution {
    pub fn reverse_list(head: option<box<listnode>>) -> option<box<listnode>> {
        let mut prev: option<box<listnode>> = none;
        let mut current: option<box<listnode>> = head;
        while let some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = some(node);
        }
        prev
    }
}
// @leet end

