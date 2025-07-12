use std::rc::Rc;

use crate::{config::Configuration, iter::StreamIterator};

/// Provides the configurations for tokenizers
/// the most basic being:
/// ```rust,ignore
/// Configuration<_, _>
/// ```
pub mod config;

#[doc(hidden)]
mod iter;

/// Gives you all the basic utilities
/// without scavenging for them.
pub mod prelude {
    pub use crate::config::*;
    pub use crate::*;
}

/// # Token
/// Represents a set of characters and their value
/// and position data. Also comes with a `kind`.
#[derive(Debug, Clone)]
pub struct Token<K: Default> {
    /* Value Data */
    pub value: String,
    pub kind: K,
    
    /* Position Data */
    pub row: usize,
    pub col: usize,
}

/// # Stream Tokenizer
/// A very basic tokenizer, no token trees, nothing.
/// Just creates a stream of tokens based on a set of rules.
/// 
/// # Example
/// 
/// ```rust
/// use roketok::prelude::*;
/// 
/// #[derive(Debug, Clone, Default)]
/// pub enum TokenKind {
///     Number,
/// 
///     Add,
///     Sub,
///     Mul,
///     Div,
/// 
///     #[default]
///     Invalid,
/// }
/// 
/// fn main() {
///     let config = Configuration::<TokenKind>::new()
///         .add_rule(|c| c.is_numeric(), TokenKind::Number)
///         .add_tokens([
///             (&['+'], TokenKind::Add),
///             (&['-'], TokenKind::Sub),
///             (&['*'], TokenKind::Mul),
///             (&['/'], TokenKind::Div),
///         ]);
///     let contents = "32 * 64 / 324 * 6 - 232 + 6644 + 324 * 3256 - 2".to_string();
///     let mut tokenizer = StreamTokenizer::new(config, &contents);
///     let stream = tokenizer.create_stream();
/// }
/// ```
pub struct StreamTokenizer<'ci, K: Default + Clone> {
    /* Configuration */
    config: Rc<Configuration<'ci, K>>,
    
    /* Content Iteration */
    iter: StreamIterator<'ci>,
    pos: (usize, usize),
}

impl<'ci, K: Default + Clone> StreamTokenizer<'ci, K> {
    /// Creates the `StreamTokenizer`, takes in basic config and
    /// file contents, or whatever you want to tokenize.
    pub fn new(config: Configuration<'ci, K>, contents: &'ci String) -> Self {
        Self {
            config: Rc::new(config),
            iter: StreamIterator::new(contents),
            pos: (1, 1),
        }
    }
    
    #[doc(hidden)]
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
    
    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    fn tokenize_symbols(&mut self,
        mut start_pos: (usize, usize),
        symbols: String
    ) -> Vec<Token<K>> {
        let mut stack = Vec::new();
        
        let mut slice = &symbols[..];
        while slice.len() != 0 {
            let matching = self.config.tokens.iter()
                .filter(|e| slice.starts_with(e.0))
                .collect::<Vec<_>>();
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
    
    /// # Create Stream
    /// This function, believe it or not creates the token stream.
    /// There are examples already showing how this works, so please refer
    /// to them.
    pub fn create_stream(&mut self) -> Box<[Token<K>]> {
        let mut stream = Vec::new();
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
                    .find(|e| e.0(&current, 0))
                {
                    let mut current_index = 1;
                    while let Some(current) = self.iter.peek() {
                        if !rule(&current, current_index) { break; }
                        self.next();
                        current_index += 1;
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
                        || config.rules.iter().find(|e| e.0(&current, 0)).is_some()
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
