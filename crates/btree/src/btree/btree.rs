use crate::btree::arena::{Arena, NodeId};

/// BTree
/// - Node keys separate the ranges of keys in each subtree.
///
/// - All leaves have the same depths, which is the tree's height.
///
/// - Nodes have minimum and maximum bounds on the number of keys they can contain.
///     We call it a minimum degree of the tree, and assign it to t variable.
///
/// - Every node other than the root must have at least (t - 1) keys.
///     This means that every internal node has at least (t) children.
///
/// - Every node may contain maximum (2 * t - 1) keys.
///     This means that every node has maximum (2 * t) children.
///     We say that the node is full if it contains exactly (2 * t - 1) keys.
///
/// - The higher is (t) of the three, the smaller is its height.
///
/// - The number of disk accesses required for most operations on a BTree,
///     is proportional to the height of the tree.
///

#[derive(Debug)]
pub struct Btree {
    pub(super) t: usize,        // Minimum and maximum bounds on the number of keys
    pub(super) arena: Arena,    // Arena for tree nodes
    pub(super) root_id: NodeId, // Root of the tree
}

impl Btree {
    pub fn new(minimum_degree: usize) -> Self {
        let mut arena = Arena::new();
        let id = arena.allocate_node(minimum_degree);

        Self {
            t: minimum_degree,
            arena,
            root_id: id,
        }
    }

    pub(super) fn find_key_index(&self, node_id: NodeId, key: u64) -> Option<usize> {
        self.arena.nodes[node_id]
            .keys
            .iter()
            .position(|&x| x == key)
    }

    pub(super) fn find_child_index(&self, node_id: NodeId, key: u64) -> usize {
        let node = &self.arena.nodes[node_id];
        let mut child_index = 0;

        while child_index < node.n && node.keys[child_index] < key {
            child_index += 1;
        }

        child_index
    }
}
