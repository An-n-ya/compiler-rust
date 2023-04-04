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
    current_token: Token
}

impl<'a> Parser<'a> {
    // 这里不要直接传Lexer，而是传一个字符串，然后在构造函数里面创建Lexer
    // 如果直接传Lexer会有生命周期问题
    pub fn new(lexer: &'a str) -> Self {
        Self {
            lexer: Lexer::new(lexer).peekable(),
            // 初始化current_token为ILLEGAL，这样在parse的时候就不用检查lexer是否为空了
            current_token: Token::new(TokenType::ILLEGAL, "".to_string(), 0),
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
                self.current_token = token.clone();
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
            let right = self.comparison();
            expr = Box::new(Binary::new(expr, self.current_token.clone(), right));
        }

        expr
    }
    
    fn comparison(&mut self) -> Box<dyn Expr<String>> {
        let mut expr = self.addition();
        
        while self.match_token(TokenType::GT) || self.match_token(TokenType::LT)
            || self.match_token(TokenType::GE) || self.match_token(TokenType::LE) {
            let right = self.addition();
            expr = Box::new(Binary::new(expr, self.current_token.clone(), right));
        }

        expr        
    }
    
    fn addition(&mut self) -> Box<dyn Expr<String>> {
        let mut expr = self.multiplication();
        
        while self.match_token(TokenType::MINUS) || self.match_token(TokenType::PLUS) {
            let right = self.multiplication();
            expr = Box::new(Binary::new(expr, self.current_token.clone(), right));
        }

        expr
    }
    
    fn multiplication(&mut self) -> Box<dyn Expr<String>> {
        let mut expr = self.unary();

        while self.match_token(TokenType::ASTERISK) || self.match_token(TokenType::SLASH) {
            let right = self.unary();
            expr = Box::new(Binary::new(expr, self.current_token.clone(), right));
        }

        expr
    }
    
    fn unary(&mut self) -> Box<dyn Expr<String>> {
        if self.match_token(TokenType::MINUS) || self.match_token(TokenType::EXCLAMATION) {
            let right = self.unary();
            Box::new(Unary::new(self.current_token.clone(), right))
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
            Box::new(Literal::new(Box::new(self.current_token.lexeme.clone())))
        } else if self.match_token(TokenType::LPAREN) {
            let expr = self.expression();
            self.match_token(TokenType::RPAREN);
            Box::new(Grouping::new(expr))
        } else {
            Box::new(Literal::new(Box::new("ILLEGAL".to_string())))
        }
    }
}