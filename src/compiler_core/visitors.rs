use super::expr::Visitor;
use super::expr::Expr;
use super::expr::Binary;
use super::expr::Unary;
use super::expr::Literal;
use super::expr::Grouping;
use super::token::Token;
use super::token::TokenType;

pub struct AstPrinter { }

impl Visitor<String> for AstPrinter {

    fn visit_binary(&mut self, expr: &Binary<String>) -> String {
        self.parenthesize(format!("{}" , expr.op.lexeme.clone()), vec![expr.left.as_ref(), expr.right.as_ref()])
    }

    fn visit_grouping(&mut self, expr: &Grouping<String>) -> String {
        self.parenthesize("group".to_string(), vec![expr.expr.as_ref()])
    }

    fn visit_literal(&mut self, expr: &Literal) -> String {
        if let Some(value) = expr.value.downcast_ref::<String>() {
            value.clone()
        } else if let Some(value) = expr.value.downcast_ref::<f64>() {
            value.to_string()
        } else {
            "null".to_string()
        }
    }

    fn visit_unary(&mut self, expr: &Unary<String>) -> String {
        self.parenthesize(expr.op.lexeme.clone(), vec![expr.right.as_ref()])
    }

}

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter { }
    }
    
    fn parenthesize(&mut self, name: String, exprs: Vec<&dyn Expr<String>>) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(&name);
        for expr in exprs {
            result.push_str(" ");
            result.push_str(&expr.accept( self));
        }
        result.push_str(")");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_printer() {
        let expr = Binary::new(
            Box::new(Unary::new(
                Token::new(TokenType::MINUS, "-".to_string(),  1),
                Box::new(Literal::new(Box::new(123.0)))
            )),
            Token::new(TokenType::ASTERISK, "*".to_string(), 1),
            Box::new(Grouping::new(
                Box::new(Literal::new(Box::new(45.67)))
            ))
        );
        let mut printer = AstPrinter::new();
        let result = expr.accept(&mut printer);
        println!("{}", result);
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
    
}

