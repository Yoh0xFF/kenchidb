#[derive(Debug)]
pub enum StorageError {
    InvalidChunkHeader(String),
    IoError(std::io::Error),
}

impl From<std::io::Error> for StorageError {
    fn from(error: std::io::Error) -> Self {
        StorageError::IoError(error)
    }
}