use roketok::{prelude::*, StreamTokenizer};

#[derive(Debug, Clone, Default)]
pub enum TokenKind {
    Number,
    
    Add,
    Sub,
    Mul,
    Div,
    
    #[default]
    Invalid,
}

fn main() {
    let config = Configuration::<TokenKind>::new()
        .add_rule(|c| c.is_numeric(), TokenKind::Number)
        .add_tokens([
            (&['+'], TokenKind::Add),
            (&['-'], TokenKind::Sub),
            (&['*'], TokenKind::Mul),
            (&['/'], TokenKind::Div),
        ]);
    
    let contents = "32 * 64 / 324 * 6 - 232 + 6644 + 324 * 3256 - 2".to_string();
    let mut tokenizer = StreamTokenizer::new(config, &contents);
    let stream = tokenizer.create_stream();
    
    println!("{:?}", stream);
}
