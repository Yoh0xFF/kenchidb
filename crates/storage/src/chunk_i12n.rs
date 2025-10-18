use crate::chunk::Chunk;

impl Chunk {
    pub fn is_allocated(&self) -> bool {
        self.block != 0
    }

    pub fn is_saved(&self) -> bool {
        self.is_allocated() && self.buffer.is_empty()
    }

    pub fn is_live(&self) -> bool {
        self.page_count_live > 0
    }

    pub fn is_evacutable(&self) -> bool {
        self.pin_count == 0
    }

    pub fn is_rewritable(&self) -> bool {
        self.is_saved()
            && self.is_live()
            && self.is_evacutable()
            && (self.page_count_live < self.page_count) // Not fully occupied
    }

}