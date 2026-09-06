use mschief::parser::Parser;

use std::io::{self, BufRead};

fn main() {
    let mut buffer = String::with_capacity(1_024);
    let mut handle = io::stdin().lock();

    while let Ok(n) = handle.read_line(&mut buffer)
        && n > 0
    {
        let ast = Parser::new(&buffer).parse();
        println!("{ast:?}");
        buffer.clear();
    }
}
