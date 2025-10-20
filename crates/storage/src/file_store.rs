use std::fs::File;
use std::sync::atomic::AtomicU64;

pub struct FileStore {
    pub file: File,
    pub size: AtomicU64,
    pub file_name: String,
    pub read_only: bool,
    pub read_count: AtomicU64,
    pub read_bytes: AtomicU64,
    pub write_count: AtomicU64,
    pub write_bytes: AtomicU64,
}

struct FileStoreHeader {
    /// Magic identifier/version number for the store format.
    /// Set to value 2 in the current implementation.
    /// Used to identify this as a KenchiDB MVStore file.
    /// Provides basic version identification.
    pub magic: [u8; 4],
}

impl FileStoreHeader {
    pub const MAGIC: [u8; 4] = *b"KNCH";
}