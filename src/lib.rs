use crate::{
    config::{Configuration, TokenConfiguration},
    tokens::{Token, TreeNode},
    iter::StreamIterator,
};

/// Provides the configurations for tokenizers
/// the most basic being:
/// ```rust,ignore
/// Configuration<_, _>
/// ```
pub mod config;

/// Provides `Record` which records the tokenizers
/// data, allowing for more complex tokenizations.
pub mod record;

/// Provides the essentials for interacting and manipulating
/// tokens after tokenization.
pub mod tokens;

#[doc(hidden)]
mod iter;

/// Gives you all the basic utilities
/// without scavenging for them.
pub mod prelude {
    pub use crate::config::*;
    pub use crate::Tokenizer;
}

/// # Tokenizer
/// Uses `Configuration` to tokenize contents.
/// 
/// # Example (Taken from README.md)
/// ```rust
/// use roketok::prelude::*;
/// 
/// #[derive(Default, Clone)]
/// enum TokenKind {
///     Identifier,
///     
///     Asterisk,
///     Ampersand,
///     Semicolon,
///     
///     Equal,
///     AddEqual,
///     
///     Parenthesis,
///     
///     #[default]
///     Invalid,
/// }
/// 
/// fn main() {
///     let contents = r#"
///         void foo(int *value) {
///             *value += 35;
///         }
///         
///         int main(void) {
///             int value = 34;
///             foo(&value);
///             return value;
///         }
///     "#;
///     
///     let config = Configuration::new()
///         .add_tokens([
///             (TokenConfiguration::Rule(&|iter, _| {
///                 if let Some(char) = iter.last() {
///                     if !char.is_alphabetic() { return false; }
///                     while let Some(char) = iter.peek() {
///                         if !char.is_alphanumeric() { break; }
///                         let _ = iter.next();
///                     }
///                     return true;
///                 }
///                 false
///             }), TokenKind::Identifier),
///             
///             (TokenConfiguration::Boring(&['*']), TokenKind::Asterisk),
///             (TokenConfiguration::Boring(&['&']), TokenKind::Ampersand),
///             
///             (TokenConfiguration::Boring(&['=']), TokenKind::Equal),
///             (TokenConfiguration::Boring(&['+', '=']), TokenKind::AddEqual),
///             
///             (TokenConfiguration::Boring(&[';']), TokenKind::Semicolon),
///             
///             (TokenConfiguration::Branch(&['('], &[')']), TokenKind::Parenthesis),
///         ]);
///     let mut tokenizer = Tokenizer::new(&config, contents);
///     let tree = tokenizer.build();
/// }
/// ```
pub struct Tokenizer<'items, K: Default + Clone> {
    config: &'items Configuration<'items, K>,
    iter: StreamIterator<'items>,
}

impl<'items, K: Default + Clone> Tokenizer<'items, K> {
    /// Creates a new `Tokenizer` from a configuration and the
    /// contents (the `String` you want to tokenize).
    /// 
    /// # Example
    /// ```rust
    /// use roketok::prelude::*;
    /// 
    /// #[derive(Default, Clone)]
    /// enum TokenKind {
    ///     #[default]
    ///     Invalid,
    /// }
    /// 
    /// let contents = "This gets tokenized. But configuration is empty, so in this case it doesn't.";
    /// 
    /// let config = Configuration::<'_, TokenKind>::new();
    /// let tokenizer = Tokenizer::new(&config, &contents);
    /// ```
    /// 
    /// See [`Tokenizer`] for more details.
    pub fn new(config: &'items Configuration<'items, K>, contents: &'items str) -> Self {
        Self {
            config,
            iter: StreamIterator::new(contents),
        }
    }
    
    #[doc(hidden)]
    fn tokenize(&mut self) -> TreeNode<K> {
        let start_iter_pos = self.iter.position() - 1;
        for (config, kind) in self.config.0.iter() {
            match config {
                TokenConfiguration::Rule(rule) => {
                    let record = self.iter.record().clone();
                    if rule(&mut self.iter, &record) == true {
                        return TreeNode::Leaf(Token {
                            value: self.iter.grab(start_iter_pos..self.iter.position()),
                            kind: kind.clone(),
                            record,
                        });
                    }
                },
                TokenConfiguration::Boring(chars) => {
                    let mut iter = self.iter;
                    let mut matches = false;
                    for (i, char) in (0..chars.len()).zip(iter.last()) {
                        if char == chars[i] {
                            matches = true;
                        } else {
                            matches = false;
                            break;
                        }
                        
                        if i + 1 != chars.len() {
                            let _ = iter.next();
                        }
                    }
                    
                    if matches {
                        let record = self.iter.record().clone();
                        self.iter = iter;
                        return TreeNode::Leaf(Token {
                            value: self.iter.grab(start_iter_pos..self.iter.position()),
                            kind: kind.clone(),
                            record,
                        });
                    }
                },
                
                // TODO
                TokenConfiguration::Branch(..) => todo!(),
            }
        }
        
        TreeNode::Leaf(Token {
            value: self.iter.last().unwrap().to_string(),
            kind: K::default(),
            record: self.iter.record().clone(),
        })
    }
    
    /// # Builds the Token Tree
    /// Creates the token tree using the configuration
    /// and contents you provided in new. See
    /// [`Tokenizer::new`] for more details.
    pub fn build(&mut self) -> Vec<TreeNode<K>> {
        let mut stream = Vec::new();
        
        while let Some(char) = self.iter.next() {
            if char.is_whitespace() { continue; }
            stream.push(self.tokenize());
        }
        
        stream
    }
}
