use crate::common::DatabaseError;

/**
 * Type IDs for the database value types.
 */
const TYPE_BYTE_ID: u8 = 0;
const TYPE_SHORT_ID: u8 = 1;
const TYPE_INT_ID: u8 = 2;
const TYPE_LONG_ID: u8 = 3;
const TYPE_FLOAT_ID: u8 = 4;
const TYPE_DOUBLE_ID: u8 = 5;
const TYPE_BOOLEAN_ID: u8 = 6;
const TYPE_STRING_ID: u8 = 7;

/**
 * Size of the database value types.
 */
const TYPE_BYTE_SIZE: usize = 2; // type_id + value
const TYPE_SHORT_SIZE: usize = 3; // type_id + 2 bytes
const TYPE_INT_SIZE: usize = 5; // type_id + 4 bytes
const TYPE_LONG_SIZE: usize = 9; // type_id + 8 bytes
const TYPE_FLOAT_SIZE: usize = 5; // type_id + 4 bytes
const TYPE_DOUBLE_SIZE: usize = 9; // type_id + 8 bytes
const TYPE_BOOLEAN_SIZE: usize = 2; // type_id + 1 byte
const TYPE_STRING_SIZE: usize = 256; // type_id + 1 byte + 255 bytes

/**
 * Names for the database value types.
 */
const TYPE_BYTE_NAME: &str = "byte";
const TYPE_SHORT_NAME: &str = "short";
const TYPE_INT_NAME: &str = "int";
const TYPE_LONG_NAME: &str = "long";
const TYPE_FLOAT_NAME: &str = "float";
const TYPE_DOUBLE_NAME: &str = "double";
const TYPE_BOOLEAN_NAME: &str = "boolean";
const TYPE_STRING_NAME: &str = "string";

/**
 * Core primitive types for the database.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    String(String), // Max 255 UTF-8 characters
}

impl Value {
    /**
     * Get the type ID for the value.
     */
    pub fn type_id(&self) -> u8 {
        match self {
            Value::Byte(_) => TYPE_BYTE_ID,
            Value::Short(_) => TYPE_SHORT_ID,
            Value::Int(_) => TYPE_INT_ID,
            Value::Long(_) => TYPE_LONG_ID,
            Value::Float(_) => TYPE_FLOAT_ID,
            Value::Double(_) => TYPE_DOUBLE_ID,
            Value::Boolean(_) => TYPE_BOOLEAN_ID,
            Value::String(_) => TYPE_STRING_ID,
        }
    }

    /**
     * Get the size of the value.
     */
    pub fn type_size(&self) -> usize {
        match self {
            Value::Byte(_) => TYPE_BYTE_SIZE,
            Value::Short(_) => TYPE_SHORT_SIZE,
            Value::Int(_) => TYPE_INT_SIZE,
            Value::Long(_) => TYPE_LONG_SIZE,
            Value::Float(_) => TYPE_FLOAT_SIZE,
            Value::Double(_) => TYPE_DOUBLE_SIZE,
            Value::Boolean(_) => TYPE_BOOLEAN_SIZE,
            Value::String(_) => TYPE_STRING_SIZE,
        }
    }

    /**
     * Get the name of the value type.
     */
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Byte(_) => TYPE_BYTE_NAME,
            Value::Short(_) => TYPE_SHORT_NAME,
            Value::Int(_) => TYPE_INT_NAME,
            Value::Long(_) => TYPE_LONG_NAME,
            Value::Float(_) => TYPE_FLOAT_NAME,
            Value::Double(_) => TYPE_DOUBLE_NAME,
            Value::Boolean(_) => TYPE_BOOLEAN_NAME,
            Value::String(_) => TYPE_STRING_NAME,
        }
    }

    /**
     * Serialize the value to a byte array.
     */
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Value::Byte(value) => serialize_byte(*value),
            Value::Short(value) => serialize_short(*value),
            Value::Int(value) => serialize_int(*value),
            Value::Long(value) => serialize_long(*value),
            Value::Float(value) => serialize_float(*value),
            Value::Double(value) => serialize_double(*value),
            Value::Boolean(value) => serialize_boolean(*value),
            Value::String(value) => serialize_string(value),
        }
    }

    pub fn deserialize(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
        if bytes.is_empty() {
            return Err(DatabaseError::InvalidData("Empty bytes".to_string()));
        }

        match bytes[0] {
            TYPE_BYTE_ID => deserialize_byte(bytes),
            TYPE_SHORT_ID => deserialize_short(bytes),
            TYPE_INT_ID => deserialize_int(bytes),
            TYPE_LONG_ID => deserialize_long(bytes),
            TYPE_FLOAT_ID => deserialize_float(bytes),
            TYPE_DOUBLE_ID => deserialize_double(bytes),
            TYPE_BOOLEAN_ID => deserialize_boolean(bytes),
            TYPE_STRING_ID => deserialize_string(bytes),
            _ => Err(DatabaseError::InvalidData(format!(
                "Unknown type tag: {}",
                bytes[0]
            ))),
        }
    }
}

