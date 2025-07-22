use crate::common::DatabaseError;


/// Page size - 4kb is a common choice for page size in many systems.
/// It aligns well with OS page size.
pub const PAGE_SIZE: usize = 4096; // 4 KiB

/// Page header size - contains metadata about the page.
pub const PAGE_HEADER_SIZE: usize = 24;

/// Maximum usable space per page (excluding header).
pub const MAX_PAGE_DATA_SIZE: usize = PAGE_SIZE - PAGE_HEADER_SIZE;

/// Page types for different kinds of data.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PageType {
    /// Stores actual document data.
    DataPage = 1,
    /// Stores metadata about collections and schemas.
    MetaPage = 2,
    /// Free page that can be allocated.
    FreePage = 3,
    /// Header page - first page in file with database metadata.
    HeaderPage = 4,
}

impl PageType {
    fn from_u8(value: u8) -> Result<Self, DatabaseError> {
        match value {
            1 => Ok(PageType::DataPage),
            2 => Ok(PageType::MetaPage),
            3 => Ok(PageType::FreePage),
            4 => Ok(PageType::HeaderPage),
            _ => Err(DatabaseError::InvalidData(format!(
                "Invalid page type: {}",
                value
            ))),
        }
    }
}

/// Page header structure (24 bytes total).
#[derive(Debug, Clone)]
pub struct PageHeader {
    /// Magic number to identify valid pages (4 bytes).
    pub magic: u32,
    /// Type of page (1 byte).
    pub page_type: PageType,
    /// Reserved for alignment (3 bytes).
    pub _reserved: [u8; 3],
    /// Number of records/slots in this page (2 bytes).
    pub record_count: u16,
    /// Offset to start of free space (2 bytes).
    pub free_space_start: u16,
    /// Amount of free space available (2 bytes).
    pub free_space_size: u16,
    /// Page checksum for corruption detection (4 bytes).
    pub checksum: u32,
    /// Collection ID this page belongs to (4 bytes).
    pub collection_id: u32,
}

impl PageHeader {
    const MAGIC_NUMBER: u32 = 0x4B454E43; // "KENC" in ASCII

    pub fn new(page_type: PageType, collection_id: u32) -> Self {
        Self {
            magic: Self::MAGIC_NUMBER,
            page_type,
            _reserved: [0; 3],
            record_count: 0,
            free_space_start: PAGE_HEADER_SIZE as u16,
            free_space_size: MAX_PAGE_DATA_SIZE as u16,
            checksum: 0, // Will be calculated when serializing
            collection_id,
        }
    }

    pub fn serialize(&self) -> [u8; PAGE_HEADER_SIZE] {
        let mut bytes = [0u8; PAGE_HEADER_SIZE];
        let mut offset = 0;

        // Magic number (4 bytes)
        bytes[offset..offset + 4].copy_from_slice(&self.magic.to_le_bytes());
        offset += 4;

        // Page type (1 byte)
        bytes[offset] = self.page_type as u8;
        offset += 1;

        // Reserved (3 bytes)
        bytes[offset..offset + 3].copy_from_slice(&self._reserved);
        offset += 3;

        // Record count (2 bytes)
        bytes[offset..offset + 2].copy_from_slice(&self.record_count.to_le_bytes());
        offset += 2;

        // Free space start (2 bytes)
        bytes[offset..offset + 2].copy_from_slice(&self.free_space_start.to_le_bytes());
        offset += 2;

        // Free space size (2 bytes)
        bytes[offset..offset + 2].copy_from_slice(&self.free_space_size.to_le_bytes());
        offset += 2;

        // Checksum (4 bytes)
        bytes[offset..offset + 4].copy_from_slice(&self.checksum.to_le_bytes());
        offset += 4;

        // Collection ID (4 bytes)
        bytes[offset..offset + 4].copy_from_slice(&self.collection_id.to_le_bytes());

        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, DatabaseError> {
        if bytes.len() < PAGE_HEADER_SIZE {
            return Err(DatabaseError::InvalidData(
                "Invalid page header size".to_string(),
            ));
        }

        let mut offset = 0;

        // Magic number (4 bytes)
        let magic = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        if magic != Self::MAGIC_NUMBER {
            return Err(DatabaseError::InvalidData(
                "Invalid page magic number".to_string(),
            ));
        }

        // Page type (1 byte)
        let page_type = PageType::from_u8(bytes[offset])?;
        offset += 1;

        // Reserved (3 bytes)
        let reserved = [bytes[offset], bytes[offset + 1], bytes[offset + 2]];
        offset += 3;

        // Record count (2 bytes)
        let record_count = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        // Free space start (2 bytes)
        let free_space_start = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        // Free space size (2 bytes)
        let free_space_size = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        // Checksum (4 bytes)
        let checksum = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        // Collection ID (4 bytes)
        let collection_id = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);

        Ok(Self {
            magic,
            page_type,
            _reserved: reserved,
            record_count,
            free_space_start,
            free_space_size,
            checksum,
            collection_id,
        })
    }
}

