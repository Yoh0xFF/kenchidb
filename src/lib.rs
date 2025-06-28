use std::io;

// Error types
#[derive(Debug)]
pub enum DatabaseError {
    IoError(io::Error),
    SchemaViolation(String),
    InvalidData(String),
    DocumentNotFound(u64),
    InvalidQuery(String),
}

impl From<io::Error> for DatabaseError {
    fn from(error: io::Error) -> Self {
        DatabaseError::IoError(error)
    }
}

// Core primitive types for the database
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String), // Max 255 UTF-8 characters
    Bool(bool),
}

impl Value {
    pub fn get_type_name(&self) -> &'static str {
        match self {
            Value::Byte(_) => "byte",
            Value::Short(_) => "short",
            Value::Int(_) => "int",
            Value::Long(_) => "long",
            Value::Float(_) => "float",
            Value::Double(_) => "double",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Value::Byte(v) => vec![0, *v],
            Value::Short(v) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(&v.to_le_bytes());
                bytes
            }
            Value::Int(v) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(&v.to_le_bytes());
                bytes
            }
            Value::Long(v) => {
                let mut bytes = vec![3];
                bytes.extend_from_slice(&v.to_le_bytes());
                bytes
            }
            Value::Float(v) => {
                let mut bytes = vec![4];
                bytes.extend_from_slice(&v.to_le_bytes());
                bytes
            }
            Value::Double(v) => {
                let mut bytes = vec![5];
                bytes.extend_from_slice(&v.to_le_bytes());
                bytes
            }
            Value::String(v) => {
                if v.chars().count() > 255 {
                    panic!(
                        "String too long: {} characters (max 255)",
                        v.chars().count()
                    );
                }
                let utf8_bytes = v.as_bytes();
                let mut bytes = vec![6, utf8_bytes.len() as u8];
                bytes.extend_from_slice(utf8_bytes);
                bytes
            }
            Value::Bool(v) => vec![7, if *v { 1 } else { 0 }],
        }
    }

    pub fn deserialize(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
        if bytes.is_empty() {
            return Err(DatabaseError::InvalidData("Empty bytes".to_string()));
        }

        match bytes[0] {
            0 => {
                if bytes.len() < 2 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete byte value".to_string(),
                    ));
                }
                Ok((Value::Byte(bytes[1]), 2))
            }
            1 => {
                if bytes.len() < 3 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete short value".to_string(),
                    ));
                }
                let value = i16::from_le_bytes([bytes[1], bytes[2]]);
                Ok((Value::Short(value), 3))
            }
            2 => {
                if bytes.len() < 5 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete int value".to_string(),
                    ));
                }
                let value = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                Ok((Value::Int(value), 5))
            }
            3 => {
                if bytes.len() < 9 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete long value".to_string(),
                    ));
                }
                let mut array = [0u8; 8];
                array.copy_from_slice(&bytes[1..9]);
                let value = i64::from_le_bytes(array);
                Ok((Value::Long(value), 9))
            }
            4 => {
                if bytes.len() < 5 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete float value".to_string(),
                    ));
                }
                let value = f32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                Ok((Value::Float(value), 5))
            }
            5 => {
                if bytes.len() < 9 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete double value".to_string(),
                    ));
                }
                let mut array = [0u8; 8];
                array.copy_from_slice(&bytes[1..9]);
                let value = f64::from_le_bytes(array);
                Ok((Value::Double(value), 9))
            }
            6 => {
                if bytes.len() < 2 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete string length".to_string(),
                    ));
                }
                let len = bytes[1] as usize;
                if bytes.len() < 2 + len {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete string data".to_string(),
                    ));
                }
                let string_bytes = &bytes[2..2 + len];
                let value = String::from_utf8(string_bytes.to_vec())
                    .map_err(|e| DatabaseError::InvalidData(format!("Invalid UTF-8: {}", e)))?;
                Ok((Value::String(value), 2 + len))
            }
            7 => {
                if bytes.len() < 2 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete boolean value".to_string(),
                    ));
                }
                Ok((Value::Bool(bytes[1] != 0), 2))
            }
            _ => Err(DatabaseError::InvalidData(format!(
                "Unknown type tag: {}",
                bytes[0]
            ))),
        }
    }
}
