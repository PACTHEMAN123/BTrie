pub struct BTrieNode<V> {
    pub index: usize,
    pub value: Option<V>,
    pub children: Vec<(usize, Box<BTrieNode<V>>)>,
    pub parent: Vec<(usize, Box<BTrieNode<V>>)>,
}

impl<V> BTrieNode<V> {
    pub fn new(value: Option<V>) -> Self {
        Self {
            index: 0,
            value,
            children: Vec::new(),
            parent: Vec::new(),
        }
    }

    pub fn get_child(&self, index: usize) -> Option<Self> {
        if let Ok(idx) = self.children.binary_search_by(|x| x.0.cmp(&index)) {
            return Some(self.children[idx].1)
        }
        None
    }
}