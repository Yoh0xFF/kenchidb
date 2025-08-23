use crate::btree::arena::{NodeId};
use crate::btree::btree::Btree;

/// Btree search implementation
impl Btree {
    pub fn search(&self, key: u64) -> Option<(NodeId, usize)> {
        self.recursive_search(self.root_id, key)
    }

    // Private methods
    fn recursive_search(&self, id: NodeId, key: u64) -> Option<(NodeId, usize)> {
        let node = &self.arena.nodes[id];
        let mut k: usize = 0;

        while k < node.n && node.keys[k] < key {
            k += 1;
        }

        if k < node.n && node.keys[k] == key {
            return Some((id, k));
        }

        if node.is_leaf {
            return None;
        }

        self.recursive_search(node.children_ids[k], key)
    }
}
