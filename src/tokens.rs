use crate::record::Record;

/// # Token
/// Represents a set of characters and their value
/// and record. Also comes with a `kind`.
#[derive(Debug, Clone)]
pub struct Token<K> {
    /* Value Data */
    pub value: String,
    pub kind: K,
    
    /* Record Data */
    pub record: Record,
}

/// # Branch
/// Represents a `Branch` in the token tree.
/// For example, paranthesis may be a branch.
#[derive(Debug, Clone)]
pub struct Branch<K> {
    /* Value Data */
    pub value: (String, String),
    pub kind: K,
    
    /* Stream Data */
    pub stream: Vec<TreeNode<K>>,
    
    /* Record Data */
    pub record: Record,
    pub has_end: bool,
}

// # Token Tree Node
// Represents a token tree node, allows
// flexibility in tokenization.
#[derive(Debug, Clone)]
pub enum TreeNode<K> {
    /// `Branch` contains the start and end tokens
    /// as well as a tree stream, so for example parenthesis
    /// might become a branch in certain configurations.
    Branch(Branch<K>),
    
    /// `Leaf` contains only a token and does not
    /// support branches or even other leafs.
    Leaf(Token<K>),
}
