// We start with a simple calculator:
// It accepts integer and four operators '+', '-', '/' and '*'

use std::convert::From;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Sub => 5,
            Operator::Mul | Operator::Div => 10,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

pub enum Token {
    Number(f64),
    Op(Operator),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(i) => write!(f, "....S: Number: {}", i),
            Token::Op(Operator::Add) => write!(f, "....S: Op: +"),
            Token::Op(Operator::Sub) => write!(f, "....S: Op: -"),
            Token::Op(Operator::Mul) => write!(f, "....S: Op: *"),
            Token::Op(Operator::Div) => write!(f, "....S: Op: /"),
        }
    }
}

#[derive(Clone)]
pub struct Lexer<'a> {
    iter: std::iter::Peekable<std::str::Chars<'a>>,
}

impl Lexer<'_> {
    fn read_digits(iter: &mut std::iter::Peekable<std::str::Chars<'_>>, buffer: &mut String) {
        while let Some(&c) = iter.peek() {
            if c.is_ascii_digit() {
                buffer.push(iter.next().unwrap());
            } else {
                break;
            }
        }
    }

    fn read_number(iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> f64 {
        let mut v = String::new();

        // Read real part
        Self::read_digits(iter, &mut v);

        // Check if there is a fraction
        if let Some(&c) = iter.peek() {
            if c == '.' {
                iter.next();
                v.push('.');
                Self::read_digits(iter, &mut v);
            }
        }

        v.parse::<f64>().unwrap()
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
                    return Some(Token::Op(Operator::Mul));
                }
                '/' => {
                    self.iter.next();
                    return Some(Token::Op(Operator::Div));
                }
                '0'..='9' => {
                    // In this case character will be consumed by read_number
                    return Some(Token::Number(Self::read_number(&mut self.iter)));
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
