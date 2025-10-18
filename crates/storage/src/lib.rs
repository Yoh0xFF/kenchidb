mod chunk;
mod chunk_i12n;
mod chunk_i12n_margin;
mod data_util;
mod error;
mod file_store;
mod file_store_i12n;
mod page;
mod page_impl;
mod storage_engine;
mod test;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
