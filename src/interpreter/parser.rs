use super::lexer::{Operator, Token};
use std::fmt;

pub enum Expr {
    Atom(i64),
    Operation(Box<Expr>, Operator, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(i) => write!(f, "{}", i),
            Expr::Operation(lhs, op, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
        }
    }
}

pub struct Parser {}

impl Parser {
    pub fn new(_toks: &[Token]) -> Self {
        println!("....not yet implemented...");
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_expr() {
        // 1 + 2 * 3
        // Start by creating 2 * 3
        let lhs = Box::new(Expr::Atom(2));
        let rhs = Box::new(Expr::Atom(3));
        let e = Expr::Operation(lhs, Operator::Mult, rhs);

        // And now create 1 + e
        let lhs = Box::new(Expr::Atom(1));
        let rhs = Box::new(e);
        let e = Expr::Operation(lhs, Operator::Add, rhs);

        // We are expecting to print 1 + 2 * 3 as (+ 1 (* 2 3))
        assert_eq!("(+ 1 (* 2 3))", e.to_string());
    }
}
