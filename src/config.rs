use thin_vec::ThinVec;

pub struct Configuration<'s, K: Default> {
    pub(crate) rules: ThinVec<(Box<dyn Fn(&char) -> bool>, K)>,
    pub(crate) tokens: ThinVec<(&'s [char], K)>,
}

impl<'s, K: Default> Configuration<'s, K> {
    pub fn new() -> Self {
        Self {
            rules: ThinVec::new(),
            tokens: ThinVec::new(),
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub fn add_rule<F>(mut self, f: F, kind: K) -> Self
    where
        F: Fn(&char) -> bool + 'static,
    {
        self.rules.push((Box::new(f), kind));
        self
    }
    
    #[must_use]
    #[inline(always)]
    pub fn add_tokens<const N: usize>(mut self, tokens: [(&'s [char], K) ; N]) -> Self {
        self.tokens.extend(tokens);
        self
    }
}
