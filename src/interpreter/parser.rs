use super::lexer::{Lexer, Operator, Token};
use std::{fmt, iter::Peekable};

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

pub struct Parser {
    pub expression: Expr,
}

impl Parser {
    pub fn from_lexer(lexer: Lexer) -> Result<Self, String> {
        Self::gen_expression(lexer.peekable()).map(|expression| Self { expression })
    }

    fn gen_expression(mut iter: Peekable<Lexer>) -> Result<Expr, String> {
        match iter.next() {
            Some(Token::Integer(x)) => {
                // After an integer we are expecting an op or nothing
                match iter.peek() {
                    None => Ok(Expr::Atom(x)), // return its value
                    Some(Token::Integer(_)) => Err("operator is expected".to_string()),
                    Some(Token::Op(op)) => {
                        let lhs = Expr::Atom(x);
                        let op = *op;
                        let _ = iter.next();
                        let rhs = Self::gen_expression(iter)?;
                        Ok(Expr::Operation(Box::new(lhs), op, Box::new(rhs)))
                    }
                }
            }
            Some(Token::Op(_)) => Err("an atom is expected first".to_string()),
            None => Err("....WARNING: Nothing to parse".to_string()),
        }
    }

    pub fn eval(&self) -> i64 {
        Self::eval_expr(&self.expression)
    }

    fn eval_expr(e: &Expr) -> i64 {
        match e {
            Expr::Atom(x) => *x,
            Expr::Operation(lhs, Operator::Add, rhs) => Self::eval_expr(lhs) + Self::eval_expr(rhs),
            Expr::Operation(lhs, Operator::Sub, rhs) => Self::eval_expr(lhs) - Self::eval_expr(rhs),
            Expr::Operation(lhs, Operator::Mult, rhs) => {
                Self::eval_expr(lhs) * Self::eval_expr(rhs)
            }
            Expr::Operation(lhs, Operator::Div, rhs) => Self::eval_expr(lhs) / Self::eval_expr(rhs),
        }
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::lexer::Lexer;

    use super::*;

    #[test]
    fn print_simple_expr() {
        // 1 + 2
        // Start by creating 2 * 3
        let lhs = Box::new(Expr::Atom(1));
        let rhs = Box::new(Expr::Atom(2));
        let e = Expr::Operation(lhs, Operator::Add, rhs);

        // We are expecting to print 1 + 2 * 3 as (+ 1 (* 2 3))
        assert_eq!("(+ 1 2)", e.to_string());
    }

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

    #[test]
    fn parse_atom_42() {
        let l = Lexer::from("42");
        let p = Parser::from_lexer(l).unwrap();
        assert_eq!("42", p.to_string())
    }

    #[test]
    fn parse_atom_2() {
        let l = Lexer::from("2");
        let p = Parser::from_lexer(l).unwrap();
        assert_eq!("2", p.to_string())
    }

    #[test]
    fn parse_simple_add() {
        let l = Lexer::from("1 + 2");
        let _p = Parser::from_lexer(l);
        //assert_eq!("(+ 1 2)", p.to_string())
    }
}
