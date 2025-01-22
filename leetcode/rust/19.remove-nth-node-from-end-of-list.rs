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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut i: i32 = 0;
        let mut front_option: &Option<Box<ListNode>> = &head;
        let mut back_option: &Option<Box<ListNode>> = &head;
        while let Some(front) = front_option {
            if i > n {
                back_option = &back_option.unwrap().next;
            }
            i += 1;
            front_option = &front.next;
        }
        // println!("{}", back_option.unwrap().val);
        // back_option.unwrap().next = None;
        head
    }
}
// @leet end
