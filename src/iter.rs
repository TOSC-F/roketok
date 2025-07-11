pub struct StreamIterator<'str>(&'str [u8], usize);
impl<'str> StreamIterator<'str> {
    pub(crate) fn new(stream: &'str str) -> Self {
        Self(stream.as_bytes(), 0)
    }
    
    #[must_use]
    #[inline(always)]
    pub fn peek(&self) -> Option<char> {
        match self.0.get(self.1) {
            Some(byte) => Some(*byte as char),
            None => None,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub fn next(&mut self) -> Option<char> {
        match self.0.get(self.1) {
            Some(byte) => {
                self.1 += 1;
                Some(*byte as char)
            },
            None => None,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub fn position(&self) -> usize {
        self.1 as usize
    }
    
    #[must_use]
    pub fn grab<Idx: Iterator<Item=usize>>(&mut self, idx: Idx) -> String {
        let mut string = String::new();
        for i in idx {
            string.push(self.0[i] as char);
        }
        string
    }
}
