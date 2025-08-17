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
        self.arena.nodes[left_child_id].keys[left_child_n] =
            self.arena.nodes[parent_id].keys[index];

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
