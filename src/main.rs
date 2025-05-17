use pratt_calculator::interpreter::{lexer::Lexer, parser::Parser};
use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut input = String::new();

    println!("Start of the REPL... Ctrl+D to quit");

    loop {
        print!(">> ");
        stdout.flush().unwrap();
        input.clear();

        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(n) => {
                println!("..reading {} bytes", n);
                println!("..lexer called");
                let lexer = Lexer::from(input.trim());

                // print tokens to check that everything is ok. We need to clone
                // it because we still need lexer for the parser.
                lexer.clone().for_each(|token| println!("{}", token));

                println!("..parser called");
                let _parser = Parser::from_lexer(lexer);
            }
            Err(e) => eprintln!("failed to read line: {}", e),
        }
    }
}
