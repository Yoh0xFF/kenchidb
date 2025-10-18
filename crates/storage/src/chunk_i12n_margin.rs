use crate::chunk::{Chunk, ChunkFooter, ChunkHeader};
use crate::data_util::get_fletcher32;
use crate::error::StorageError;

impl Chunk {
    pub fn serialize_header(&self) -> [u8; ChunkHeader::SIZE] {
        let header = ChunkHeader {
            magic: ChunkHeader::MAGIC,
            id: self.id,
            length: self.length,
            version: self.version,
            time: self.time,
            max_length: self.max_length,
            page_count: self.page_count,
            pin_count: self.pin_count,
            table_of_content_position: self.table_of_content_position,
            layout_root_position: self.layout_root_position,
            map_id: self.map_id,
            next: self.next,
        };

        header.serialize_header()
    }

    pub fn deserialize_header(bytes: &[u8]) -> Result<ChunkHeader, StorageError> {
        ChunkHeader::deserialize_header(bytes)
    }

    pub fn serialize_footer(&self) -> [u8; ChunkFooter::SIZE] {
        let footer = ChunkFooter {
            id: self.id,
            length: self.length,
            version: self.version,
            checksum: 0,
        };

        footer.serialize_footer()
    }

    pub fn deserialize_footer(bytes: &[u8]) -> Result<ChunkFooter, StorageError> {
        ChunkFooter::deserialize_footer(bytes)
    }

    pub fn verify_footer(bytes: &[u8]) -> bool {
        ChunkFooter::verify_footer(bytes)
    }
}

impl ChunkHeader {
    pub fn serialize_header(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];

        bytes[Self::FIELD_MAGIC_OFFSET..Self::FIELD_MAGIC_OFFSET + 4]
            .copy_from_slice(Self::MAGIC.as_slice());
        bytes[Self::FIELD_ID_OFFSET..Self::FIELD_ID_OFFSET + 4]
            .copy_from_slice(&self.id.to_le_bytes());
        bytes[Self::FIELD_LENGTH_OFFSET..Self::FIELD_LENGTH_OFFSET + 4]
            .copy_from_slice(&self.length.to_le_bytes());
        bytes[Self::FIELD_VERSION_OFFSET..Self::FIELD_VERSION_OFFSET + 8]
            .copy_from_slice(&self.version.to_le_bytes());
        bytes[Self::FIELD_TIME_OFFSET..Self::FIELD_TIME_OFFSET + 8]
            .copy_from_slice(&self.time.to_le_bytes());
        bytes[Self::FIELD_MAX_LENGTH_OFFSET..Self::FIELD_MAX_LENGTH_OFFSET + 4]
            .copy_from_slice(&self.max_length.to_le_bytes());
        bytes[Self::FIELD_PAGE_COUNT_OFFSET..Self::FIELD_PAGE_COUNT_OFFSET + 4]
            .copy_from_slice(&self.page_count.to_le_bytes());
        bytes[Self::FIELD_PIN_COUNT_OFFSET..Self::FIELD_PIN_COUNT_OFFSET + 4]
            .copy_from_slice(&self.pin_count.to_le_bytes());
        bytes[Self::FIELD_TABLE_OF_CONTENT_POSITION_OFFSET
            ..Self::FIELD_TABLE_OF_CONTENT_POSITION_OFFSET + 4]
            .copy_from_slice(&self.table_of_content_position.to_le_bytes());
        bytes[Self::FIELD_LAYOUT_ROOT_POSITION_OFFSET..Self::FIELD_LAYOUT_ROOT_POSITION_OFFSET + 8]
            .copy_from_slice(&self.layout_root_position.to_le_bytes());
        bytes[Self::FIELD_MAP_ID_OFFSET..Self::FIELD_MAP_ID_OFFSET + 4]
            .copy_from_slice(&self.map_id.to_le_bytes());
        bytes[Self::FIELD_NEXT_OFFSET..Self::FIELD_NEXT_OFFSET + 8]
            .copy_from_slice(&self.next.to_le_bytes());

