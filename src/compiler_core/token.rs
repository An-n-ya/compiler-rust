use std::{collections::HashMap};

// lazy_static! {
//     pub static ref KEYWORDS: HashMap<&'static str, self::Token> = HashMap::from([
//         ("fn", Token::FUNCTION),   
//         ("let", Token::FUNCTION),   
//     ]);
// }

#[derive(Debug, PartialEq)]
#[allow(dead_code, non_camel_case_types)]
pub enum Token {
    

    ILLEGAL,
    EOF,
    IDENT(String),
    NUMBER(f64),
    STRING,
    ASSIGN,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EXCLAMATION,
    GT,
    LT,
    CARET,

    EQ,
    NOT_EQ,

    COMMA,
    SEMICOLON,
    COLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    
    // 关键字
    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
}

// FIXME: 这里的arm太多了，应该使用默认arm
impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::IDENT(s) => Self::IDENT(s.clone()),
            Self::NUMBER(s) => Self::NUMBER(s.clone()),
            Token::ILLEGAL => Token::ILLEGAL,
            Token::EOF => Token::EOF,
            Token::STRING => Token::STRING,
            Token::ASSIGN => Token::ASSIGN,
            Token::PLUS => Token::PLUS,
            Token::MINUS => Token::MINUS,
            Token::ASTERISK => Token::ASTERISK,
            Token::SLASH => Token::SLASH,
            Token::EXCLAMATION => Token::EXCLAMATION,
            Token::GT => Token::GT,
            Token::LT => Token::LT,
            Token::CARET => Token::CARET,
            Token::EQ => Token::EQ,
            Token::NOT_EQ => Token::NOT_EQ,
            Token::COMMA => Token::COMMA,
            Token::SEMICOLON => Token::SEMICOLON,
            Token::COLON => Token::COLON,
            Token::LPAREN => Token::LPAREN,
            Token::RPAREN => Token::RPAREN,
            Token::LBRACE => Token::LBRACE,
            Token::RBRACE => Token::RBRACE,
            Token::LBRACKET => Token::LBRACKET,
            Token::RBRACKET => Token::RBRACKET,
            Token::FUNCTION => Token::FUNCTION,
            Token::LET => Token::LET,
            Token::IF => Token::IF,
            Token::ELSE => Token::ELSE,
            Token::RETURN => Token::RETURN,
            Token::TRUE => Token::TRUE,
            Token::FALSE => Token::FALSE,
        }
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


pub fn loopkup_ident(token: &mut String) -> Token {
    let keywords = HashMap::from([
        ("fn", Token::FUNCTION),   
        ("let", Token::LET),   
        ("if", Token::IF),   
        ("else", Token::ELSE),   
        ("return", Token::RETURN),   
        ("true", Token::TRUE),   
        ("false", Token::FALSE),   
    ]);
    match keywords.get(token.as_str()) {
        Some(tok) => {
            tok.clone()   
        }
        None => {
            Token::IDENT(token.to_string())      
        }
    }
}
