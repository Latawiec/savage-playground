use std::io::{BufRead, self};

fn main() {    
    let mut lines = io::stdin().lock().lines();

    while let Some(_read_bytes) = lines.next() {
        println!("{}", _read_bytes.unwrap());
    }
}
