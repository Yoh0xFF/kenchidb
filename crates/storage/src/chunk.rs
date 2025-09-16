
/// Chunks are large storage units that:
/// - Serve as containers for multiple pages
/// - Have a minimum size of 4096 bytes (one block) and grow in fixed block increments
/// - Are the unit of allocation and persistence in the file system
/// - Can contain up to 67 million pages and be up to 2GB in size
/// - Have their own headers and footers for metadata
pub struct Chunk {}