use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}:{:?}", self.row + 1, self.column + 1)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
    pub location: Location,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}, {:?}", self.kind, self.value, self.location)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}, {:?}", self.kind, self.value, self.location)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    Ampersand,
    Bang,
    Caret,
    Pipe,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    At,
    Colon,
    Comma,
    Dollar,
    Dot,
    Hash,
    Newline,
    QuestionMark,
    Semicolon,
    Underscore,
    If,
    Elif,
    Else,
    For,
    While,
    Do,
    Loop,
    Fn,
    Return,
    True,
    False,
    Null,
    Identifier,
    Float,
    Integer,
    String,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Identifier(String),
    Boolean(bool),
    Float(f64),
    Integer(i128),
    String(String),
    Unknown(char),
    None,
}

impl TokenKind {
    pub fn is_primary(&self) -> bool {
        match self {
            TokenKind::If
            | TokenKind::Elif
            | TokenKind::Else
            | TokenKind::For
            | TokenKind::While
            | TokenKind::Do
            | TokenKind::Loop
            | TokenKind::Fn
            | TokenKind::Return
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Null
            | TokenKind::Identifier
            | TokenKind::Integer
            | TokenKind::String => true,
            _ => false,
        }
    }
}
