use crate::record::Record;

#[derive(Clone, Copy)]
pub struct StreamIterator<'str> {
    contents: &'str [u8],
    pos: usize,
    record: Record,
}

impl<'str> StreamIterator<'str> {
    pub(crate) fn new(stream: &'str str) -> Self {
        Self {
            contents: stream.as_bytes(),
            pos: 0,
            record: Record {
                pos: (1, 1)
            }
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub fn peek(&self) -> Option<char> {
        match self.contents.get(self.pos) {
            Some(byte) => Some(*byte as char),
            None => None,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub fn last(&self) -> Option<char> {
        match self.contents.get(self.pos - 1) {
            Some(byte) => Some(*byte as char),
            None => None,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub fn next(&mut self) -> Option<char> {
        match self.contents.get(self.pos) {
            Some(byte) => {
                let char = *byte as char;
                if char == '\n' {
                    self.record.pos.0 += 1;
                    self.record.pos.1 = 0; 
                } else {
                    self.record.pos.1 += 1;
                }
                self.pos += 1;
                Some(char)
            },
            None => None,
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub fn position(&self) -> usize {
        self.pos as usize
    }
    
    #[must_use]
    pub fn grab<Idx: Iterator<Item=usize>>(&self, idx: Idx) -> String {
        let mut string = String::new();
        for i in idx {
            string.push(self.contents[i] as char);
        }
        string
    }
    
    #[must_use]
    pub fn record(&self) -> &Record {
        &self.record
    }
}
