impl<V> BTrie<V> {
    pub fn new(order: usize) -> Self {
        Self {
            root: BTrieNode::new(None),
            order
        }
    }

    pub fn get(&self, index: usize) -> Option<V> {
        let node = self.traversal(index, QueryMode::Find);
        if let Some(node) = node {
            node.value
        } else {
            None
        }
    }

    pub fn traversal(&self, key: usize, mode: QueryMode) -> Option<BTrieNode<V>> {

        let mut lz = if key == 0 {
            0
        } else {
            (MSBITL - key.leading_zeros() as usize) / self.order
        };
        lz = lz * self.order;
        let mut bitmask = ((1 << self.order) - 1) << lz;

        let mut current_node = Some(self.root);

        while bitmask != 0 && current_node.is_some() {
            let i = (key & bitmask) >> lz;
            let subtree = current_node.get_child(i);
        }
    }
}