use thin_vec::ThinVec;

/// # Configuration
/// The most simple configuration system, used by
/// `StreamTokenizer` as a way to setup rules and tokens.
/// 
/// # Example
/// 
/// ```rust
/// use roketok::prelude::Configuration;
/// 
/// #[derive(Clone, Copy, Default)]
/// enum TokenKind {
///     Number,
/// 
///     Plus,
///     Equal,
///     FatArrow,
/// 
///     #[default]
///     Invalid,
/// }
/// 
/// fn main() {
///     let config = Configuration::<TokenKind>::new()
///         .add_rule(|c| c.is_numeric(), TokenKind::Number)
///         .add_tokens([
///             (&['+'], TokenKind::Plus),
///             (&['='], TokenKind::Equal),
///             (&['=', '>'], TokenKind::FatArrow),
///         ]);
/// }
/// ```
pub struct Configuration<'s, K: Default> {
    pub(crate) rules: ThinVec<(Box<dyn Fn(&char) -> bool>, K)>,
    pub(crate) tokens: ThinVec<(&'s [char], K)>,
}

impl<'s, K: Default> Configuration<'s, K> {
    /// Creates the very basic `Configuration`
    pub fn new() -> Self {
        Self {
            rules: ThinVec::new(),
            tokens: ThinVec::new(),
        }
    }
    
    /// # Add Rule
    /// Quite literally does what it says. But you can define
    /// any rule, as long as it works char by char. Currently
    /// advanced rule systems are reserved for other systems.
    /// Examples are already shown, please refer to them.
    #[must_use]
    #[inline(always)]
    pub fn add_rule<F>(mut self, f: F, kind: K) -> Self
    where
        F: Fn(&char) -> bool + 'static,
    {
        self.rules.push((Box::new(f), kind));
        self
    }
    
    /// # Add Tokens
    /// Adds a table of tokens to your configuration.
    /// Each entry is simply in this format:
    /// ```rust,ignore
    /// (&['_', ..], K)
    /// ```
    /// more of these are previously shown, please
    /// refer to those examples.
    #[must_use]
    #[inline(always)]
    pub fn add_tokens<const N: usize>(mut self, tokens: [(&'s [char], K) ; N]) -> Self {
        self.tokens.extend(tokens);
        self
    }
}
