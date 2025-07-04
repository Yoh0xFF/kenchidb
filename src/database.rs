use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom, Write}, path::Path};

use crate::{error::DatabaseError, schema::{Document, Schema}, value::Value};

// Collection - stores documents with a specific schema
pub struct Collection {
    pub schema: Schema,
    pub documents: HashMap<u64, Document>,
    pub next_id: u64,
    pub file: Option<File>,
}

impl Collection {
    pub fn new(schema: Schema) -> Self {
        Self {
            schema,
            documents: HashMap::new(),
            next_id: 1,
            file: None,
        }
    }

    pub fn with_file<P: AsRef<Path>>(schema: Schema, path: P) -> Result<Self, DatabaseError> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;

        let mut collection = Self {
            schema,
            documents: HashMap::new(),
            next_id: 1,
            file: Some(file),
        };

        collection.load_from_file()?;
        Ok(collection)
    }

    pub fn insert(&mut self, mut document: Document) -> Result<u64, DatabaseError> {
        document.id = self.next_id;
        self.schema.validate_document(&document)?;

        self.documents.insert(document.id, document);
        self.next_id += 1;

        if self.file.is_some() {
            self.save_to_file()?;
        }

        Ok(self.next_id - 1)
    }

    pub fn find_by_id(&self, id: u64) -> Option<&Document> {
        self.documents.get(&id)
    }

    pub fn find_all(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }

    pub fn update(&mut self, id: u64, document: Document) -> Result<(), DatabaseError> {
        if !self.documents.contains_key(&id) {
            return Err(DatabaseError::DocumentNotFound(id));
        }

        let mut updated_doc = document;
        updated_doc.id = id;
        self.schema.validate_document(&updated_doc)?;

        self.documents.insert(id, updated_doc);

        if self.file.is_some() {
            self.save_to_file()?;
        }

        Ok(())
    }

    pub fn delete(&mut self, id: u64) -> Result<(), DatabaseError> {
        if self.documents.remove(&id).is_none() {
            return Err(DatabaseError::DocumentNotFound(id));
        }

        if self.file.is_some() {
            self.save_to_file()?;
        }

        Ok(())
    }

    fn save_to_file(&mut self) -> Result<(), DatabaseError> {
        // Simple serialization format
        let serialized = self.serialize();

        if let Some(ref mut file) = self.file {
            file.seek(SeekFrom::Start(0))?;
            file.set_len(0)?; // Truncate file

            file.write_all(&serialized)?;
            file.flush()?;
        }
        Ok(())
    }

    fn load_from_file(&mut self) -> Result<(), DatabaseError> {
        if let Some(ref mut file) = self.file {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            if !buffer.is_empty() {
                *self = Self::deserialize(&buffer, self.schema.clone())?;
            }
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Write document count
        bytes.extend_from_slice(&(self.documents.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.next_id.to_le_bytes());

        // Write documents
        for document in self.documents.values() {
            let doc_bytes = self.serialize_document(document);
            bytes.extend_from_slice(&(doc_bytes.len() as u32).to_le_bytes());
            bytes.extend_from_slice(&doc_bytes);
        }

        bytes
    }

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

    fn deserialize(bytes: &[u8], schema: Schema) -> Result<Self, DatabaseError> {
        let mut offset: usize;

        if bytes.len() < 12 {
            return Err(DatabaseError::InvalidData("File too short".to_string()));
        }

        // Read document count and next_id
        let doc_count = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let next_id = u64::from_le_bytes([
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11]
        ]);
        offset = 12;

        let mut documents = HashMap::new();

        // Read documents
        for _ in 0..doc_count {
            if offset + 4 > bytes.len() {
                return Err(DatabaseError::InvalidData("Incomplete document length".to_string()));
            }

            let doc_length = u32::from_le_bytes([
                bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
            ]) as usize;
            offset += 4;

            if offset + doc_length > bytes.len() {
                return Err(DatabaseError::InvalidData("Incomplete document data".to_string()));
            }

            let document = Self::deserialize_document(&bytes[offset..offset + doc_length])?;
            documents.insert(document.id, document);
            offset += doc_length;
        }

        Ok(Self {
            schema,
            documents,
            next_id,
            file: None,
        })
    }

    fn deserialize_document(bytes: &[u8]) -> Result<Document, DatabaseError> {
        let mut offset: usize;

        if bytes.len() < 12 {
            return Err(DatabaseError::InvalidData("Document data too short".to_string()));
        }

        // Read document ID
        let id = u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]);
        offset = 8;

        // Read field count
        let field_count = u32::from_le_bytes([
            bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
        ]) as usize;
        offset += 4;

        let mut data = HashMap::new();

        // Read fields
        for _ in 0..field_count {
            if offset >= bytes.len() {
                return Err(DatabaseError::InvalidData("Incomplete field data".to_string()));
            }

            // Read field name
            let key_len = bytes[offset] as usize;
            offset += 1;

            if offset + key_len > bytes.len() {
                return Err(DatabaseError::InvalidData("Incomplete field name".to_string()));
            }

            let key = String::from_utf8(bytes[offset..offset + key_len].to_vec())
                .map_err(|e| DatabaseError::InvalidData(format!("Invalid field name UTF-8: {}", e)))?;
            offset += key_len;

            // Read field value
            let (value, value_size) = Value::deserialize(&bytes[offset..])?;
            data.insert(key, value);
            offset += value_size;
        }

        Ok(Document { id, data })
    }
}

// Main Database struct
pub struct Database {
    collections: HashMap<String, Collection>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
        }
    }

    pub fn create_collection(&mut self, name: String, schema: Schema) -> Result<(), DatabaseError> {
        if self.collections.contains_key(&name) {
            return Err(DatabaseError::InvalidQuery(
                format!("Collection '{}' already exists", name)
            ));
        }

        self.collections.insert(name, Collection::new(schema));
        Ok(())
    }

    pub fn create_collection_with_file<P: AsRef<Path>>(
        &mut self,
        name: String,
        schema: Schema,
        path: P
    ) -> Result<(), DatabaseError> {
        if self.collections.contains_key(&name) {
            return Err(DatabaseError::InvalidQuery(
                format!("Collection '{}' already exists", name)
            ));
        }

        let collection = Collection::with_file(schema, path)?;
        self.collections.insert(name, collection);
        Ok(())
    }

    pub fn collection(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.get_mut(name)
    }
}