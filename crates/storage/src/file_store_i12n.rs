use std::fs::File;
use crate::error::StorageError;
use crate::file_store::FileStore;

impl FileStore {
    fn open(file_name: String, read_only: bool) -> Result<bool, StorageError> {
        let file = File::options()
            .read(true)
            .write(!read_only)
            .create(true)
            .open(file_name)?;

        Ok(true)
    }

    fn close() {}

    fn sync() {}

    fn read_fully() {}

    fn write_fully() {}

    fn size() {}

    fn get_file_name() {}
}