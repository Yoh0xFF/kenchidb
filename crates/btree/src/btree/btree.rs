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

    pub(super) fn is_root(&self, node_id: NodeId) -> bool {
        node_id == self.root_id
    }

    pub(super) fn is_node_full(&self, node_id: NodeId) -> bool {
        let node = &self.arena.nodes[node_id];
        node.n == 2 * self.t - 1
    }

    pub(super) fn is_node_underflow(&self, node_id: NodeId) -> bool {
        if self.is_root(node_id) {
            return false;
        }

        let node = &self.arena.nodes[node_id];
        node.n == self.t - 1
    }
}
