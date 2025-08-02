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
    number_of_keys: u16,       // Number of keys currently stored in the node
    is_leaf: bool,             // Indicator of the internal and leaf nodes
    keys: Vec<u64>,            // Node keys in monotonically increasing order key[i] <= key[i + 1]
    children_ids: Vec<NodeId>, // Node (number_of_keys + 1) pointers to the children
}

#[derive(Debug, Clone)]
pub struct Arena {
    nodes: Vec<BtreeNode>,
    free_list: Vec<NodeId>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            free_list: vec![],
        }
    }

    pub fn allocate_node(&mut self) -> NodeId {
        if let Some(id) = self.free_list.pop() {
            return id;
        }

        let id = self.nodes.len();
        self.nodes.push(BtreeNode {
            id,
            number_of_keys: 0,
            is_leaf: true,
            keys: vec![],
            children_ids: vec![],
        });
        id
    }

    pub fn deallocate_node(&mut self, id: NodeId) {
        self.free_list.push(id);
        self.nodes[id] = BtreeNode {
            id,
            number_of_keys: 0,
            is_leaf: true,
            keys: vec![],
            children_ids: vec![],
        };
    }
}

#[derive(Debug, Clone)]
pub struct Btree {
    minimum_degree: u16, // Minimum and maximum bounds on the number of keys
    arena: Arena,        // Arena for tree nodes
    root_id: NodeId,     // Root of the tree
}

impl Btree {
    pub fn new(minimum_degree: u16) -> Self {
        let mut arena = Arena::new();
        let id = arena.allocate_node();

        Self {
            minimum_degree,
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

        while index < node.number_of_keys as usize && node.keys[index] < key {
            index += 1;
        }

        if index < node.number_of_keys as usize && node.keys[index] == key {
            return Some((node_id, index as u16));
        }

        if node.is_leaf {
            return None;
        }

        self.recursive_search(node.children_ids[index], key)
    }

    fn split_child(&mut self, parent_id: NodeId, index: usize) {
        todo!()
    }
}
