use mschief::{driver::Driver, parser::Parser};

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

        let driver = Driver::new();
        let ast = Parser::new(&driver, &file).parse();
        println!("{ast:?}")
    } else {
        let mut buffer = String::with_capacity(8_192);
        let mut handle = io::stdin().lock();

        while let Ok(n) = handle.read_line(&mut buffer)
            && n > 0
        {
            let driver = Driver::new();
            {
                let ast = Parser::new(&driver, &buffer).parse();
                println!("{ast:?}");
            }

            buffer.clear();
        }
    }
}
