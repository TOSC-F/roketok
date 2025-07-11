use std::rc::Rc;

use crate::{config::Configuration, iter::StreamIterator};
use thin_vec::ThinVec;

pub mod config;
mod iter;

pub mod prelude {
    pub use crate::config::*;
    pub use crate::*;
}

#[derive(Debug, Clone)]
pub struct Token<K: Default> {
    /* Value Data */
    pub value: String,
    pub kind: K,
    
    /* Position Data */
    pub row: usize,
    pub col: usize,
}

pub struct StreamTokenizer<'ci, K: Default + Clone> {
    /* Configuration */
    config: Rc<Configuration<'ci, K>>,
    
    /* Content Iteration */
    iter: StreamIterator<'ci>,
    pos: (usize, usize),
}

impl<'ci, K: Default + Clone> StreamTokenizer<'ci, K> {
    pub fn new(config: Configuration<'ci, K>, contents: &'ci String) -> Self {
        Self {
            config: Rc::new(config),
            iter: StreamIterator::new(contents),
            pos: (1, 1),
        }
    }
    
    fn next(&mut self) -> Option<char> {
        if let Some(next) = self.iter.next() {
            if next == '\n' {
                self.pos.0 += 1;
                self.pos.1 = 1;
            } else {
                self.pos.1 += 1;
            }
            
            return Some(next);
        }
        
        None
    }
    
    #[must_use]
    #[inline(always)]
    fn tokenize_symbols(&mut self,
        mut start_pos: (usize, usize),
        symbols: String
    ) -> ThinVec<Token<K>> {
        let mut stack = ThinVec::new();
        
        let mut slice = &symbols[..];
        while slice.len() != 0 {
            let matching = self.config.tokens.iter()
                .filter(|e| slice.starts_with(e.0))
                .collect::<ThinVec<_>>();
            if matching.len() == 0 {
                stack.push(Token {
                    value: slice.to_string(),
                    kind: K::default(),
                    row: start_pos.0,
                    col: start_pos.1,
                });
                break;
            }
            
            let mut best_match = matching[0];
            for entry in matching {
                if best_match.0.len() < entry.0.len() {
                    best_match = entry;
                }
            }
            
            stack.push(Token {
                value: best_match.0.iter().collect::<String>(),
                kind: best_match.1.clone(),
                row: start_pos.0,
                col: start_pos.1,
            });
            
            let best_match_len = best_match.0.len();
            slice = &slice[best_match_len..];
            start_pos.1 += best_match_len;
        }
        
        stack
    }
    
    pub fn create_stream(&mut self) -> Box<[Token<K>]> {
        let mut stream = ThinVec::new();
        let config = self.config.clone();
        
        let mut start_iter_pos;
        let mut start_pos;
        'update: loop {
            start_iter_pos = self.iter.position();
            start_pos = self.pos;
            while let Some(current) = self.next() {
                if current.is_whitespace() {
                    continue 'update;
                }
                
                if let Some((rule, kind)) = config.rules.iter()
                    .find(|e| e.0(&current))
                {
                    while let Some(current) = self.iter.peek() {
                        if !rule(&current) { break; }
                        self.next();
                    }
                    
                    let end_iter_pos = self.iter.position();
                    let value = self.iter.grab(start_iter_pos..end_iter_pos);
                    stream.push(Token {
                        /* ValueData */
                        value,
                        kind: kind.clone(),
                        
                        /* Position Data */
                        row: start_pos.0,
                        col: start_pos.1
                    });
                    
                    continue 'update;
                }
                
                while let Some(current) = self.iter.peek() {
                    if current.is_whitespace()
                        || config.rules.iter().find(|e| e.0(&current)).is_some()
                    {
                        break;
                    }
                    
                    self.next();
                }
                
                let symbols = self.iter.grab(start_iter_pos..self.iter.position());
                stream.extend(self.tokenize_symbols(start_pos, symbols));
                continue 'update;
            }
            
            break;
        }
        
        stream.into()
    }
}
