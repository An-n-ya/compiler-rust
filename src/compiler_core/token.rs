use std::{collections::HashMap, fmt, any::Any};


// lazy_static! {
//     pub static ref KEYWORDS: HashMap<&'static str, self::Token> = HashMap::from([
//         ("fn", Token::FUNCTION),   
//         ("let", Token::FUNCTION),   
//     ]);
// }

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
#[allow(dead_code, non_camel_case_types)]
pub enum TokenType {
    

    ILLEGAL,
    EOF,
    // literal
    IDENT,
    NUMBER,
    STRING,
    
    // operator
    ASSIGN,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EXCLAMATION,
    GT,
    LT,
    GE,
    LE,
    CARET,

    EQ,
    NOT_EQ,

    COMMA,
    SEMICOLON,
    COLON,
    DOT,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    
    // 关键字
    FUNCTION,
    CLASS,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
    AND,
    OR,
    FOR,
    WHILE,
    BREAK,
    NULL,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Box<dyn Any>,
    pub line: i32
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Self {
        Self {
            token_type,
            lexeme: lexeme.clone(),
            literal: Box::new(lexeme.clone()),
            line
        }
    }
    
    pub fn new_with_literal(token_type: TokenType, lexeme: String,
        literal: Box<dyn Any>, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
        && self.lexeme == other.lexeme
        && self.line == other.line
        && (
            // 字面量只有两种情况：String和f64
            if let Some(literal) = self.literal.downcast_ref::<String>() {
                if let Some(other_literal) = other.literal.downcast_ref::<String>() {
                    literal == other_literal
                } else {
                    false
                }
            } else if let Some(literal) = self.literal.downcast_ref::<f64>() {
                if let Some(other_literal) = other.literal.downcast_ref::<f64>() {
                    literal == other_literal
                } else {
                    false
                }
            } else {
                false
            }
        )
    }
}


// pub static KEYWORDS: HashMap<&str, self::Token> = HashMap::from([
//     ("fn", Token::FUNCTION),   
//     ("let", Token::LET),   
//     ("if", Token::IF),   
//     ("else", Token::ELSE),   
//     ("return", Token::RETURN),   
//     ("true", Token::TRUE),   
//     ("false", Token::FALSE),   
// ]);


pub fn loopkup_ident(token: &mut String, line: i32) -> Token {
    let keywords = HashMap::from([
        ("fn", TokenType::FUNCTION),   
        ("class", TokenType::CLASS),   
        ("let", TokenType::LET),   
        ("if", TokenType::IF),   
        ("else", TokenType::ELSE),   
        ("return", TokenType::RETURN),   
        ("true", TokenType::TRUE),   
        ("false", TokenType::FALSE),
        ("and", TokenType::AND),
        ("or", TokenType::OR),
        ("for", TokenType::FOR),
        ("while", TokenType::WHILE),
        ("break", TokenType::BREAK),
        ("null", TokenType::NULL),
    ]);
    match keywords.get(token.as_str()) {
        Some(&tok) => {
            // 关键字
            Token::new(tok, tok.to_string(), line)
        }
        None => {
            // 标识符
            Token::new(TokenType::IDENT, token.to_string(), line)     
        }
    }
}
