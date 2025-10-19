use crate::error::StorageError;
use crate::file_store::FileStore;
use std::fs::File;

impl FileStore {
    fn open(file_name: String, read_only: bool) -> Result<Self, StorageError> {
        let file = File::options()
            .read(true)
            .write(!read_only)
            .create(true)
            .open(file_name)?;

        Ok(FileStore {
            file,
        })
    }

    fn close() {}

    fn sync() {}

    fn read_fully() {}

    fn write_fully() {}

    fn size() {}

    fn get_file_name() {}
}
