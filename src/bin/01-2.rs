//use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io;
use std::collections::HashSet;
use std::vec::Vec;


fn main() -> io::Result<()> {
    let f = File::open("01-1.input")?;
    let reader = BufReader::new(f);    

    let mut numbers = Vec::new();

    for line in reader.lines() {
        let num = i32::from_str_radix(&line?, 10).unwrap();
        numbers.push(num);
    }

    let mut n : i32 = 0;
    let mut seen = HashSet::new();

    seen.insert(0);

    'num: loop {
        for num in &numbers {
            n += num;
            if seen.contains( &n ) {
                println!("First sequence {}", n);
                break 'num;
            }
            seen.insert(n.clone());
        }
    }

    Ok(())
}
