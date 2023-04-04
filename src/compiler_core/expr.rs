use std::any::Any;

use super::token::Token;

pub(crate) trait Expr<T> {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

pub(crate) struct Binary<R> {
    pub left: Box<dyn Expr<R>>,
    pub op: Token,
    pub right: Box<dyn Expr<R>>,
}

pub(crate) struct Grouping<R> {
    pub expr: Box<dyn Expr<R>>
}

pub(crate) struct Literal {
    pub value: Box<dyn Any>
}

pub(crate) struct Unary<R> {
    pub op: Token,
    pub right: Box<dyn Expr<R>>
}

impl<R> Binary<R> {
    pub fn new(left: Box<dyn Expr<R>>, op: Token, right: Box<dyn Expr<R>>) -> Self {
        Self {
            left,
            op,
            right
        }
    }
}

impl<R> Grouping<R> {
    pub fn new(expr: Box<dyn Expr<R>>) -> Self {
        Self {
            expr
        }
    }
}

impl Literal {
    pub fn new(value: Box<dyn Any>) -> Self {
        Self {
            value
        }
    }
}

impl<R> Unary<R> {
    pub fn new(op: Token, right: Box<dyn Expr<R>>) -> Self {
        Self {
            op,
            right
        }
    }
}

impl<R> Expr<R> for Binary<R> {
    fn accept(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_binary(self)
    }
}
impl<R> Expr<R> for Grouping<R> {
    fn accept(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_grouping(self)
    }
}
impl<R> Expr<R> for Literal {
    fn accept(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_literal(self)
    }
}
impl<R> Expr<R> for Unary<R> {
    fn accept(&self, visitor: &mut dyn Visitor<R>) -> R {
        visitor.visit_unary(self)
    }
}

pub(crate) trait Visitor<T> {
    fn visit_binary(&mut self, t: &Binary<T>) -> T;    
    fn visit_grouping(&mut self, t: &Grouping<T>) -> T;    
    fn visit_literal(&mut self, t: &Literal) -> T;    
    fn visit_unary(&mut self, t: &Unary<T>) -> T;    
}