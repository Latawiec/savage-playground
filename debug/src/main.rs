use std::io::{self, Write, Read};

fn main() {
    println!("Hello! Let's go!\n");
    let mut input: String = Default::default();
    while let Ok(_read_bytes) = io::stdin().read_line(&mut input) {
        println!("Read: {}", _read_bytes);
        let _ = io::stdout().write_all(input.as_bytes());
        input.clear();
    }
}