use crate::chunk::{ChunkFooter, ChunkHeader};
use crate::error::StorageError;

impl ChunkHeader {
    pub const MAGIC: &'static str = "KNCH";
    pub const SIZE: usize = 96;

    pub fn serialize_header(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        let mut offset = 0;

        // 4 byte fields
        // magic number
        bytes[offset..offset + 4].copy_from_slice(Self::MAGIC.as_bytes());
        offset += 4;
        // id
        bytes[offset..offset + 4].copy_from_slice(&self.id.to_le_bytes());
        offset += 4;
        // length
        bytes[offset..offset + 4].copy_from_slice(&self.length.to_le_bytes());
        offset += 4;
        // page_count
        bytes[offset..offset + 4].copy_from_slice(&self.page_count.to_le_bytes());
        offset += 4;
        // table_of_content_position
        bytes[offset..offset + 4].copy_from_slice(&self.table_of_content_position.to_le_bytes());
        offset += 4;
        // max_length
        bytes[offset..offset + 4].copy_from_slice(&self.max_length.to_le_bytes());
        offset += 4;
        // pin_count
        bytes[offset..offset + 4].copy_from_slice(&self.pin_count.to_le_bytes());
        offset += 4;
        // map_id
        bytes[offset..offset + 4].copy_from_slice(&self.map_id.to_le_bytes());
        offset += 4;

        // 8 byte fields
        // version
        bytes[offset..offset + 8].copy_from_slice(&self.version.to_le_bytes());
        offset += 8;
        // time
        bytes[offset..offset + 8].copy_from_slice(&self.time.to_le_bytes());
        offset += 8;
        // layout_root_position
        bytes[offset..offset + 8].copy_from_slice(&self.layout_root_position.to_le_bytes());
        offset += 8;
        // next
        bytes[offset..offset + 8].copy_from_slice(&self.next.to_le_bytes());
        offset += 8;

        bytes[offset..offset + 32].copy_from_slice(&[0u8; 32]);

        bytes
    }

    pub fn deserialize_header(bytes: &[u8]) -> Result<Self, StorageError> {
        if bytes.len() != Self::SIZE {
            return Err(StorageError::InvalidChunkHeader(
                "Invalid chunk header size".to_string(),
            ));
        }

        let mut offset = 0;

        // Helper macro to read and advance offset
        macro_rules! read_u32 {
            () => {{
                let value = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
                offset += 4;
                value
            }};
        }
        macro_rules! read_u64 {
            () => {{
                let value = u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap());
                offset += 8;
                value
            }};
        }

        // 4 byte fields
        let magic = bytes[offset..offset + 4].try_into().unwrap();
        offset += 4;
        let id = read_u32!();
        let length = read_u32!();
        let page_count = read_u32!();
        let table_of_content_position = read_u32!();
        let max_length = read_u32!();
        let pin_count = read_u32!();
        let map_id = read_u32!();

        // 8 byte fields
        let version = read_u64!();
        let time = read_u64!();
        let layout_root_position = read_u64!();
        let next = read_u64!();

        Ok(Self {
            magic,
            id,
            length,
            page_count,
            table_of_content_position,
            max_length,
            pin_count,
            map_id,
            version,
            time,
            layout_root_position,
            next,
        })
    }
}

impl ChunkFooter {
    pub const MAGIC: &'static str = "KNCH";
    pub const SIZE: usize = 96;

    pub fn serialize_footer(&self) -> Result<Self, StorageError> {
        todo!()
    }

    pub fn deserialize_footer(bytes: &[u8]) -> Self {
        todo!()
    }

    pub fn verify_footer(&self) -> bool {
        todo!()
    }
}
