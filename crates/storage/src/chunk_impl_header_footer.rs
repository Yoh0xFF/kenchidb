use crate::chunk::{ChunkFooter, ChunkHeader};
use crate::error::StorageError;

impl ChunkHeader {
    pub const MAGIC: &'static str = "KNCH";
    pub const SIZE: usize = 96;

    pub fn serialize_header(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        let mut offset = 0;

        // Magic number (4 bytes)
        bytes[offset..offset + 4].copy_from_slice(Self::MAGIC.as_bytes());
        offset += 4;

        // Format version (2 bytes)
        bytes[offset..offset + 2].copy_from_slice(&self.format_version.to_le_bytes());
        offset += 2;
        
        bytes
    }

    pub fn deserialize_header(bytes: &[u8]) -> Result<Self, StorageError> {
        if bytes.len() != Self::SIZE {
            return Err(StorageError::InvalidChunkHeader("Invalid chunk header size".to_string()));    
        }
        
        let mut offset = 0;

        let magic = [bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]];
        offset += 4;

        let format_version = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        Ok(Self {
            magic,
            format_version,
        })
    }
}

impl ChunkFooter {
    pub fn serialize_footer(&self) {
        todo!()
    }

    pub fn deserialize_footer(&self) {
        todo!()
    }

    pub fn verify_footer(&self) {
        todo!()
    }
}
