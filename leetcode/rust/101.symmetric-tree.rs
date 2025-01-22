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
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root_node = root.unwrap();
        let root_borrow = root_node.borrow();
        Self::check_node(root_borrow.left.clone(), root_borrow.right.clone())
    }

    pub fn check_node(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                let (p_borrow, q_borrow) = (p_node.borrow(), q_node.borrow());
                p_borrow.val == q_borrow.val
                    && Self::check_node(p_borrow.left.clone(), q_borrow.right.clone())
                    && Self::check_node(p_borrow.right.clone(), q_borrow.left.clone())
            }
            _ => false,
        }
    }
}
// @leet end
