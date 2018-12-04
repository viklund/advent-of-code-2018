use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let f = File::open("data/02-1.input")?;
    let reader = BufReader::new(f);    

    let mut three = 0;
    let mut two   = 0;

    for line in reader.lines() {
        let mut counts = HashMap::new();

        for c in line?.chars() {
            let counter = counts.entry(c).or_insert(0);
            *counter += 1;
        }
        let mut ftwo = false;
        let mut fthree = false;
        for (_k,v) in counts.iter() {
            if v == &2 && !ftwo {
                two += 1;
                ftwo = true;
            }
            else if v == &3 && !fthree {
                three += 1;
                fthree = true;
            }
        }
    }
    println!("{} * {} = {}", two, three, two*three);

    Ok(())
}
