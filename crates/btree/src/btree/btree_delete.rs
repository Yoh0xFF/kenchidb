use crate::btree::arena::NodeId;
use crate::btree::btree::Btree;

/// Btree delete implementation
impl Btree {
    pub fn delete(&mut self, key: u64) {
        self.recursive_delete(self.root_id, key);

        let nodes = &mut self.arena.nodes;

        // Handle root shrinking
        if nodes[self.root_id].n == 0 && !nodes[self.root_id].is_leaf {
            let old_root = self.root_id;
            self.root_id = nodes[self.root_id].children_ids[0];
            self.arena.deallocate_node(old_root, self.t);
        }
    }

    fn recursive_delete(&mut self, id: NodeId, key: u64) {
        let pos = self.arena.nodes[id].find_key_index(key);

        // We are in the leaf node
        if self.arena.nodes[id].is_leaf {
            if pos == None {
                return;
            }
            let k = pos.unwrap();
            let n = self.arena.nodes[id].n;
            self.arena.nodes[id].keys.copy_within(k + 1..n, k);
            self.arena.nodes[id].n -= 1;
            return;
        }

        // We are in the internal node
        match pos {
            Some(k) => {
                // Case 1: key is in the internal node
                self.delete_from_internal_node(id, k);
            }
            None => {
                // Case 2: key is not in this node, recurse to child
                let mut c_k = self.arena.nodes[id].find_child_index(key);
                let mut c_id = self.arena.nodes[id].children_ids[c_k];

                if self.is_node_underflow(c_id) {
                    // Child has a minimum number of keys, need to fix before deletion
                    self.fix_child(id, c_k);

                    // After fixing, the key might have moved, so re-find the child
                    c_k = self.arena.nodes[id].find_child_index(key);
                    c_id = self.arena.nodes[id].children_ids[c_k];
                }

                self.recursive_delete(c_id, key);
            }
        }
    }

    fn delete_from_internal_node(&mut self, id: NodeId, k: usize) {
        let key = self.arena.nodes[id].keys[k];
        let left_child_id = self.arena.nodes[id].children_ids[k];
        let right_child_id = self.arena.nodes[id].children_ids[k + 1];

        if !self.is_node_underflow(left_child_id) {
            // Case 1a: left child has at least t keys
            let predecessor = self.find_predecessor(left_child_id);
            self.arena.nodes[id].keys[k] = predecessor;
            self.recursive_delete(left_child_id, predecessor);
        } else if !self.is_node_underflow(right_child_id) {
            // Case 1b: right child has at least t keys
            let successor = self.find_successor(right_child_id);
            self.arena.nodes[id].keys[k] = successor;
            self.recursive_delete(right_child_id, successor);
        } else {
            // Case 1c: both children have t - 1 keys, merge them
            self.merge_children(id, k);
            self.recursive_delete(left_child_id, key); // Key is now in the merged child
        }
    }

    fn find_predecessor(&self, p_id: NodeId) -> u64 {
        // Find the maximum key in the subtree rooted at parent
        let mut node_id = p_id;

        while !self.arena.nodes[node_id].is_leaf {
            // Rightmost child
            node_id = self.arena.nodes[node_id].children_ids[self.arena.nodes[node_id].n];
        }

        // Last key in leaf
        self.arena.nodes[node_id].keys[self.arena.nodes[node_id].n - 1]
    }

    fn find_successor(&self, p_id: NodeId) -> u64 {
        // Find the minimum key in the subtree rooted at parent
        let mut node_id = p_id;

        while !self.arena.nodes[node_id].is_leaf {
            // Leftmost child
            node_id = self.arena.nodes[node_id].children_ids[0];
        }

        // First key in leaf
        self.arena.nodes[node_id].keys[0]
    }

