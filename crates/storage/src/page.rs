use std::sync::atomic::AtomicU64;

// Basic position and identifier types
pub type PagePosition = u64; // Encoded page position
pub type ChunkId = u32;
pub type PageNumber = u32;

#[derive(Debug)]
pub struct PageCore<Key> {
    /// ************************
    /// * Core identity fields *
    /// ************************
    /// Reference to the BTree that owns this page,
    /// This provides access to the ley/value type information,
    /// serialization methods, and store configuration
    pub tree_id: u32,
    /// Encoded position of the page in the chunk.
    /// 0 = the page has not been saved yet
    /// 1 = the page marked as removed but not saved
    /// otherwise encodes: chunk id, offset within chunk, page length, and type
    /// Uses atomic operations to handle concurrent access during save/remove operations
    pub position: AtomicU64,
    /// Sequential 0-based page number within the chunk.
    /// Used for addressing pages withing a chunk's table of content.
    pub page_number: PageNumber,

    /// **********************************
    /// * Caching and performance fields *
    /// **********************************
    /// Caches the last binary search result to optimize repeated searches on the same page.
    /// Since b-tree operations often exhibit locality, this can significantly speed up
    /// further searches.
    pub cached_compare: u32,
    /// Estimated RAM usage in bytes for persistent pages,
    /// or IN_MEMORY constant for in-memory pages.
    /// Critical for the memory management and cache eviction policies.
    pub memory: u32,
    /// Actual butes used on disk by this page only (not including child pages),
    /// used for storage statistics and compactions decisions.
    pub disk_space_used: u32,

    /// ***********************
    /// * Data storage fields *
    /// ***********************
    /// Array holding the actual ket objects.
    /// For internal nodes, the keys[i] is larger than the largest key in the child[i].
    pub keys: Vec<Key>,
}

#[derive(Debug)]
pub enum PageKind<Value> {
    Internal {
        /// Array holding the actual value objects.
        values: Vec<Value>,
    },
    Leaf {
        /// Array of child pages.
        children: Vec<PageNumber>,
        /// total number of key-value pairs in ths subtree.
        total_count: u64,
    },
}

#[derive(Debug)]
pub struct Page<Key, Value> {
    /// Page core fields
    pub core: PageCore<Key>,
    /// Page kind-specific fields
    pub kind: PageKind<Value>,
}