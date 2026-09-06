use mschief::lexer::Lexer;

use std::{
    env,
    fs::File,
    io::{self, BufRead, Read},
    process,
};

fn main() {
    let file = env::args().nth(1);
    let mut buffer = String::with_capacity(8_192);

    if let Some(file) = file {
        let Ok(mut file) = File::open(file) else {
            process::exit(1);
        };
        let Ok(_) = file.read_to_string(&mut buffer) else {
            process::exit(1);
        };

        let lexer = Lexer::new(&buffer);
        for token in lexer {
            print!("{token:?} ");
        }
    } else {
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
