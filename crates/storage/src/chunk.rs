use bitvec::prelude::BitVec;
use bytes::Bytes;

/// Chunk header
/// 64 bytes
/// !IMPORTANT: Do not change field order
#[derive(Debug, Copy, Clone)]
pub struct ChunkHeader {
    /// 4 byte fields
    /// 8 fields * 4 bytes = 32 bytes
    pub magic: [u8; 4],
    pub id: u32,
    pub length: u32,
    pub page_count: u32,
    pub table_of_content_position: u32,
    pub max_length: u32,
    pub pin_count: u32,
    pub map_id: u32,
    /// 8 byte fields
    /// 4 fields * 8 bytes = 32 bytes
    pub version: u64,
    pub time: u64,
    pub layout_root_position: u64,
    pub next: u64,

}

/// Chunk footer
/// 20 bytes
/// !IMPORTANT: Do not change field order
#[derive(Debug, Copy, Clone)]
pub struct ChunkFooter {
    /// 4 byte fields
    /// 3 fields * 4 bytes = 12 bytes
    pub id: u32,
    pub length: u32,
    pub checksum: u32,
    /// 8 byte fields
    /// 1 field * 8 bytes = 8 bytes
    pub version: u64,
}

/// Chunks are large storage units that:
/// - Serve as containers for multiple pages
/// - Have a minimum size of 4096 bytes (one block) and grow in fixed block increments
/// - Are the unit of allocation and persistence in the file system
/// - Can contain up to 67 million pages and be up to 2GB in size
/// - Have their own headers and footers for metadata
#[derive(Debug, Clone)]
pub struct Chunk {
    /// ******************************
    /// * Core Identity and Location *
    /// ******************************
    /// Unique chunk identifier
    pub id: u32,
    /// Version stored in this chunk
    pub version: u64,
    /// Creation time (milliseconds since store creation)
    pub time: u64,
    /// length in number of blocks (each block is 4096 bytes)
    pub length: u32,
    /// Chunk offset in the file (can change during compaction)
    pub block: u64,

    /// *******************
    /// * Page management *
    /// *******************
    /// Total number of pages stored in the chunk
    pub page_count: u32,
    /// Number of pages still alive (not deleted) in the latest version
    pub page_count_live: u32,
    /// Byte offset for the table of contents that maps page numbers to positions
    pub table_of_content_position: u32,
    /// Bit set tracking deleted pages (set bit = deleted page)
    pub occupancy: BitVec,

    /// ****************************
    /// * Size and Memory Tracking *
    /// ****************************
    /// Sum of max length of all pages in the chunk
    pub max_length: u32,
    /// Sum of max length of all live pages in the chunk (not deleted)
    pub max_length_live: u32,

    /// ************************************
    /// * Garbage Collection and Lifecycle *
    /// ************************************
    /// GC priority (0 = needs collection, higher = lower priority)
    pub collect_priority: u16,
    /// Time when chunk become unused (in milliseconds since store creation)
    pub unused: u64,
    /// Store version when chunk became unused
    pub unused_at_version: u64,
    /// number of the live-pinned pages (cannot be evacuated/moved)
    pub pin_count: u32,

    /// ***************************
    /// * Versioning and Metadata *
    /// ***************************
    /// Position of the root of the layout map.
    /// Serves as a pointer to the root page of the layout map for that specific chunk.
    /// It stores:
    /// - Position reference: It stores the position (address)
    /// of the root page of the layout map within the chunk
    /// - Layout map root: The layout map is a special map that
    /// contains metadata about all other maps stored in the database
    pub layout_root_position: u64,
    /// The last used map id
    pub map_id: u32,
    /// Predicted position of the next chunk
    pub next: u64,

    /// *********************
    /// * Buffer Management *
    /// *********************
    /// ByteBuffer holding serialized content before saving to filestore (allows early page GC)
    pub buffer: Bytes,
}

impl Chunk {
    /// Maximum chunk id (2^26 - 1, about 67 million chunks)
    pub const MAX_ID: u32 = (1 << 26) - 1;
    /// Maximum size of the chunk header in bytes
    pub const MAX_HEADER_LENGTH: u16 = 1024;
    /// Maximum size of the chunk footer in bytes
    pub const MAX_FOOTER_LENGTH: u8 = 128;
}