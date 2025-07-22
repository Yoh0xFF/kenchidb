use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

use crate::{
    common::DatabaseError,
    storage::page::{PAGE_SIZE, Page, PageType},
};

/// Manages file I/O operations for pages
pub struct FileManager {
    file: File,
    page_count: u32,
}

impl FileManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DatabaseError> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;

        // Calculate page count from file size
        let file_size = file.metadata()?.len();
        let page_count = (file_size / (PAGE_SIZE as u64)) as u32;

        Ok(Self { file, page_count })
    }

    /// Read a page from file
    pub fn read_page(&mut self, page_id: u32) -> Result<Page, DatabaseError> {
        if page_id >= self.page_count {
            return Err(DatabaseError::InvalidData(
                "Page ID out of bounds".to_string(),
            ));
        }

        let offset = (page_id as u64) * (PAGE_SIZE as u64);
        self.file.seek(SeekFrom::Start(offset))?;

        let mut buffer = [0u8; PAGE_SIZE];
        self.file.read_exact(&mut buffer)?;

        Page::deserialize(&buffer)
    }

    /// Write a page to file
    pub fn write_page(&mut self, page_id: u32, page: &mut Page) -> Result<(), DatabaseError> {
        let offset = (page_id as u64) * (PAGE_SIZE as u64);
        self.file.seek(SeekFrom::Start(offset))?;

        let page_bytes = page.serialize();
        self.file.write_all(&page_bytes)?;
        self.file.flush()?;

        // Update page count if we wrote beyond current file size
        if page_id >= self.page_count {
            self.page_count = page_id + 1;
        }

        Ok(())
    }

    /// Allocate a new page
    pub fn allocate_page(
        &mut self,
        page_type: PageType,
        collection_id: u32,
    ) -> Result<(u32, Page), DatabaseError> {
        let page_id = self.page_count;
        let page = Page::new(page_type, collection_id);
        self.page_count += 1;
        Ok((page_id, page))
    }

    pub fn page_count(&self) -> u32 {
        self.page_count
    }
}
