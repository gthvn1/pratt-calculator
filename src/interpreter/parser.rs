use super::lexer::{Lexer, Operator, Token};
use std::{fmt, iter::Peekable};

pub enum Expr {
    Atom(f64),
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
        Self::gen_expression(&mut lexer.peekable(), 0).map(|expression| Self { expression })
    }

    //  1    *   ( 2 + 3)
    // LHS
    //      OP
    //           ^   -> gen_expression 2 + 3
    //             ^ + : if + has higher precedence we continue and LHS = 1
    //                   if not then LHS == (1 * 2)
    //
    fn gen_expression(iter: &mut Peekable<Lexer>, precedence: u8) -> Result<Expr, String> {
        let mut lhs = match iter.next() {
            Some(Token::Number(x)) => Expr::Atom(x),
            Some(Token::Op(_)) => return Err("....Err: an atom is expected".to_string()),
            Some(Token::LeftParen) => {
                let lhs = Self::gen_expression(iter, 0);
                // If we returned from gen_expression we should have a RightParen
                let t = iter
                    .next()
                    .ok_or("....Err: right parenthesis expected,found nothing".to_string())?;
                if t != Token::RightParen {
                    return Err("...Err: Right parenthesis is expected".to_string());
                }
                lhs?
            }
            Some(Token::RightParen) => {
                return Err("....Err: right parenthesis not expected".to_string())
            }
            None => return Err("....Warn: Nothing to parse".to_string()),
        };

        loop {
            let op = match iter.peek() {
                Some(Token::Op(op)) => *op,
                Some(Token::RightParen) => break,
                None => return Ok(lhs), // End of expression
                _ => return Err("....Err: an operation is expected".to_string()),
            };

            let op_precedence = op.precedence();
            if op_precedence < precedence {
                // We can return lhs because the current precedence is lower than
                // the previously found.
                break;
            }
            // Else if the precedence is higher we just need to continue the iteration
            iter.next(); // consume the operator

            // We add one to the precedence in case of equality. It allows
            // to always choose the same side and be deterministic.
            let rhs = Self::gen_expression(iter, op_precedence + 1)?;
            lhs = Expr::Operation(Box::new(lhs), op, Box::new(rhs));
        }

        Ok(lhs)
    }

    pub fn eval(&self) -> f64 {
        Self::eval_expr(&self.expression)
    }

    fn eval_expr(e: &Expr) -> f64 {
        match e {
            Expr::Atom(x) => *x,
            Expr::Operation(lhs, Operator::Add, rhs) => Self::eval_expr(lhs) + Self::eval_expr(rhs),
            Expr::Operation(lhs, Operator::Sub, rhs) => Self::eval_expr(lhs) - Self::eval_expr(rhs),
            Expr::Operation(lhs, Operator::Div, rhs) => Self::eval_expr(lhs) / Self::eval_expr(rhs),
            Expr::Operation(lhs, Operator::Mul, rhs) => Self::eval_expr(lhs) * Self::eval_expr(rhs),
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
        let lhs = Box::new(Expr::Atom(1.0));
        let rhs = Box::new(Expr::Atom(2.0));
        let e = Expr::Operation(lhs, Operator::Add, rhs);

        // We are expecting to print 1 + 2 * 3 as (+ 1 (* 2 3))
        assert_eq!("(+ 1 2)", e.to_string());
    }

    #[test]
    fn print_expr() {
        // 1 + 2 * 3
        // Start by creating 2 * 3
        let lhs = Box::new(Expr::Atom(2.0));
        let rhs = Box::new(Expr::Atom(3.14));
        let e = Expr::Operation(lhs, Operator::Mul, rhs);

        // And now create 1 + e
        let lhs = Box::new(Expr::Atom(1.0));
        let rhs = Box::new(e);
        let e = Expr::Operation(lhs, Operator::Add, rhs);

        // We are expecting to print 1 + 2 * 3 as (+ 1 (* 2 3))
        assert_eq!("(+ 1 (* 2 3.14))", e.to_string());
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
        let p = Parser::from_lexer(l).unwrap();
        assert_eq!("(+ 1 2)", p.to_string())
    }

    #[test]
    fn parse_expression() {
        let l = Lexer::from("1 + 2 * 3");
        let p = Parser::from_lexer(l).unwrap();
        assert_eq!("(+ 1 (* 2 3))", p.to_string())
    }

    #[test]
    fn parse_expression_with_prio() {
        let l = Lexer::from("1.0 * 2.1 + 3");
        let p = Parser::from_lexer(l).unwrap();
        assert_eq!("(+ (* 1 2.1) 3)", p.to_string())
    }

    #[test]
    fn eval_expression1() {
        let l = Lexer::from("1 * 2 + 3");
        let p = Parser::from_lexer(l).unwrap();
        assert_eq!(5.0, p.eval())
    }

    #[test]
    fn eval_expression2() {
        let l = Lexer::from("1 + 2 * 3");
        let p = Parser::from_lexer(l).unwrap();
        assert_eq!(7.0, p.eval())
    }

    #[test]
    fn eval_expression_paren1() {
        let l = Lexer::from("(1 + 2) * 3");
        let p = Parser::from_lexer(l).unwrap();
        assert_eq!(9.0, p.eval())
    }
}
