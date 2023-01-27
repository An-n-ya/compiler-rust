use std::iter::Peekable;
use std::str::Chars;
use super::token::Token;
use super::token::loopkup_ident;

pub struct Lexer<'a> {
    expr: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Lexer { expr: new_expr.chars().peekable() }
    }
    
    fn skip_whitespace(&mut self) {
        let whitespace = " \t\n\r";
        loop {
            // 一直找空格
            match self.expr.peek() {
                None => return,
                Some(n) => {
                    if whitespace.contains(*n) {
                        self.expr.next();
                    } else {
                        return;
                    }
                }
            }
            
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    
    // 跳过空格
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        
        let next_char = self.expr.next();
        
        match next_char {
            Some('0'..='9') => {
                // 处理数字的情况
                let mut number = next_char?.to_string();
                
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else {
                        break;
                    }
                }
                // 返回数字
                Some(Token::NUMBER(number.parse::<f64>().unwrap()))
            },
            Some('+') => Some(Token::PLUS),
            Some('-') => Some(Token::MINUS),
            Some('*') => Some(Token::ASTERISK),
            Some('/') => Some(Token::SLASH),
            Some('^') => Some(Token::CARET),
            Some('(') => Some(Token::LPAREN),
            Some(')') => Some(Token::RPAREN),
            Some('{') => Some(Token::LBRACE),
            Some('}') => Some(Token::RBRACE),
            Some('[') => Some(Token::LBRACKET),
            Some(']') => Some(Token::RBRACKET),
            Some(':') => Some(Token::COLON),
            Some(',') => Some(Token::COMMA),
            Some(';') => Some(Token::SEMICOLON),
            Some('>') => Some(Token::GT),
            Some('<') => Some(Token::LT),
            None => None, 
            Some('=') => {
                // 如果后面跟的是等号，就返回EQ 否则返回赋值号
                if let Some(next_char) = self.expr.peek() {
                    if next_char == &'=' {
                        // 往后读一位
                        self.expr.next();
                        return Some(Token::EQ) 
                    }
                }
                Some(Token::ASSIGN)
            },
            Some('!') => {
                // 如果后面跟的是等号，就返回NOT_EQ 否则返回感叹号
                if let Some(next_char) = self.expr.peek() {
                    if next_char == &'=' {
                        // 往后读一位
                        self.expr.next();
                        return Some(Token::NOT_EQ) 
                    }
                }
                Some(Token::EXCLAMATION)
            }
            Some(_) => {
                if next_char?.is_alphabetic() {
                    let mut identifier = next_char?.to_string();
                    while let Some(next_char) = self.expr.peek() {
                        if next_char.is_alphabetic() {
                            identifier.push(self.expr.next()?);
                        } else {
                            break;
                        }
                    }
                    // 如果是字符，先看下是不是关键字，如果不是关键字就当做Identifier
                    Some(loopkup_ident(&mut identifier))
                } else {
                    // 其他情况就返回非法token
                    Some(Token::ILLEGAL)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_test() {
        let mut lexer = Lexer::new("   1.1+2");
        let expects = [Token::NUMBER(1.1), Token::PLUS, Token::NUMBER(2.0)];
        for expect in expects {
            assert_eq!(lexer.next().unwrap(), expect);
        }
    }
    
    #[test]
    fn keywords_test() {
        let mut lexer = Lexer::new("true false if else return let fn");
        let expects = [
            Token::TRUE,
            Token::FALSE,
            Token::IF,
            Token::ELSE,
            Token::RETURN,
            Token::LET,
            Token::FUNCTION  
        ];
        for expect in expects {
            assert_eq!(lexer.next().unwrap(), expect);
        }
    }
    
    #[test]
    fn special_character_test() {
        let mut lexer = Lexer::new("=+(){}[],;*/!<>^");
        let expects = [
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::LBRACKET,
            Token::RBRACKET,
            Token::COMMA,
            Token::SEMICOLON,
            Token::ASTERISK,
            Token::SLASH,
            Token::EXCLAMATION,
            Token::LT,
            Token::GT,
            Token::CARET
        ];
        for expect in expects {
            assert_eq!(lexer.next().unwrap(), expect);
        }
        
    }
    
    #[test]
    fn real_case_test() {
        let mut lexer = Lexer::new("let five = 5;
        let ten = 10;
        let add = fn(x,y) {
            x + y;
        };

        let result = add(five, ten)
        
        if (ten != five) {
            return true;
        } else {
            return false;
        }");
        let expects = [
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::NUMBER(5.0),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::NUMBER(10.0),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::IF,
            Token::LPAREN,
            Token::IDENT("ten".to_string()),
            Token::NOT_EQ,
            Token::IDENT("five".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
        ];
        print!("{:?}", expects);
        for expect in expects.iter() {
            print!("{:?}", *expect);
            assert_eq!(lexer.next().unwrap(), *expect);
        }
    }
}