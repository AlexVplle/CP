// @leet start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(root_node) = root {
            queue.push_back(root_node);
        }
        while !queue.is_empty() {
            let level_size: usize = queue.len();
            let mut level_nodes: Vec<i32> = Vec::with_capacity(level_size);
            for _ in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();
                level_nodes.push(node_ref.val);
                if let Some(ref left) = node_ref.left {
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = node_ref.right {
                    queue.push_back(right.clone());
                }
            }
            result.push(level_nodes);
        }

        result
    }
}
// @leet end
