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
    Bool(bool),
    String(String), // Max 255 UTF-8 characters
    NullableByte(bool, u8),
    NullableShort(bool, i16),
    NullableInt(bool, i32),
    NullableLong(bool, i64),
    NullableFloat(bool, f32),
    NullableDouble(bool, f64),
    NullableBool(bool, bool),
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
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::NullableByte(_, _) => "nullable_byte",
            Value::NullableShort(_, _) => "nullable_short",
            Value::NullableInt(_, _) => "nullable_int",
            Value::NullableLong(_, _) => "nullable_long",
            Value::NullableFloat(_, _) => "nullable_float",
            Value::NullableDouble(_, _) => "nullable_double",
            Value::NullableBool(_, _) => "nullable_bool",
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Value::Byte(value) => vec![0, *value],
            Value::Short(value) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::Int(value) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::Long(value) => {
                let mut bytes = vec![3];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::Float(value) => {
                let mut bytes = vec![4];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::Double(value) => {
                let mut bytes = vec![5];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::Bool(value) => vec![6, if *value { 1 } else { 0 }],
            Value::String(value) => {
                if value.chars().count() > 255 {
                    panic!(
                        "String too long: {} characters (max 255)",
                        value.chars().count()
                    );
                }
                let utf8_bytes = value.as_bytes();
                let mut bytes = vec![7, utf8_bytes.len() as u8];
                bytes.extend_from_slice(utf8_bytes);
                bytes
            }
            Value::NullableByte(has_value, value) => {
                vec![8, *value, if *has_value { 1 } else { 0 }]
            }
            Value::NullableShort(has_value, value) => {
                let mut bytes = vec![9, if *has_value { 1 } else { 0 }];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::NullableInt(has_value, value) => {
                let mut bytes = vec![10, if *has_value { 1 } else { 0 }];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::NullableLong(has_value, value) => {
                let mut bytes = vec![11, if *has_value { 1 } else { 0 }];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::NullableFloat(has_value, value) => {
                let mut bytes = vec![12, if *has_value { 1 } else { 0 }];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::NullableDouble(has_value, value) => {
                let mut bytes = vec![13, if *has_value { 1 } else { 0 }];
                bytes.extend_from_slice(&value.to_le_bytes());
                bytes
            }
            Value::NullableBool(has_value, value) => vec![
                14,
                if *has_value { 1 } else { 0 },
                if *value { 1 } else { 0 },
            ],
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
                        "Incomplete boolean value".to_string(),
                    ));
                }
                Ok((Value::Bool(bytes[1] != 0), 2))
            }
            7 => {
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
            8 => {
                if bytes.len() < 3 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete nullable byte value".to_string(),
                    ));
                }
                Ok((Value::NullableByte(bytes[1] != 0, bytes[2]), 3))
            }
            9 => {
                if bytes.len() < 4 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete nullable short value".to_string(),
                    ));
                }
                let value = i16::from_le_bytes([bytes[2], bytes[3]]);
                Ok((Value::NullableShort(bytes[1] != 0, value), 4))
            }
            10 => {
                if bytes.len() < 6 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete nullable int value".to_string(),
                    ));
                }
                let value = i32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
                Ok((Value::NullableInt(bytes[1] != 0, value), 6))
            }
            11 => {
                if bytes.len() < 10 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete nullable long value".to_string(),
                    ));
                }
                let mut array = [0u8; 8];
                array.copy_from_slice(&bytes[2..10]);
                let value = i64::from_le_bytes(array);
                Ok((Value::NullableLong(bytes[1] != 0, value), 10))
            }
            12 => {
                if bytes.len() < 6 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete nullable float value".to_string(),
                    ));
                }
                let value = f32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
                Ok((Value::NullableFloat(bytes[1] != 0, value), 6))
            }
            13 => {
                if bytes.len() < 10 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete nullable double value".to_string(),
                    ));
                }
                let mut array = [0u8; 8];
                array.copy_from_slice(&bytes[2..10]);
                let value = f64::from_le_bytes(array);
                Ok((Value::NullableDouble(bytes[1] != 0, value), 10))
            }
            14 => {
                if bytes.len() < 3 {
                    return Err(DatabaseError::InvalidData(
                        "Incomplete nullable boolean value".to_string(),
                    ));
                }
                Ok((Value::NullableBool(bytes[1] != 0, bytes[2] != 0), 3))
            }
            _ => Err(DatabaseError::InvalidData(format!(
                "Unknown type tag: {}",
                bytes[0]
            ))),
        }
    }
}
