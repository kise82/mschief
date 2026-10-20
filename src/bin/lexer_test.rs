use mschief::lexer::Lexer;

use std::{
    env, fs,
    io::{self, BufRead},
    process,
};

fn main() {
    let file = env::args().nth(1);

    if let Some(file) = file {
        let Ok(file) = fs::read_to_string(file) else {
            process::exit(1);
        };

        debug_lexer(&file);
    } else {
        let mut buffer = String::with_capacity(8_192);
        let mut handle = io::stdin().lock();

        while let Ok(n) = handle.read_line(&mut buffer)
            && n > 0
        {
            debug_lexer(&buffer);
            buffer.clear();
        }
    }
}

fn debug_lexer(input: &str) {
    let mut lexer = Lexer::new(input);
    let mut line = lexer.line();
    while let Some(token) = lexer.next() {
        if line != lexer.line() {
            line = lexer.line();
            print!("\n{line:4}");
        }
        print!(" {token:?}");
    }
}