        bytes
    }

    pub fn deserialize_header(bytes: &[u8]) -> Result<Self, StorageError> {
        if bytes.len() != Self::SIZE {
            return Err(StorageError::InvalidChunkHeader(
                "Invalid chunk header size".to_string(),
            ));
        }

        let magic = bytes[Self::FIELD_MAGIC_OFFSET..Self::FIELD_MAGIC_OFFSET + 4]
            .try_into()
            .unwrap();
        let id = read_u32(bytes, Self::FIELD_ID_OFFSET);
        let length = read_u32(bytes, Self::FIELD_LENGTH_OFFSET);
        let version = read_u64(bytes, Self::FIELD_VERSION_OFFSET);
        let time = read_u64(bytes, Self::FIELD_TIME_OFFSET);
        let max_length = read_u32(bytes, Self::FIELD_MAX_LENGTH_OFFSET);
        let page_count = read_u32(bytes, Self::FIELD_PAGE_COUNT_OFFSET);
        let pin_count = read_u32(bytes, Self::FIELD_PIN_COUNT_OFFSET);
        let table_of_content_position =
            read_u32(bytes, Self::FIELD_TABLE_OF_CONTENT_POSITION_OFFSET);
        let layout_root_position = read_u64(bytes, Self::FIELD_LAYOUT_ROOT_POSITION_OFFSET);
        let map_id = read_u32(bytes, Self::FIELD_MAP_ID_OFFSET);
        let next = read_u64(bytes, Self::FIELD_NEXT_OFFSET);

        Ok(Self {
            magic,
            id,
            length,
            version,
            time,
            max_length,
            page_count,
            pin_count,
            table_of_content_position,
            layout_root_position,
            map_id,
            next,
        })
    }
}

impl ChunkFooter {
    pub fn serialize_footer(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];

        bytes[Self::FIELD_ID_OFFSET..Self::FIELD_ID_OFFSET + 4]
            .copy_from_slice(&self.id.to_le_bytes());
        bytes[Self::FIELD_LENGTH_OFFSET..Self::FIELD_LENGTH_OFFSET + 4]
            .copy_from_slice(&self.length.to_le_bytes());
        bytes[Self::FIELD_VERSION_OFFSET..Self::FIELD_VERSION_OFFSET + 8]
            .copy_from_slice(&self.version.to_le_bytes());
        let checksum = get_fletcher32(&bytes, 0, Self::FIELD_CHECKSUM_OFFSET);
        bytes[Self::FIELD_CHECKSUM_OFFSET..Self::FIELD_CHECKSUM_OFFSET + 4]
            .copy_from_slice(&checksum.to_le_bytes());

        bytes
    }

    pub fn deserialize_footer(bytes: &[u8]) -> Result<Self, StorageError> {
        if bytes.len() != Self::SIZE {
            return Err(StorageError::InvalidChunkHeader(
                "Invalid chunk footer size".to_string(),
            ));
        }

        let id = read_u32(bytes, Self::FIELD_ID_OFFSET);
        let length = read_u32(bytes, Self::FIELD_LENGTH_OFFSET);
        let version = read_u64(bytes, Self::FIELD_VERSION_OFFSET);
        let checksum = read_u32(bytes, Self::FIELD_CHECKSUM_OFFSET);

        Ok(Self {
            id,
            length,
            version,
            checksum,
        })
    }

    pub fn verify_footer(bytes: &[u8]) -> bool {
        if bytes.len() != Self::SIZE {
            return false;
        }

        let stored_checksum = u32::from_le_bytes(
            bytes[Self::FIELD_CHECKSUM_OFFSET..Self::FIELD_CHECKSUM_OFFSET + 4]
                .try_into()
                .unwrap(),
        );

        let calculated_checksum = get_fletcher32(bytes, 0, Self::FIELD_CHECKSUM_OFFSET);

        stored_checksum == calculated_checksum
    }
}

// Helper functions for reading with automatic offset advancement
fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    let value = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    value
}

fn read_i32(bytes: &[u8], offset: usize) -> i32 {
    let value = i32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    value
}

fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    let value = u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap());
    value
}