/// Slot directory entry - points to records withing a page
#[derive(Debug, Clone, Copy)]
pub struct SlotEntry {
    /// Offset from start of page where record begins
    pub offset: u16,
    /// Length of the record in bytes
    pub length: u16,
}

impl SlotEntry {
    pub fn new(offset: u16, length: u16) -> Self {
        Self { offset, length }
    }

    pub fn serialize(&self) -> [u8; 4] {
        let mut bytes = [0u8; 4];
        bytes[0..2].copy_from_slice(&self.offset.to_le_bytes());
        bytes[2..4].copy_from_slice(&self.length.to_le_bytes());
        bytes
    }

    pub fn deserialize(bytes: &[u8]) -> Result<SlotEntry, DatabaseError> {
        if bytes.len() < 4 {
            return Err(DatabaseError::InvalidData(
                "Invalid slot entry size".to_string(),
            ));
        }

        let offset = u16::from_le_bytes([bytes[0], bytes[1]]);
        let length = u16::from_le_bytes([bytes[1], bytes[2]]);

        return Ok(SlotEntry { offset, length });
    }
}

/// A database page containing header, slot directory and data.
#[derive(Debug, Clone)]
pub struct Page {
    pub header: PageHeader,
    pub slots: Vec<SlotEntry>,
    pub data: Vec<u8>,
}

impl Page {
    pub fn new(page_type: PageType, collection_id: u32) -> Self {
        Self {
            header: PageHeader::new(page_type, collection_id),
            slots: Vec::new(),
            data: vec![0u8; MAX_PAGE_DATA_SIZE],
        }
    }

    /// Insert a record into the page, returns slot index if successful
    pub fn insert_record(&mut self, record_data: &[u8]) -> Result<u16, DatabaseError> {
        let record_size = record_data.len();
        let slot_size = 4; // Each slot entry is 4 bytes

        // Check if we have enough space (need spaced for data + slot entry)
        if (self.header.free_space_size as usize) < record_size + slot_size {
            return Err(DatabaseError::InvalidData(
                "Not enough space in page".to_string(),
            ));
        }

        // Calculate where to place the new record (grows backwards from end)
        let data_end = PAGE_SIZE - (self.slots.len() * slot_size);
        let new_record_offset = data_end - record_size;

        // Copy record data to page
        let data_start_in_page = new_record_offset - PAGE_HEADER_SIZE;
        self.data[data_start_in_page..data_start_in_page + record_size]
            .copy_from_slice(record_data);

        // Add slot entry
        let slot_index = self.slots.len() as u16;
        self.slots.push(SlotEntry {
            offset: new_record_offset as u16,
            length: record_size as u16,
        });

        // Update header
        self.header.record_count += 1;
        self.header.free_space_size = self
            .header
            .free_space_size
            .saturating_sub((record_size + slot_size) as u16);

        Ok(slot_index)
    }

