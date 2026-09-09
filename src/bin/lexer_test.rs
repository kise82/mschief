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

        let lexer = Lexer::new(&file);
        for token in lexer {
            print!("{token:?} ");
        }
    } else {
        let mut buffer = String::with_capacity(8_192);
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
}
