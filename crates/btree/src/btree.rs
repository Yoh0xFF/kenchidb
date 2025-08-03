/// BTree
/// - Node keys separate the ranges of keys in each subtree.
///
/// - All leaves have the same depths, which is the tree's height.
///
/// - Nodes have minimum and maximum bounds on the number of keys they can contain.
///     We call it a minimum_degree of the tree.
///
/// - Every node other than the root must have at least (minimum_degree - 1) keys.
///     This means that every internal node has at least (minimum_degree) children.
///
/// - Every node may contain maximum (2 * minimum_degree - 1) keys.
///     This means that every node has maximum (2 * minimum_degree) children.
///     We say that the node is full if it contains exactly (2 * minimum_degree - 1) keys.
///
/// - The higher is (minimum_degree) of the three, the smaller is its height.
///
/// - The number of disk accesses required for most operations on a BTree,
///     is proportional to the height of the tree.
///

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct BtreeNode {
    id: NodeId,                // Node unique id
    n: usize,                  // Number of keys currently stored in the node
    leaf: bool,                // Indicator of the internal and leaf nodes
    keys: Vec<u64>,            // Node keys in monotonically increasing order key[i] <= key[i + 1]
    children_ids: Vec<NodeId>, // Node (number_of_keys + 1) pointers to the children
}

#[derive(Debug)]
pub struct Arena {
    nodes: Vec<BtreeNode>,
    free_list: Vec<NodeId>,
}

#[derive(Debug)]
pub struct Btree {
    md: usize,       // Minimum and maximum bounds on the number of keys
    arena: Arena,    // Arena for tree nodes
    root_id: NodeId, // Root of the tree
}

impl Arena {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            free_list: vec![],
        }
    }

    pub fn allocate_node(&mut self, md: usize) -> NodeId {
        if let Some(id) = self.free_list.pop() {
            return id;
        }

        let id = self.nodes.len();
        self.nodes.push(BtreeNode {
            id,
            n: 0,
            leaf: true,
            keys: vec![],
            children_ids: vec![],
        });
        self.nodes[id].keys.resize(2 * md - 1, 0);
        self.nodes[id].children_ids.resize(2 * md, 0);
        id
    }

    pub fn deallocate_node(&mut self, id: NodeId, md: usize) {
        self.free_list.push(id);
        self.nodes[id].n = 0;
        self.nodes[id].leaf = true;
        self.nodes[id].keys.clear();
        self.nodes[id].keys.resize(2 * md - 1, 0);
        self.nodes[id].children_ids.clear();
        self.nodes[id].children_ids.resize(2 * md, 0);
    }
}

impl Btree {
    pub fn new(minimum_degree: usize) -> Self {
        let mut arena = Arena::new();
        let id = arena.allocate_node(minimum_degree);

        Self {
            md: minimum_degree,
            arena,
            root_id: id,
        }
    }

    pub fn search(&self, key: u64) -> Option<(NodeId, u16)> {
        self.recursive_search(self.root_id, key)
    }

    pub fn insert() {
        todo!()
    }

    pub fn delete() {
        todo!()
    }

    // Private methods
    fn recursive_search(&self, node_id: NodeId, key: u64) -> Option<(NodeId, u16)> {
        let mut index: usize = 0;
        let node = &self.arena.nodes[node_id];

        while index < node.n && node.keys[index] < key {
            index += 1;
        }

        if index < node.n && node.keys[index] == key {
            return Some((node_id, index as u16));
        }

        if node.leaf {
            return None;
        }

        self.recursive_search(node.children_ids[index], key)
    }

    /// split creates a sibling node from a given node by splitting the node in two around a median.
    /// split will split the child at md leaving the [0, md-1] keys
    /// while moving the set of [md, 2md-1] keys to the sibling.
    fn split_child(&mut self, parent_id: NodeId, child_index: usize) {
        let new_sibling_id = self.arena.allocate_node(self.md);
        let nodes = self.arena.nodes.as_mut_slice();

        // **************************
        // * Work on the child node *
        // **************************

        // Get the child properties
        let child_id = nodes[parent_id].children_ids[child_index];
        let is_leaf = nodes[child_id].leaf;
        let median_key = nodes[child_id].keys[self.md - 1];

        // Set up the new sibling node
        nodes[new_sibling_id].leaf = is_leaf;
        nodes[new_sibling_id].n = self.md - 1;

        // Copy the upper half of keys from the child to the new sibling
        for i in 0..(self.md - 1) {
            nodes[new_sibling_id].keys[i] = nodes[child_id].keys[i + self.md];
        }

        // If not leaf, copy the upper half of the children pointers
        if !is_leaf {
            for i in 0..self.md {
                nodes[new_sibling_id].children_ids[i] = nodes[child_id].children_ids[i + self.md];
            }
        }

        // Update the original child's key count
        nodes[child_id].n = self.md - 1;

        // ***************************
        // * Work on the parent node *
        // ***************************

        // Shift existing children pointers to make room for the new sibling
        for i in (child_index + 1..=nodes[parent_id].n).rev() {
            nodes[parent_id].children_ids[i + 1] = nodes[parent_id].children_ids[i];
        }
        nodes[parent_id].children_ids[child_index + 1] = new_sibling_id;

        // Shift existing keys in the parent node to make room for the median key
        for i in (child_index..nodes[parent_id].n).rev() {
            nodes[parent_id].keys[i + 1] = nodes[parent_id].keys[i];
        }
        nodes[parent_id].keys[child_index] = median_key;

        // Increment parent node's key count
        nodes[parent_id].n += 1;
    }
}
