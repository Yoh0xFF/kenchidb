use std::collections::HashMap;

use crate::{error::DatabaseError, value::Value};

// Schema definition for type safety
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    Boolean,
}

impl FieldType {
    pub fn validates(&self, value: &Value) -> bool {
        match (self, value) {
            (FieldType::Byte, Value::Byte(_)) => true,
            (FieldType::Short, Value::Short(_)) => true,
            (FieldType::Int, Value::Int(_)) => true,
            (FieldType::Long, Value::Long(_)) => true,
            (FieldType::Float, Value::Float(_)) => true,
            (FieldType::Double, Value::Double(_)) => true,
            (FieldType::String, Value::String(_)) => true,
            (FieldType::Boolean, Value::Boolean(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Schema {
    pub fn new(name: String, fields: Vec<Field>) -> Self {
        Self { name, fields }
    }

    pub fn validate_document(&self, document: &Document) -> Result<(), DatabaseError> {
        // Check that all required fields are present
        for field in &self.fields {
            match document.data.get(&field.name) {
                Some(value) => {
                    if !field.field_type.validates(value) {
                        return Err(DatabaseError::SchemaViolation(
                            format!("Field '{}' has wrong type. Expected {:?}, got {}",
                                field.name, field.field_type, value.type_name())
                        ));
                    }
                }
                None => {
                    if !field.nullable {
                        return Err(DatabaseError::SchemaViolation(
                            format!("Required field '{}' is missing", field.name)
                        ));
                    }
                }
            }
        }

        // Check that no extra fields are present
        for key in document.data.keys() {
            if !self.fields.iter().any(|f| f.name == *key) {
                return Err(DatabaseError::SchemaViolation(
                    format!("Unknown field '{}' not in schema", key)
                ));
            }
        }

        Ok(())
    }
}

// Document structure
#[derive(Debug, Clone)]
pub struct Document {
    pub id: u64,
    pub data: HashMap<String, Value>,
}

impl Document {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            data: HashMap::new(),
        }
    }

    pub fn set<T: Into<Value>>(&mut self, field: &str, value: T) {
        self.data.insert(field.to_string(), value.into());
    }

    pub fn get(&self, field: &str) -> Option<&Value> {
        self.data.get(field)
    }
}