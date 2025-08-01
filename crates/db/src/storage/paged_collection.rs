use std::{collections::HashMap, path::Path};

use crate::{
    common::DatabaseError,
    schema::{Document, Value},
    storage::{file_manager::FileManager, page::PageType},
};

/// Enhanced collection that uses page-based storage
pub struct PagedCollection {
    pub schema: crate::schema::Schema,
    pub file_manager: FileManager,
    pub collection_id: u32,
    pub documents: HashMap<u64, (u32, u16)>, // document_id -> (page_id, slot_index)
    pub next_id: u64,
    pub current_page_id: Option<u32>, // Current page for insertions
}

impl PagedCollection {
    pub fn new<P: AsRef<Path>>(
        schema: crate::schema::Schema,
        collection_id: u32,
        file_path: P,
    ) -> Result<Self, DatabaseError> {
        let file_manager = FileManager::new(file_path)?;

        Ok(Self {
            schema,
            file_manager,
            collection_id,
            documents: HashMap::new(),
            next_id: 1,
            current_page_id: None,
        })
    }

    /// Insert a document using page-based storage
    pub fn insert(&mut self, mut document: Document) -> Result<u64, DatabaseError> {
        document.id = self.next_id;
        self.schema.validate_document(&document)?;

        // Serialize document using existing serialization
        let serialized_doc = self.serialize_document(&document);

        // Find or create a page with enough space
        let (page_id, slot_index) = self.find_page_for_insert(&serialized_doc)?;

        // Store mapping from document ID to page location
        self.documents.insert(document.id, (page_id, slot_index));
        self.next_id += 1;

        Ok(document.id)
    }

    /// Find a page with enough space for the record, or create a new one
    fn find_page_for_insert(&mut self, record_data: &[u8]) -> Result<(u32, u16), DatabaseError> {
        // Try current page first
        if let Some(current_page_id) = self.current_page_id {
            if let Ok(mut page) = self.file_manager.read_page(current_page_id) {
                if page.can_fit(record_data.len()) {
                    let slot_index = page.insert_record(record_data)?;
                    self.file_manager.write_page(current_page_id, &mut page)?;
                    return Ok((current_page_id, slot_index));
                }
            }
        }

        // Current page is full or doesn't exist, allocate new page
        let (page_id, mut page) = self
            .file_manager
            .allocate_page(PageType::DataPage, self.collection_id)?;

        let slot_index = page.insert_record(record_data)?;
        self.file_manager.write_page(page_id, &mut page)?;
        self.current_page_id = Some(page_id);

        Ok((page_id, slot_index))
    }

    /// Retrieve a document by ID
    pub fn find_by_id(&mut self, id: u64) -> Result<Option<Document>, DatabaseError> {
        if let Some((page_id, slot_index)) = self.documents.get(&id) {
            let page = self.file_manager.read_page(*page_id)?;
            let record_data = page.get_record(*slot_index)?;
            let document = self.deserialize_document(record_data)?;
            Ok(Some(document))
        } else {
            Ok(None)
        }
    }

    /// Reuse existing document serialization logic
    fn serialize_document(&self, document: &Document) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Write document ID
        bytes.extend_from_slice(&document.id.to_le_bytes());

        // Write field count
        bytes.extend_from_slice(&(document.data.len() as u32).to_le_bytes());

        // Write fields
        for (key, value) in &document.data {
            let key_bytes = key.as_bytes();
            bytes.push(key_bytes.len() as u8);
            bytes.extend_from_slice(key_bytes);

            let value_bytes = value.serialize();
            bytes.extend_from_slice(&value_bytes);
        }

        bytes
    }

    /// Reuse existing document deserialization logic
    fn deserialize_document(&self, bytes: &[u8]) -> Result<Document, DatabaseError> {
        let mut offset: usize;

        if bytes.len() < 12 {
            return Err(DatabaseError::InvalidData(
                "Document data too short".to_string(),
            ));
        }

        // Read document ID
        let id = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        offset = 8;

        // Read field count
        let field_count = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        let mut data = HashMap::new();

        // Read fields
        for _ in 0..field_count {
            if offset >= bytes.len() {
                return Err(DatabaseError::InvalidData(
                    "Incomplete field data".to_string(),
                ));
            }

            // Read field name
            let key_len = bytes[offset] as usize;
            offset += 1;

            if offset + key_len > bytes.len() {
                return Err(DatabaseError::InvalidData(
                    "Incomplete field name".to_string(),
                ));
            }

            let key = String::from_utf8(bytes[offset..offset + key_len].to_vec()).map_err(|e| {
                DatabaseError::InvalidData(format!("Invalid field name UTF-8: {}", e))
            })?;
            offset += key_len;

            // Read field value
            let (value, value_size) = Value::deserialize(&bytes[offset..])?;
            data.insert(key, value);
            offset += value_size;
        }

        Ok(Document { id, data })
    }

    /// Get statistics about the collection
    pub fn stats(&self) -> CollectionStats {
        CollectionStats {
            total_documents: self.documents.len(),
            total_pages: self.file_manager.page_count(),
            collection_id: self.collection_id,
        }
    }
}

#[derive(Debug)]
pub struct CollectionStats {
    pub total_documents: usize,
    pub total_pages: u32,
    pub collection_id: u32,
}