    fn fix_child(&mut self, p_id: NodeId, k: usize) {
        let p = &self.arena.nodes[p_id];

        if k > 0 {
            let lc_id = p.children_ids[k - 1];
            if !self.is_node_underflow(lc_id) {
                // Case 2a: Left sibling has extra keys, borrow from it
                self.borrow_from_left_sibling(p_id, k);
                return;
            }
        }

        if k < p.n {
            let rc_id = p.children_ids[k + 1];
            if !self.is_node_underflow(rc_id) {
                // Case 2b: Right sibling has extra keys, borrow from it
                self.borrow_from_right_sibling(p_id, k);
                return;
            }
        }

        // Case 2c: Both siblings have minimum keys, merge with a sibling
        if k > 0 {
            self.merge_children(p_id, k - 1);
        } else {
            self.merge_children(p_id, k);
        }
    }

    fn borrow_from_left_sibling(&mut self, p_id: NodeId, k: usize) {
        let nodes = &mut self.arena.nodes;

        let rc_id = nodes[p_id].children_ids[k]; // right child
        let lc_id = nodes[p_id].children_ids[k - 1]; // left child

        let rc_n = nodes[rc_id].n;
        let lc_n = nodes[lc_id].n;

        // Move parent key down to child
        nodes[rc_id].keys.copy_within(0..rc_n, 1);
        nodes[rc_id].keys[0] = nodes[p_id].keys[k - 1];

        // Move left sibling's last key up to parent
        nodes[p_id].keys[k - 1] = nodes[lc_id].keys[lc_n - 1];

        // Move left sibling's last child to the current child (if not leaf)
        if !nodes[rc_id].is_leaf {
            nodes[rc_id].children_ids.copy_within(0..=rc_n, 1);
            nodes[rc_id].children_ids[0] = nodes[lc_id].children_ids[lc_n];
        }

        nodes[rc_id].n += 1;
        nodes[lc_id].n -= 1;
    }

    fn borrow_from_right_sibling(&mut self, p_id: NodeId, k: usize) {
        let nodes = &mut self.arena.nodes;

        let lc_id = nodes[p_id].children_ids[k]; // left child
        let rc_id = nodes[p_id].children_ids[k + 1]; // right child

        let lc_n = nodes[lc_id].n;
        let rc_n = nodes[rc_id].n;

        // Move parent key down to child
        nodes[lc_id].keys[lc_n] = nodes[p_id].keys[k];

        // Move right sibling's first key up to parent
        nodes[p_id].keys[k] = nodes[rc_id].keys[0];
        nodes[rc_id].keys.copy_within(1..rc_n, 0);

        // Move right sibling's first child to the current child (if not leaf)
        if !nodes[lc_id].is_leaf {
            nodes[lc_id].children_ids[lc_n + 1] = nodes[rc_id].children_ids[0];
            nodes[rc_id].children_ids.copy_within(1..=rc_n, 0);
        }

        nodes[lc_id].n += 1;
        nodes[rc_id].n -= 1;
    }

    fn merge_children(&mut self, p_id: NodeId, k: usize) {
        let nodes = &mut self.arena.nodes;

        let lc_id = nodes[p_id].children_ids[k]; // left child
        let rc_id = nodes[p_id].children_ids[k + 1]; // right child

        let p_n = nodes[p_id].n;
        let lc_n = nodes[lc_id].n;
        let rc_n = nodes[rc_id].n;

        // Move the parent key down to the left child
        nodes[lc_id].keys[lc_n] = nodes[p_id].keys[k];

        // Move all keys from right child to left
        for i in 0..rc_n {
            nodes[lc_id].keys[lc_n + 1 + i] = nodes[rc_id].keys[i];
        }

        // Move all children from right child to left
        if !nodes[lc_id].is_leaf {
            for i in 0..=rc_n {
                nodes[lc_id].children_ids[lc_n + 1 + i] = nodes[rc_id].children_ids[i];
            }
        }

        // Remove the key and child pointer from the parent
        for i in k..(p_n - 1) {
            nodes[p_id].keys[i] = nodes[p_id].keys[i + 1];
            nodes[p_id].children_ids[i + 1] = nodes[p_id].children_ids[i + 2];
        }

        // Update nodes' key numbers
        nodes[lc_id].n = lc_n + rc_n + 1;
        nodes[p_id].n -= 1;

        // Deallocate right child
        self.arena.deallocate_node(rc_id, self.t);
    }
}
