impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut current = head;
        let mut index = 0;
        while let Some(node) = current {
            while let Some(&(_, value)) = stack.last() {
                if value > node.val {
                    break;
                }
                stack.pop();
            }

            if let Some(&(prev_index, _)) = stack.last() {
                while result.len() <= prev_index {
                    result.push(0);
                }
                result[prev_index] = node.val;
            }

            stack.push((index, node.val));
            index += 1;
            current = node.next;
        }
        while result.len() < index {
            result.push(0);
        }
        result
    }
}
