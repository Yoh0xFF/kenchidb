use std::fs::File;

pub struct FileStore {
    pub file: File,
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