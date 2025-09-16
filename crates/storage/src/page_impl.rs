use crate::page::{Page, PageCore, PageKind, PagePosition, PageReference};
use std::sync::atomic::AtomicU64;

impl<Key, Value> Page<Key, Value> {
    pub fn new_leaf(tree_id: u32, keys: Vec<Key>, values: Vec<Value>) -> Self {
        Page {
            core: PageCore::new(tree_id, keys),
            kind: PageKind::Leaf { values },
        }
    }

    pub fn new_internal(
        tree_id: u32,
        keys: Vec<Key>,
        children: Vec<PageReference<Key, Value>>,
        total_count: u64,
    ) -> Self {
        Page {
            core: PageCore::new(tree_id, keys),
            kind: PageKind::Internal {
                children,
                total_count,
            },
        }
    }

    pub fn get_key(&self, index: usize) -> &Key {
        &self.core.keys[index]
    }
    
    pub fn get_key_count(&self) -> usize {
        self.core.keys.len()
    }
    
    pub fn is_leaf(&self) -> bool {
        match &self.kind {
            PageKind::Internal { .. } => false,
            PageKind::Leaf { .. } => true,
        }
    }
    
    pub fn get_position(&self) -> PagePosition {
        self.core.position.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn calculate_memory(&self) -> u32 {
        todo!("We need to implement mv map first")
    }

    pub fn add_memory(&mut self, memory: u32)  {
        assert!(self.core.memory <= u32::MAX - memory);
        self.core.memory += memory;
    }
    
    pub fn get_memory(&self) -> u32 {
        self.core.memory
    }
}

impl<Key> PageCore<Key> {
    pub fn new(tree_id: u32, keys: Vec<Key>) -> Self {
        PageCore {
            tree_id,
            position: AtomicU64::new(0),
            page_number: 0,
            cached_compare: 0,
            memory: 0,
            disk_space_used: 0,
            keys,
        }
    }
}