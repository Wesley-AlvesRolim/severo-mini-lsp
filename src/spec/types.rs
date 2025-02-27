#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
    pub literal: Option<Literal>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TokenType {
    Var,
    Identifier,
    String,
    Number,
    Print,
    Equal,
    LeftParen,
    RightParen,
    Eof,
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub struct ScanResult {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ScanError {
    pub message: String,
    pub col: usize,
}
