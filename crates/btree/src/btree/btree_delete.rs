use crate::btree::arena::NodeId;
use crate::btree::btree::Btree;

/// Btree delete implementation
impl Btree {
    pub fn delete(&mut self, key: u64) {
        self.recursive_delete(self.root_id, key);
    }

    fn recursive_delete(&mut self, node_id: NodeId, key: u64) {
        let position = self.arena.nodes[node_id]
            .keys
            .iter()
            .position(|&x| x == key);

        // We are in the leaf node
        if self.arena.nodes[node_id].is_leaf {
            match position {
                Some(index) => {
                    self.arena.nodes[node_id].keys.remove(index);
                    self.arena.nodes[node_id].n -= 1; 
                }
                None => return,
            }
            return;
        }

        // We are in the internal node
        match position {
            Some(index) => {
                // Case 1: key is in the internal node
                self.delete_from_internal_node(node_id, index);
            }
            None => {
                // Case 2: key is not in this node, recurse to child
                let mut child_index = self.find_child_index(node_id, key);
                let mut child_id = self.arena.nodes[node_id].children_ids[child_index];
                let child = &self.arena.nodes[child_id];

                if child.n < self.t {
                    // Child has a minimum number of keys, need to fix before deletion
                    self.fix_child(node_id, child_index);

                    // After fixing, the key might have moved, so re-find the child
                    child_index = self.find_child_index(node_id, key);
                    child_id = self.arena.nodes[node_id].children_ids[child_index];
                }

                self.recursive_delete(child_id, key);
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
            let successor = self.find_successor(right_child_id);
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

        while !self.arena.nodes[node_id].is_leaf {
            // Rightmost child
            node_id = self.arena.nodes[node_id].children_ids[self.arena.nodes[node_id].n];
        }

        // Last key in leaf
        self.arena.nodes[node_id].keys[self.arena.nodes[node_id].n - 1]
    }

    fn find_successor(&self, parent_id: NodeId) -> u64 {
        // Find the minimum key in the subtree rooted at parent
        let mut node_id = parent_id;

        while !self.arena.nodes[node_id].is_leaf {
            // Leftmost child
            node_id = self.arena.nodes[node_id].children_ids[0];
        }

        // First key in leaf
        self.arena.nodes[node_id].keys[0]
    }

    fn fix_child(&mut self, parent_id: NodeId, child_index: usize) {
        let parent = &self.arena.nodes[parent_id];

        if child_index > 0 && self.arena.nodes[parent.children_ids[child_index - 1]].n >= self.t {
            // Case 2a: Left sibling has extra keys, borrow from it
            self.borrow_from_left_sibling(parent_id, child_index);
        } else if child_index < parent.n && self.arena.nodes[parent.children_ids[child_index + 1]].n >= self.t {
            // Case 2b: Right sibling has extra keys, borrow from it
            self.borrow_from_right_sibling(parent_id, child_index);
        } else {
            // Case 2c: Both siblings have minimum keys, merge with a sibling
            if child_index > 0 {
                self.merge_children(parent_id, child_index - 1);
            } else {
                self.merge_children(parent_id, child_index);
            }
        }
    }

    fn borrow_from_left_sibling(&mut self, parent_id: NodeId, child_index: usize) {
        let child_id = self.arena.nodes[parent_id].children_ids[child_index];
        let left_sibling_id = self.arena.nodes[parent_id].children_ids[child_index - 1];

        // Move parent key down to child
        let parent_key = self.arena.nodes[parent_id].keys[child_index - 1];
        self.arena.nodes[child_id].keys.insert(0, parent_key);

        // Move left sibling's last key up to parent
        let left_sibling_key = self.arena.nodes[left_sibling_id].keys.pop();
        self.arena.nodes[parent_id].keys[child_index - 1] = left_sibling_key.unwrap();

        // Move left sibling's last child to the current child (if not leaf)
        if !self.arena.nodes[child_id].is_leaf {
            let left_sibling_child_id = self.arena.nodes[left_sibling_id].children_ids.pop();
            self.arena.nodes[child_id].children_ids.insert(0, left_sibling_child_id.unwrap());
        }

        self.arena.nodes[child_id].n += 1;
        self.arena.nodes[left_sibling_id].n -= 1;
    }
    
    fn borrow_from_right_sibling(&mut self, parent_id: NodeId, child_index: usize) {
        let child_id = self.arena.nodes[parent_id].children_ids[child_index];
        let right_sibling_id = self.arena.nodes[parent_id].children_ids[child_index + 1];

        // Move parent key down to child
        let parent_key = self.arena.nodes[parent_id].keys[child_index];
        self.arena.nodes[child_id].keys.push(parent_key);

        // Move right sibling's first key up to parent
        let right_sibling_key = self.arena.nodes[right_sibling_id].keys.remove(0);
        self.arena.nodes[parent_id].keys[child_index] = right_sibling_key;

        // Move right sibling's first child to the current child (if not leaf)
        if !self.arena.nodes[child_id].is_leaf {
            let right_sibling_child_id = self.arena.nodes[right_sibling_id].children_ids.remove(0);
            self.arena.nodes[child_id].children_ids.push(right_sibling_child_id);
        }

        self.arena.nodes[child_id].n += 1;
        self.arena.nodes[right_sibling_id].n -= 1;
    }

    fn merge_children(&mut self, parent_id: NodeId, index: usize) {
        let left_child_id = self.arena.nodes[parent_id].children_ids[index];
        let right_child_id = self.arena.nodes[parent_id].children_ids[index + 1];

        let left_child_n = self.arena.nodes[left_child_id].n;
        let right_child_n = self.arena.nodes[right_child_id].n;

        // Move the parent key down to the left child
        self.arena.nodes[left_child_id].keys[left_child_n] =
            self.arena.nodes[parent_id].keys[index];

        // Move all keys from right child to left
        let right_child_keys = self.arena.nodes[right_child_id].keys.clone();
        self.arena.nodes[left_child_id]
            .keys
            .extend(right_child_keys);

        // Move all children from right child to left
        if !self.arena.nodes[left_child_id].is_leaf {
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
