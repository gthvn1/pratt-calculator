use pratt_calculator::interpreter::{
    lexer::{Lexer, Token},
    parser::Parser,
};
use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut input = String::new();

    println!("Starting REPL. Ctrl+D to quit");

    loop {
        print!(">> ");
        stdout.flush().unwrap();
        input.clear();

        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(n) => {
                println!("..reading {} bytes", n);
                println!("..lexer called");
                let tokens: Vec<Token> = Lexer::new(input.trim()).collect();

                println!("..done");
                for t in &tokens {
                    println!("{}", t);
                }

                println!("..parser called");
                let _parser = Parser::new(&tokens);
                println!("..done");
            }
            Err(e) => eprintln!("failed to read line: {}", e),
        }
    }
}