    /// Get a record by slot index
    pub fn get_record(&self, slot_index: u16) -> Result<&[u8], DatabaseError> {
        if (slot_index as usize) >= self.slots.len() {
            return Err(DatabaseError::InvalidData("Invalid slot index".to_string()));
        }

        let slot = self.slots[slot_index as usize];
        let data_start = slot.offset as usize - PAGE_HEADER_SIZE;
        let data_end = data_start + slot.length as usize;

        if data_end > self.data.len() {
            return Err(DatabaseError::InvalidData(
                "Record extends beyond page".to_string(),
            ));
        }

        Ok(&self.data[data_start..data_end])
    }

    /// Calculate and update checksum for the page
    pub fn update_checksum(&mut self) {
        // Simple checksum - sum of all data bytes
        let mut checksum = 0u32;

        // Include slot data in checksum
        for slot in &self.slots {
            let slot_bytes = slot.serialize();
            for byte in slot_bytes {
                checksum = checksum.wrapping_add(byte as u32);
            }
        }

        // Include actual record data
        for byte in &self.data {
            checksum = checksum.wrapping_add(*byte as u32);
        }

        self.header.checksum = checksum;
    }

    /// Serialize entire page to bytes
    pub fn serialize(&mut self) -> [u8; PAGE_SIZE] {
        let mut page_bytes = [0u8; PAGE_SIZE];

        // Update checksum before serializing
        self.update_checksum();

        // Serialize header
        let header_bytes = self.header.serialize();
        page_bytes[0..PAGE_HEADER_SIZE].copy_from_slice(&header_bytes);

        // Serialize slot directory (grows upward from header)
        let mut slot_offset = PAGE_HEADER_SIZE;
        for slot in &self.slots {
            let slot_bytes = slot.serialize();
            page_bytes[slot_offset..slot_offset + 4].copy_from_slice(&slot_bytes);
            slot_offset += 4;
        }

        // Copy data section
        page_bytes[PAGE_HEADER_SIZE..PAGE_HEADER_SIZE + self.data.len()]
            .copy_from_slice(&self.data);

        page_bytes
    }

    /// Deserialize page from bytes
    pub fn deserialize(bytes: &[u8]) -> Result<Self, DatabaseError> {
        if bytes.len() != PAGE_SIZE {
            return Err(DatabaseError::InvalidData("Invalid page size".to_string()));
        }

        // Deserialize header
        let header = PageHeader::deserialize(&bytes[0..PAGE_HEADER_SIZE])?;

        // Deserialize slots
        let mut slots: Vec<SlotEntry> = Vec::new();
        let mut slot_offset = PAGE_HEADER_SIZE;

        for _ in 0..header.record_count {
            if slot_offset + 4 > bytes.len() {
                return Err(DatabaseError::InvalidData("Invalid slot directory".to_string()));
            }

            let slot = SlotEntry::deserialize(&bytes[slot_offset..slot_offset + 4])?;
            slots.push(slot);
            slot_offset += 4;
        }

        // Copy data section
        let mut data = vec![0u8; MAX_PAGE_DATA_SIZE];
        data.copy_from_slice(&bytes[PAGE_HEADER_SIZE..PAGE_HEADER_SIZE + MAX_PAGE_DATA_SIZE]);

        let page = Self {
            header,
            slots,
            data,
        };

        // Verify checksum
        // Note: In production, you'd want to verify the checksum here

        Ok(page)
    }

    /// Get available free space in bytes
    pub fn free_space(&self) -> usize {
        self.header.free_space_size as usize
    }

    /// Check if page can fit a record of given size
    pub fn can_fit(&self, record_size: usize) -> bool {
        let slot_size = 4;
        (self.header.free_space_size as usize) >= record_size + slot_size
    }
}
