pub type NodeId = usize;

#[derive(Debug, Clone)]
pub(super) struct BtreeNode {
    pub(super) id: NodeId,                // Node unique id
    pub(super) n: usize,                  // Number of keys currently stored in the node
    pub(super) leaf: bool,                // Indicator of the internal and leaf nodes
    pub(super) keys: Vec<u64>,            // Node keys in monotonically increasing order key[i] <= key[i + 1]
    pub(super) children_ids: Vec<NodeId>, // Node (number_of_keys + 1) pointers to the children
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
            leaf: true,
            keys: vec![],
            children_ids: vec![],
        });
        self.nodes[id].keys.resize(2 * t - 1, 0);
        self.nodes[id].children_ids.resize(2 * t, 0);
        id
    }

    pub fn deallocate_node(&mut self, id: NodeId, t: usize) {
        self.free_list.push(id);
        self.nodes[id].n = 0;
        self.nodes[id].leaf = true;
        self.nodes[id].keys.clear();
        self.nodes[id].keys.resize(2 * t - 1, 0);
        self.nodes[id].children_ids.clear();
        self.nodes[id].children_ids.resize(2 * t, 0);
    }
}
