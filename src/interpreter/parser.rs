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

// As our parser returns an Option and we want to be able to call eval on it we
// need to implement eval for Option<Parser>.
pub trait EvalOptionParser {
    fn eval(self) -> Option<i64>;
}

impl EvalOptionParser for Option<Parser> {
    fn eval(self) -> Option<i64> {
        self.map(|p| p.eval_expr())
    }
}

pub struct Parser {
    pub expression: Expr,
}

impl Parser {
    pub fn from_lexer(lexer: Lexer) -> Option<Self> {
        Self::gen_expression(lexer.peekable()).map(|expression| Self { expression })
    }

    fn gen_expression(mut iter: Peekable<Lexer>) -> Option<Expr> {
        match iter.next() {
            Some(Token::Integer(x)) => {
                // After an integer we are expecting an op or nothing
                match iter.peek() {
                    None => Some(Expr::Atom(x)), // return its value
                    Some(Token::Integer(_)) => {
                        eprintln!("....ERROR: An operator is expected");
                        None
                    }
                    Some(Token::Op(_)) => {
                        eprintln!("....TODO: parse op after reading an atom");
                        None
                    }
                }
            }
            Some(Token::Op(_)) => {
                eprintln!("....ERROR: An atom is expected first");
                None
            }
            None => {
                eprintln!("....WARNING: Nothing to parse");
                None
            }
        }
    }

    pub fn eval_expr(&self) -> i64 {
        // TODO: real eval
        42
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
