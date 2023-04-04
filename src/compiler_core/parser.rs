use super::expr::Expr;
use super::expr::Binary;
use super::expr::Literal;
use super::expr::Unary;
use super::expr::Grouping;
use super::lexer::Lexer;
use super::token::TokenType;
use super::token::Token;
use std::any::Any;
use std::iter::Peekable;
use std::rc::Rc;
use std::sync::Arc;

pub struct Parser<'a>{
    lexer: Peekable<Lexer<'a>>,
    current_token: TokenType,
    current_line: i32,
    current_value: String
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a str) -> Self {
        Self {
            lexer: Lexer::new(lexer).peekable(),
            current_token: TokenType::EOF,
            current_line: 0,
            current_value: "none".to_string()
        }
    }
    
    pub fn parse(&mut self) -> Box<dyn Expr<String>> {
        if !self.lexer.peek().is_none() {
            return self.expression()
        }
        Box::new(Literal::new(Box::new("ILLEGAL".to_string())))
    }
    

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.lexer.peek() {
            if token.token_type == token_type {
                // 检查下一个token是否是token_type，如果是的话，就消费它并返回true，否则返回false
                self.current_token = token.token_type;
                self.current_line = token.line;
                self.current_value = token.lexeme.clone();
                self.lexer.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn expression(&mut self) -> Box<dyn Expr<String>> {
        self.equality()
    }
    
    fn equality(&mut self) -> Box<dyn Expr<String>> {
        let mut expr = self.comparison();
        
        while self.match_token(TokenType::NOT_EQ) || self.match_token(TokenType::EQ) {
            let op = Token::new(self.current_token, self.current_token.to_string(), self.current_line);
            let right = self.comparison();
            expr = Box::new(Binary::new(expr, op, right));
        }

        expr
    }
    
    fn comparison(&mut self) -> Box<dyn Expr<String>> {
        let mut expr = self.addition();
        
        while self.match_token(TokenType::GT) || self.match_token(TokenType::LT)
            || self.match_token(TokenType::GE) || self.match_token(TokenType::LE) {
            let op = Token::new(self.current_token, self.current_token.to_string(), self.current_line);
            let right = self.addition();
            expr = Box::new(Binary::new(expr, op, right));
        }

        expr        
    }
    
    fn addition(&mut self) -> Box<dyn Expr<String>> {
        let mut expr = self.multiplication();
        
        while self.match_token(TokenType::MINUS) || self.match_token(TokenType::PLUS) {
            let op = Token::new(self.current_token, self.current_token.to_string(), self.current_line);
            let right = self.multiplication();
            expr = Box::new(Binary::new(expr, op, right));
        }

        expr
    }
    
    fn multiplication(&mut self) -> Box<dyn Expr<String>> {
        let mut expr = self.unary();

        while self.match_token(TokenType::ASTERISK) || self.match_token(TokenType::SLASH) {
            let op = Token::new(self.current_token, self.current_token.to_string(), self.current_line);
            let right = self.unary();
            expr = Box::new(Binary::new(expr, op, right));
        }

        expr
    }
    
    fn unary(&mut self) -> Box<dyn Expr<String>> {
        if self.match_token(TokenType::MINUS) || self.match_token(TokenType::EXCLAMATION) {
            let op = Token::new(self.current_token, self.current_token.to_string(), self.current_line);
            let right = self.unary();
            Box::new(Unary::new(op, right))
        } else {
            self.primary()
        }
    }
    
    fn primary(&mut self) -> Box<dyn Expr<String>> {
        if self.match_token(TokenType::FALSE)
        || self.match_token(TokenType::TRUE)
        || self.match_token(TokenType::NULL)
        || self.match_token(TokenType::NUMBER)
        || self.match_token(TokenType::STRING) {
            Box::new(Literal::new(Box::new(self.current_value.clone())))
        } else if self.match_token(TokenType::LPAREN) {
            let expr = self.expression();
            self.match_token(TokenType::RPAREN);
            Box::new(Grouping::new(expr))
        } else {
            Box::new(Literal::new(Box::new("ILLEGAL".to_string())))
        }
    }
}