// We start with a simple calculator:
// It accepts integer and four operators '+', '-', '/' and '*'

use std::convert::From;
use std::fmt::Display;

pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mult => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

pub enum Token {
    Integer(i64),
    Op(Operator),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "....S: Integer: {}", i),
            Token::Op(Operator::Add) => write!(f, "....S: Op: +"),
            Token::Op(Operator::Sub) => write!(f, "....S: Op: -"),
            Token::Op(Operator::Mult) => write!(f, "....S: Op: *"),
            Token::Op(Operator::Div) => write!(f, "....S: Op: /"),
        }
    }
}

#[derive(Clone)]
pub struct Lexer<'a> {
    iter: std::iter::Peekable<std::str::Chars<'a>>,
}

impl Lexer<'_> {
    fn read_integer(iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> i64 {
        let mut v = String::new();

        while let Some(&c) = iter.peek() {
            if c.is_ascii_digit() {
                let x = iter.next().unwrap();
                v.push(x);
            } else {
                break;
            }
        }

        v.parse::<i64>().unwrap()
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // We use the peek method to be able to read integer. So
        // don't forget to consume the character in each case if
        // needed.
        while let Some(&c) = self.iter.peek() {
            match c {
                '+' => {
                    self.iter.next();
                    return Some(Token::Op(Operator::Add));
                }
                '-' => {
                    self.iter.next();
                    return Some(Token::Op(Operator::Sub));
                }
                '*' => {
                    self.iter.next();
                    return Some(Token::Op(Operator::Mult));
                }
                '/' => {
                    self.iter.next();
                    return Some(Token::Op(Operator::Div));
                }
                '0'..='9' => {
                    // In this case character will be consumed by read_integer
                    return Some(Token::Integer(Self::read_integer(&mut self.iter)));
                }
                c if c.is_whitespace() => {
                    // Just skip it
                    let _ = self.iter.next();
                }
                _ => {
                    // Also skip unknown char but report an error
                    let _ = self.iter.next();
                    eprintln!(".... {} is unknown (skipped)", c);
                }
            }
        }

        None
    }
}
