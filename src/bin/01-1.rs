//use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let f = File::open("data/01-1.input")?;
    let reader = BufReader::new(f);    

    let mut n = 0;

    for line in reader.lines() {
        let num = i32::from_str_radix(&line?, 10).unwrap();
        n += num;
    }

    println!("Result: {}", n);
    Ok(())
}
