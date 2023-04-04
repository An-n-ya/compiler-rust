use std::iter::Peekable;
use std::str::Chars;
use super::token::TokenType;
use super::token::Token;
use super::token::loopkup_ident;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    expr: Peekable<Chars<'a>>,
    line: i32,
}


impl<'a> Lexer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Lexer { expr: new_expr.chars().peekable(), line: 1 }
    }

    fn expected(&mut self, expected: char) -> bool {
        match self.expr.peek() {
            Some(&actual) if actual == expected => {
                self.expr.next();
                true
            },
            _ => false
        }
    }

}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    
    // 跳过空格
    fn next(&mut self) -> Option<Self::Item> {
        
        let next_char = self.expr.next();
        
        match next_char {
            Some('+') => Some(Token::new(TokenType::PLUS, next_char?.to_string(), self.line)),
            Some('.') => Some(Token::new(TokenType::DOT, next_char?.to_string(), self.line)),
            Some('-') => Some(Token::new(TokenType::MINUS, next_char?.to_string(), self.line)),
            Some('*') => Some(Token::new(TokenType::ASTERISK, next_char?.to_string(), self.line)),
            Some('^') => Some(Token::new(TokenType::CARET, next_char?.to_string(), self.line)),
            Some('(') => Some(Token::new(TokenType::LPAREN, next_char?.to_string(), self.line)),
            Some(')') => Some(Token::new(TokenType::RPAREN, next_char?.to_string(), self.line)),
            Some('{') => Some(Token::new(TokenType::LBRACE, next_char?.to_string(), self.line)),
            Some('}') => Some(Token::new(TokenType::RBRACE, next_char?.to_string(), self.line)),
            Some('[') => Some(Token::new(TokenType::LBRACKET, next_char?.to_string(), self.line)),
            Some(']') => Some(Token::new(TokenType::RBRACKET, next_char?.to_string(), self.line)),
            Some(':') => Some(Token::new(TokenType::COLON, next_char?.to_string(), self.line)),
            Some(',') => Some(Token::new(TokenType::COMMA, next_char?.to_string(), self.line)),
            Some(';') => Some(Token::new(TokenType::SEMICOLON,next_char?.to_string(), self.line)),
            None => None, 
            Some('=') => {
                // 如果后面跟的是等号，就返回EQ 否则返回赋值号
                if self.expected('=') {
                    Some(Token::new(TokenType::EQ, "==".to_string(), self.line))
                } else {
                    Some(Token::new(TokenType::ASSIGN, "=".to_string(), self.line))
                }
            },
            Some('!') => {
                // 如果后面跟的是等号，就返回NOT_EQ 否则返回感叹号
                if self.expected('=') {
                    Some(Token::new(TokenType::NOT_EQ, "!=".to_string(), self.line))
                } else {
                    Some(Token::new(TokenType::EXCLAMATION, "!".to_string(), self.line))
                }
            },
            Some('<') => {
                if self.expected('=') {
                    Some(Token::new(TokenType::LE, "<=".to_string(), self.line))                    
                } else {
                    Some(Token::new(TokenType::LT, "<".to_string(), self.line))
                }
            },
            Some('>') => {
                if self.expected('=') {
                    Some(Token::new(TokenType::GE, ">=".to_string(), self.line))
                } else {
                    Some(Token::new(TokenType::GT, ">".to_string(), self.line))
                }
            },
            Some('/') => {
                if self.expected('/') {
                    // 如果是注释，就一直读到换行符
                    while let Some(&next_char) = self.expr.peek() {
                        if next_char != '\n' {
                            self.expr.next();
                        } else {
                            break;
                        }
                    };
                    // 返回下一个token
                    self.next()
                } else {
                    Some(Token::new(TokenType::SLASH, "/".to_string(), self.line))
                }
            },
            Some('\r') => {
                // 如果是空格，就跳过
                self.next()
            },
            Some('\t') => {
                // 如果是空格，就跳过
                self.next()
            },
            Some(' ') => {
                // 如果是空格，就跳过
                self.next()
            },
            Some('\n') => {
                // 如果是换行符，line加1，返回下一个token
                self.line += 1;
                self.next()
            },
            Some('"') => {
                // 如果是双引号，就一直读到下一个双引号
                let mut string = String::new();
                while let Some(&next_char) = self.expr.peek() {
                    if next_char != '"' {
                        if next_char == '\n' {
                            // 遇到换行符，line加1
                            self.line += 1;
                        }
                        string.push(self.expr.next()?);
                    } else {
                        break;
                    }
                }
                // 跳过下一个双引号
                self.expr.next();
                Some(Token::new(
                        TokenType::STRING,
                        string.clone(),
                        self.line))
            },
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
                
                // 如果这个非数字字符是小数点，就继续读数字
                if self.expr.peek() == Some(&'.') {
                    number.push(self.expr.next()?);
                    while let Some(next_char) = self.expr.peek() {
                        if next_char.is_numeric() {
                            number.push(self.expr.next()?);
                        } else {
                            break;
                        }
                    }
                }


                // 返回数字
                Some(Token::new(
                        TokenType::NUMBER,
                        number.clone(),
                        self.line))
            },
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
                    Some(loopkup_ident(&mut identifier, self.line))
                } else {
                    // 其他情况就返回非法token
                    Some(Token::new(TokenType::ILLEGAL, next_char?.to_string(), self.line))
                }
            }
        }
       
    }
}


// pub struct CloneableLexer<'a>(Rc<RefCell<Lexer + 'a>>);

