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
                println!("..read {} bytes", n);

                println!("..generate tokens");
                let lexer = Lexer::from(input.trim());
                lexer.clone().for_each(|token| println!("{}", token));

                println!("..evaluate");
                let parser = match Parser::from_lexer(lexer) {
                    Ok(p) => p,
                    Err(s) => {
                        eprintln!("{}", s);
                        continue;
                    }
                };

                println!("{}", parser);
                let res = parser.eval();
                println!("..result: {}", res);
            }
            Err(e) => eprintln!("failed to read line: {}", e),
        }
    }
}
