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
                println!("..ECHO {} bytes <{}>", n, &input);
            }
            Err(e) => eprintln!("failed to read line: {}", e),
        }
    }
}
