use crate::btree::arena::NodeId;
use crate::btree::btree::Btree;

/// BTree insert implementation
impl Btree {
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

    fn recursive_insert(&mut self, node_id: NodeId, key: u64) {
        if self.arena.nodes[node_id].leaf {
            self.insert_into_leaf_node(node_id, key);
        } else {
            self.insert_into_internal_node(node_id, key);
        }
    }

    fn insert_into_leaf_node(&mut self, node_id: NodeId, key: u64) {
        let n = self.arena.nodes[node_id].n;

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
    }

    fn insert_into_internal_node(&mut self, node_id: NodeId, key: u64) {
        let t = self.t;
        let n = self.arena.nodes[node_id].n;

        // find the child where the key belongs
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
                // does the key go into child[i] or child[i + 1]?
                pos += 1;
            }
        }

        let child_id = self.arena.nodes[node_id].children_ids[pos];
        self.recursive_insert(child_id, key);
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
}
