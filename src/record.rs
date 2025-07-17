/// # Record
/// Records the tokenizers data like current row, column, etc.
/// It is used in tokens, rules, etc.
#[derive(Debug, Clone, Copy)]
pub struct Record {
    pub(crate) pos: (usize, usize),
}

impl Record {
    /// Get the recorded row.
    #[must_use]
    #[inline(always)]
    pub fn row(&self) -> usize {
        self.pos.0
    }
    
    /// Get the recorded column.
    #[must_use]
    #[inline(always)]
    pub fn col(&self) -> usize {
        self.pos.1
    }
}
