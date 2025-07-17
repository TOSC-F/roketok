use crate::{iter::StreamIterator, record::Record};

/// # Token Configuration
/// Only certain configurations support this, but it
/// basically describes what a token does.
pub enum TokenConfiguration<'tok> {
    /// The absolute most boring token ever, does absolutely
    /// nothing and is just processed normally.
    Boring(&'tok [char]),
    
    /// A `branch` contains a start and end token, it basically
    /// creates a new branch in the token tree.
    Branch(&'tok [char], &'tok [char]),
    
    /// A rule based token, like an identifier for example.
    /// These tokens are not bound by a strict symbol-only
    /// token system and they can do whatever they want.
    Rule(&'tok dyn Fn(&mut StreamIterator, &Record) -> bool),
}


/// # Basic Configuration
/// The most simple configuration system, used by
/// `StreamTokenizer` as a way to setup rules and tokens.
/// 
/// # Example
/// 
/// ```rust
/// use roketok::prelude::*;
/// 
/// #[derive(Clone, Copy, Default)]
/// enum TokenKind {
///     Number,
/// 
///     Plus,
///     Equal,
///     FatArrow,
/// 
///     Parenthesis,
/// 
///     #[default]
///     Invalid,
/// }
///  
/// let config = Configuration::<TokenKind>::new()
///     .add_tokens([
///         (TokenConfiguration::Branch(&['('], &[')']), TokenKind::Parenthesis),
///         (TokenConfiguration::Boring(&['+']), TokenKind::Plus),
///         (TokenConfiguration::Boring(&['=']), TokenKind::Equal),
///         (TokenConfiguration::Boring(&['=', '>']), TokenKind::FatArrow),
///     ]);
/// ```
pub struct Configuration<'tok, K: Default>(pub(crate) Vec<(TokenConfiguration<'tok>, K)>);

impl<'tok, K: Default> Configuration<'tok, K> {
    /// Creates the very basic `Configuration`
    pub const fn new() -> Self {
        Self(Vec::new())
    }
    
    /// # Add Tokens
    /// Extends the amount of tokens stored in the Configuration.
    /// It's relatively simple and every entry takes in a tuple of
    /// a `TokenConfiguration` and the token kind item `K`.
    #[must_use]
    #[inline(always)]
    pub fn add_tokens<const N: usize>(mut self, tokens: [(TokenConfiguration<'tok>, K) ; N]) -> Self {
        self.0.extend(tokens);
        self
    }
}
