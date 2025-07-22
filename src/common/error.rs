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
