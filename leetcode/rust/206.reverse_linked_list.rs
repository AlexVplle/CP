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
