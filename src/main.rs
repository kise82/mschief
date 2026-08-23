use mschief::lexer::Lexer;

use std::io::{self, BufRead};

fn main() {
    let mut buffer = String::with_capacity(1_024);
    let mut handle = io::stdin().lock();

    while let Ok(n) = handle.read_line(&mut buffer)
        && n > 0
    {
        let lexer = Lexer::new(&buffer);
        for token in lexer {
            print!("{token:?} ");
        }
        buffer.clear();
    }
}