// impl<'a> Iterator for CloneableLexer<'a> {
//     type Item = Token;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn keywords_test() {
        let mut lexer = Lexer::new("true false if else return let fn");
        let expects = [
            Token::new(TokenType::TRUE, "TRUE".to_string(), 1),
            Token::new(TokenType::FALSE, "FALSE".to_string(), 1),
            Token::new(TokenType::IF, "IF".to_string(), 1),
            Token::new(TokenType::ELSE, "ELSE".to_string(), 1),
            Token::new(TokenType::RETURN, "RETURN".to_string(), 1),
            Token::new(TokenType::LET, "LET".to_string(), 1),
            Token::new(TokenType::FUNCTION, "FUNCTION".to_string(), 1)

        ];
        for expect in expects {
            assert_eq!(lexer.next().unwrap(), expect);
        }
    }
    
    #[test]
    fn special_character_test() {
        let mut lexer = Lexer::new("=+(){}[],;*/!<>^>= <= == !=");
        let expects = [
            Token::new(TokenType::ASSIGN, "=".to_string(), 1),
            Token::new(TokenType::PLUS, "+".to_string(), 1),
            Token::new(TokenType::LPAREN, "(".to_string(), 1),
            Token::new(TokenType::RPAREN, ")".to_string(), 1),
            Token::new(TokenType::LBRACE, "{".to_string(), 1),
            Token::new(TokenType::RBRACE, "}".to_string(), 1),
            Token::new(TokenType::LBRACKET, "[".to_string(), 1),
            Token::new(TokenType::RBRACKET, "]".to_string(), 1),
            Token::new(TokenType::COMMA, ",".to_string(), 1),
            Token::new(TokenType::SEMICOLON, ";".to_string(), 1),
            Token::new(TokenType::ASTERISK, "*".to_string(), 1),
            Token::new(TokenType::SLASH, "/".to_string(), 1),
            Token::new(TokenType::EXCLAMATION, "!".to_string(), 1),
            Token::new(TokenType::LT, "<".to_string(), 1),
            Token::new(TokenType::GT, ">".to_string(), 1),
            Token::new(TokenType::CARET, "^".to_string(), 1),
            Token::new(TokenType::GE, ">=".to_string(), 1),
            Token::new(TokenType::LE, "<=".to_string(), 1),
            Token::new(TokenType::EQ, "==".to_string(), 1),
            Token::new(TokenType::NOT_EQ, "!=".to_string(), 1),
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

        if (ten != five) {
            return true;
        } else {
            return false;
        }");
        let expects = [
            Token::new(TokenType::LET, "LET".to_string(), 1),
            Token::new(TokenType::IDENT, "five".to_string(), 1),
            Token::new(TokenType::ASSIGN, "=".to_string(), 1),
            Token::new(TokenType::NUMBER, "5".to_string(), 1),
            Token::new(TokenType::SEMICOLON, ";".to_string(), 1),
            Token::new(TokenType::LET, "LET".to_string(), 2),
            Token::new(TokenType::IDENT, "ten".to_string(), 2),
            Token::new(TokenType::ASSIGN, "=".to_string(), 2),
            Token::new(TokenType::NUMBER, "10".to_string(), 2),
            Token::new(TokenType::SEMICOLON, ";".to_string(), 2),
            Token::new(TokenType::LET, "LET".to_string(), 3),
            Token::new(TokenType::IDENT, "add".to_string(), 3),
            Token::new(TokenType::ASSIGN, "=".to_string(), 3),
            Token::new(TokenType::FUNCTION, "FUNCTION".to_string(), 3),
            Token::new(TokenType::LPAREN, "(".to_string(), 3),
            Token::new(TokenType::IDENT, "x".to_string(), 3),
            Token::new(TokenType::COMMA, ",".to_string(), 3),
            Token::new(TokenType::IDENT, "y".to_string(), 3),
            Token::new(TokenType::RPAREN, ")".to_string(), 3),
            Token::new(TokenType::LBRACE, "{".to_string(), 3),
            Token::new(TokenType::IDENT, "x".to_string(), 4),
            Token::new(TokenType::PLUS, "+".to_string(), 4),
            Token::new(TokenType::IDENT, "y".to_string(), 4),
            Token::new(TokenType::SEMICOLON, ";".to_string(), 4),
            Token::new(TokenType::RBRACE, "}".to_string(), 5),
            Token::new(TokenType::SEMICOLON, ";".to_string(), 5),

            Token::new(TokenType::IF, "IF".to_string(), 7),
            Token::new(TokenType::LPAREN, "(".to_string(), 7),
            Token::new(TokenType::IDENT, "ten".to_string(), 7),
            Token::new(TokenType::NOT_EQ, "!=".to_string(), 7),
            Token::new(TokenType::IDENT, "five".to_string(), 7),
            Token::new(TokenType::RPAREN, ")".to_string(), 7),
            Token::new(TokenType::LBRACE, "{".to_string(), 7),
            Token::new(TokenType::RETURN, "RETURN".to_string(), 8),
            Token::new(TokenType::TRUE, "TRUE".to_string(), 8),
            Token::new(TokenType::SEMICOLON, ";".to_string(), 8),
            Token::new(TokenType::RBRACE, "}".to_string(), 9),
            Token::new(TokenType::ELSE, "ELSE".to_string(), 9),
            Token::new(TokenType::LBRACE, "{".to_string(), 9),
            Token::new(TokenType::RETURN, "RETURN".to_string(), 10),
            Token::new(TokenType::FALSE, "FALSE".to_string(), 10),
            Token::new(TokenType::SEMICOLON, ";".to_string(), 10),
            Token::new(TokenType::RBRACE, "}".to_string(), 11),
        ];
        print!("{:?}", expects);
        for expect in expects.iter() {
            print!("{:?}", *expect);
            assert_eq!(lexer.next().unwrap(), *expect);
        }
    }
}