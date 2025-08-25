pub type NodeId = usize;

#[derive(Debug, Clone)]
pub(super) struct BtreeNode {
    pub(super) id: NodeId,                // Node unique id
    pub(super) n: usize,                  // Number of keys currently stored in the node
    pub(super) is_leaf: bool,             // Indicator of the internal and leaf nodes
    pub(super) keys: Vec<u64>, // Node keys in monotonically increasing order key[i] <= key[i + 1]
    pub(super) children: Vec<NodeId>, // Node (number_of_keys + 1) pointers to the children
}

impl BtreeNode {
    pub(super) fn find_key_index(&self, key: u64) -> Option<usize> {
        self.keys.iter().position(|&x| x == key)
    }

    pub(super) fn find_child_index(&self, key: u64) -> usize {
        let mut child_index = 0;

        while child_index < self.n && self.keys[child_index] < key {
            child_index += 1;
        }

        child_index
    }
}

#[derive(Debug)]
pub(super) struct Arena {
    pub(super) nodes: Vec<BtreeNode>,
    free_list: Vec<NodeId>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            free_list: vec![],
        }
    }

    pub fn allocate_node(&mut self, t: usize) -> NodeId {
        if let Some(id) = self.free_list.pop() {
            return id;
        }

        let id = self.nodes.len();
        self.nodes.push(BtreeNode {
            id,
            n: 0,
            is_leaf: true,
            keys: vec![],
            children: vec![],
        });
        self.nodes[id].keys.resize(2 * t - 1, 0);
        self.nodes[id].children.resize(2 * t, 0);
        id
    }

    pub fn deallocate_node(&mut self, id: NodeId, t: usize) {
        self.free_list.push(id);
        self.nodes[id].n = 0;
        self.nodes[id].is_leaf = true;
        self.nodes[id].keys.clear();
        self.nodes[id].keys.resize(2 * t - 1, 0);
        self.nodes[id].children.clear();
        self.nodes[id].children.resize(2 * t, 0);
    }
}
