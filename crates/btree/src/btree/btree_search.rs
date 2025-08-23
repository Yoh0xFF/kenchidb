use crate::btree::arena::{NodeId};
use crate::btree::btree::Btree;

/// Btree search implementation
impl Btree {
    pub fn search(&self, key: u64) -> Option<(NodeId, usize)> {
        self.recursive_search(self.root_id, key)
    }

    // Private methods
    fn recursive_search(&self, node_id: NodeId, key: u64) -> Option<(NodeId, usize)> {
        let mut index: usize = 0;
        let node = &self.arena.nodes[node_id];

        while index < node.n && node.keys[index] < key {
            index += 1;
        }

        if index < node.n && node.keys[index] == key {
            return Some((node_id, index));
        }

        if node.is_leaf {
            return None;
        }

        self.recursive_search(node.children_ids[index], key)
    }
}
