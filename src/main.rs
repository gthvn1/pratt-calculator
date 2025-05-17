use pratt_calculator::interpreter::{
    lexer::Lexer,
    parser::{EvalOptionParser, Parser},
};
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
                match Parser::from_lexer(lexer).eval() {
                    None => println!("....failed"),
                    Some(x) => println!("....{}", x),
                }
            }
            Err(e) => eprintln!("failed to read line: {}", e),
        }
    }
}
