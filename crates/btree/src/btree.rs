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
    t: usize,        // Minimum and maximum bounds on the number of keys
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

    pub fn search(&self, key: u64) -> Option<(NodeId, usize)> {
        self.recursive_search(self.root_id, key)
    }

    /// O(h) disk access
    /// O(md * h) = O(md * log.md(n)) CPU time
    pub fn insert(&mut self, key: u64) {
        let t = self.t;

        if self.arena.nodes[self.root_id].n == 2 * t - 1 {
            let new_root_id = self.split_root();
            self.recursive_insert(new_root_id, key);
        } else {
            self.recursive_insert(self.root_id, key);
        }
    }

    pub fn delete(&mut self, key: u64) {
        self.recursive_delete(self.root_id, key);
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

        if node.leaf {
            return None;
        }

        self.recursive_search(node.children_ids[index], key)
    }

    fn recursive_insert(&mut self, node_id: NodeId, key: u64) {
        let t = self.t;
        let n = self.arena.nodes[node_id].n;

        if self.arena.nodes[node_id].leaf {
            // inserting into a leaf
            let mut pos = 0;

            // find insertion position
            while pos < n && self.arena.nodes[node_id].keys[pos] < key {
                pos += 1;
            }

            // shift keys and insert
            for i in (pos..n).rev() {
                self.arena.nodes[node_id].keys[i + 1] = self.arena.nodes[node_id].keys[i];
            }
            self.arena.nodes[node_id].keys[pos] = key;
            self.arena.nodes[node_id].n += 1;
        } else {
            // find the child where key belongs
            let mut pos = 0;

            // find insertion position
            while pos < n && key > self.arena.nodes[node_id].keys[pos] {
                pos += 1;
            }

            let child_id = self.arena.nodes[node_id].children_ids[pos];
            if self.arena.nodes[child_id].n == 2 * t - 1 {
                // split the child if it is full
                self.split_child(node_id, pos);
                if key > self.arena.nodes[node_id].keys[pos] {
                    // does key go into child[i] or child[i + 1]?
                    pos += 1;
                }
            }

            let child_id = self.arena.nodes[node_id].children_ids[pos];
            self.recursive_insert(child_id, key);
        }
    }

    fn split_root(&mut self) -> NodeId {
        let t = self.t;

        // allocate the new root
        let new_root_id = self.arena.allocate_node(t);

        // set new root properties
        self.arena.nodes[new_root_id].leaf = false;
        self.arena.nodes[new_root_id].n = 0;
        self.arena.nodes[new_root_id].children_ids[0] = self.root_id;

        // overwrite the old root and split it
        self.root_id = new_root_id;
        self.split_child(new_root_id, 0);

        new_root_id
    }

    /// split creates a sibling node from a given node by splitting the node in two around a median.
    /// split will split the child at md leaving the [0, md-1] keys
    /// while moving the set of [md, 2md-1] keys to the sibling.
    fn split_child(&mut self, parent_id: NodeId, child_index: usize) {
        let t = self.t;
        let new_sibling_id = self.arena.allocate_node(t);

        // **************************
        // * Work on the child node *
        // **************************

        // Get the child properties
        let child_id = self.arena.nodes[parent_id].children_ids[child_index];
        let is_leaf = self.arena.nodes[child_id].leaf;
        let median_key = self.arena.nodes[child_id].keys[t - 1];

        // Set up the new sibling node
        self.arena.nodes[new_sibling_id].leaf = is_leaf;
        self.arena.nodes[new_sibling_id].n = t - 1;

        // Copy the upper half of keys from the child to the new sibling
        for i in 0..(t - 1) {
            self.arena.nodes[new_sibling_id].keys[i] = self.arena.nodes[child_id].keys[i + t];
        }

        // If not leaf, copy the upper half of the children pointers
        if !is_leaf {
            for i in 0..t {
                self.arena.nodes[new_sibling_id].children_ids[i] =
                    self.arena.nodes[child_id].children_ids[i + t];
            }
        }

        // Update the original child's key count
        self.arena.nodes[child_id].n = t - 1;

        // ***************************
        // * Work on the parent node *
        // ***************************

        // Shift existing children pointers to make room for the new sibling
        for i in (child_index + 1..=self.arena.nodes[parent_id].n).rev() {
            self.arena.nodes[parent_id].children_ids[i + 1] =
                self.arena.nodes[parent_id].children_ids[i];
        }
        self.arena.nodes[parent_id].children_ids[child_index + 1] = new_sibling_id;

        // Shift existing keys in the parent node to make room for the median key
        for i in (child_index..self.arena.nodes[parent_id].n).rev() {
            self.arena.nodes[parent_id].keys[i + 1] = self.arena.nodes[parent_id].keys[i];
        }
        self.arena.nodes[parent_id].keys[child_index] = median_key;

        // Increment parent node's key count
        self.arena.nodes[parent_id].n += 1;
    }

    fn recursive_delete(&mut self, node_id: NodeId, key: u64) {
        let position = self.arena.nodes[node_id]
            .keys
            .iter()
            .position(|&x| x == key);

        // We are in the leaf node
        if self.arena.nodes[node_id].leaf {
            match position {
                Some(index) => {
                    self.arena.nodes[node_id].keys.remove(index);
                }
                None => return,
            }
        }

        // We are in the internal node
        match position {
            Some(index) => {
                // Case 1: key is in internal node
                self.delete_from_internal_node(node_id, index);
            }
            None => {
                // Case 2: key is not in this node, recurse to child
                todo!()
            }
        }
    }

    fn delete_from_internal_node(&mut self, node_id: NodeId, index: usize) {
        let t = self.t;
        let key = self.arena.nodes[node_id].keys[index];
        let left_child_id = self.arena.nodes[node_id].children_ids[index];
        let right_child_id = self.arena.nodes[node_id].children_ids[index + 1];

        if self.arena.nodes[left_child_id].n >= t {
            // Case 1a: left child has at least t keys
            let predecessor = self.find_predecessor(left_child_id);
            self.arena.nodes[node_id].keys[index] = predecessor;
            self.recursive_delete(left_child_id, predecessor);
        } else if self.arena.nodes[right_child_id].n >= t {
            // Case 1b: right child has at least t keys
            let successor = self.find_successor(left_child_id);
            self.arena.nodes[node_id].keys[index] = successor;
            self.recursive_delete(right_child_id, successor);
        } else {
            // Case 1c: both children have t - 1 keys, merge them
            self.merge_children(node_id, index);
            self.recursive_delete(left_child_id, key); // Key is now in the merged child
        }
    }

    fn find_predecessor(&self, parent_id: NodeId) -> u64 {
        // Find the maximum key in the subtree rooted at parent
        let mut node_id = parent_id;

        while !self.arena.nodes[node_id].leaf {
            // Rightmost child
            node_id = self.arena.nodes[node_id].children_ids[self.arena.nodes[node_id].n];
        }

        // Last key in leaf
        self.arena.nodes[node_id].keys[self.arena.nodes[node_id].n - 1]
    }

    fn find_successor(&self, parent_id: NodeId) -> u64 {
        // Find the minimum key in the subtree rooted at parent
        let mut node_id = parent_id;

        while !self.arena.nodes[node_id].leaf {
            // Leftmost child
            node_id = self.arena.nodes[node_id].children_ids[0];
        }

        // First key in leaf
        self.arena.nodes[node_id].keys[0]
    }

    fn merge_children(&mut self, parent_id: NodeId, index: usize) {
        let left_child_id = self.arena.nodes[parent_id].children_ids[index];
        let right_child_id = self.arena.nodes[parent_id].children_ids[index + 1];

        let left_child_n = self.arena.nodes[left_child_id].n;
        let right_child_n = self.arena.nodes[right_child_id].n;

        // Move parent key down to the left child
        self.arena.nodes[left_child_id].keys[left_child_n] = self.arena.nodes[parent_id].keys[index];

        // Move all keys from right child to left
        let right_child_keys = self.arena.nodes[right_child_id].keys.clone();
        self.arena.nodes[left_child_id]
            .keys
            .extend(right_child_keys);

        // Move all children from right child to left
        if !self.arena.nodes[left_child_id].leaf {
            let right_child_children = self.arena.nodes[right_child_id].children_ids.clone();
            self.arena.nodes[left_child_id]
                .children_ids
                .extend(right_child_children);
        }

        // Remove the key and child pointer from parent
        self.arena.nodes[parent_id].keys.remove(index);
        self.arena.nodes[parent_id].children_ids.remove(index + 1);

        // Update nodes' key numbers
        self.arena.nodes[left_child_id].n = left_child_n + right_child_n + 1;
        self.arena.nodes[parent_id].n -= 1;

        // Deallocate right child
        self.arena.deallocate_node(right_child_id, self.t);
    }
}
