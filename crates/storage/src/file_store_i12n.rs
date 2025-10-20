use crate::error::StorageError;
use crate::file_store::FileStore;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::atomic::{AtomicU64, Ordering};

impl FileStore {
    pub fn open(file_name: String, read_only: bool) -> Result<Self, StorageError> {
        let file = File::options()
            .read(true)
            .write(!read_only)
            .create(true)
            .open(file_name.clone())?;

        let metadata = file.metadata()?;

        Ok(FileStore {
            file,
            size: AtomicU64::new(metadata.len()),
            file_name,
            read_only,
            read_count: AtomicU64::new(0),
            read_bytes: AtomicU64::new(0),
            write_count: AtomicU64::new(0),
            write_bytes: AtomicU64::new(0),
        })
    }

    pub fn close(self) {
        drop(self.file);
    }

    pub fn size(&self) -> u64 {
        self.size.load(Ordering::Relaxed)
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn read_fully(&mut self, offset: u64, length: u32) -> Result<Vec<u8>, StorageError> {
        let size = self.size.load(Ordering::Relaxed);

        if offset >= size {
            return Err(StorageError::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Offset beyond the file size",
            )));
        }

        if offset + (length as u64) > size {
            return Err(StorageError::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Cannot read {} bytes at offset {}: would exceed file size {}",
                    length, offset, size
                ),
            )));
        }

        self.file.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0u8; length as usize];
        self.file.read_exact(&mut buffer)?;

        self.read_count.fetch_add(1, Ordering::Relaxed);
        self.read_bytes.fetch_add(length as u64, Ordering::Relaxed);

        Ok(buffer)
    }

    pub fn write_fully(&mut self, offset: u64, buffer: &[u8]) -> Result<(), StorageError> {
        if self.read_only {
            return Err(StorageError::ReadOnly(
                "File is open in a readonly mode".to_string(),
            ));
        }

        let length = buffer.len();

        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(&buffer)?;

        self.size
            .fetch_max(offset + (length as u64), Ordering::Relaxed);

        self.write_count.fetch_add(1, Ordering::Relaxed);
        self.write_bytes.fetch_add(length as u64, Ordering::Relaxed);

        Ok(())
    }

    pub fn sync(&self) -> Result<(), StorageError> {
        Ok(self.file.sync_all()?)
    }
}