/**
 * Serialize values to bytes.
 */
#[inline]
fn serialize_byte(value: u8) -> Vec<u8> {
    vec![TYPE_BYTE_ID, value]
}

#[inline]
fn serialize_short(value: i16) -> Vec<u8> {
    let mut bytes = vec![TYPE_SHORT_ID];
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes
}

#[inline]
fn serialize_int(value: i32) -> Vec<u8> {
    let mut bytes = vec![TYPE_INT_ID];
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes
}

#[inline]
fn serialize_long(value: i64) -> Vec<u8> {
    let mut bytes = vec![TYPE_LONG_ID];
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes
}

#[inline]
fn serialize_float(value: f32) -> Vec<u8> {
    let mut bytes = vec![TYPE_FLOAT_ID];
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes
}

#[inline]
fn serialize_double(value: f64) -> Vec<u8> {
    let mut bytes = vec![TYPE_DOUBLE_ID];
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes
}

#[inline]
fn serialize_boolean(value: bool) -> Vec<u8> {
    vec![TYPE_BOOLEAN_ID, if value { 1 } else { 0 }]
}

#[inline]
fn serialize_string(value: &str) -> Vec<u8> {
    if value.chars().count() > 255 {
        panic!(
            "String too long: {} characters (max 255)",
            value.chars().count()
        );
    }
    let utf8_bytes = value.as_bytes();
    let mut bytes = vec![TYPE_STRING_ID, utf8_bytes.len() as u8];
    bytes.extend_from_slice(utf8_bytes);
    bytes
}

/**
 * Deserialize bytes to values.
 */
#[inline]
fn deserialize_byte(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
    if bytes.len() < TYPE_BYTE_SIZE {
        return Err(DatabaseError::InvalidData(
            "Incomplete byte value".to_string(),
        ));
    }
    Ok((Value::Byte(bytes[1]), TYPE_BYTE_SIZE))
}

#[inline]
fn deserialize_short(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
    if bytes.len() < TYPE_SHORT_SIZE {
        return Err(DatabaseError::InvalidData(
            "Incomplete short value".to_string(),
        ));
    }
    let value = i16::from_le_bytes([bytes[1], bytes[2]]);
    Ok((Value::Short(value), TYPE_SHORT_SIZE))
}

#[inline]
fn deserialize_int(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
    if bytes.len() < TYPE_INT_SIZE {
        return Err(DatabaseError::InvalidData(
            "Incomplete int value".to_string(),
        ));
    }
    let value = i32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
    Ok((Value::Int(value), TYPE_INT_SIZE))
}

#[inline]
fn deserialize_long(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
    if bytes.len() < TYPE_LONG_SIZE {
        return Err(DatabaseError::InvalidData(
            "Incomplete long value".to_string(),
        ));
    }
    let mut array = [0u8; 8];
    array.copy_from_slice(&bytes[1..9]);
    let value = i64::from_le_bytes(array);
    Ok((Value::Long(value), TYPE_LONG_SIZE))
}

#[inline]
fn deserialize_float(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
    if bytes.len() < TYPE_FLOAT_SIZE {
        return Err(DatabaseError::InvalidData(
            "Incomplete float value".to_string(),
        ));
    }
    let value = f32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
    Ok((Value::Float(value), TYPE_FLOAT_SIZE))
}

#[inline]
fn deserialize_double(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
    if bytes.len() < TYPE_DOUBLE_SIZE {
        return Err(DatabaseError::InvalidData(
            "Incomplete double value".to_string(),
        ));
    }
    let mut array = [0u8; 8];
    array.copy_from_slice(&bytes[1..9]);
    let value = f64::from_le_bytes(array);
    Ok((Value::Double(value), TYPE_DOUBLE_SIZE))
}

#[inline]
fn deserialize_boolean(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
    if bytes.len() < TYPE_BOOLEAN_SIZE {
        return Err(DatabaseError::InvalidData(
            "Incomplete boolean value".to_string(),
        ));
    }
    Ok((Value::Boolean(bytes[1] != 0), TYPE_BOOLEAN_SIZE))
}

#[inline]
fn deserialize_string(bytes: &[u8]) -> Result<(Value, usize), DatabaseError> {
    if bytes.len() < TYPE_STRING_SIZE {
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
