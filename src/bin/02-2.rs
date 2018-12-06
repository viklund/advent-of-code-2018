use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io;
use std::vec::Vec;

fn main() -> io::Result<()> {
    let f = File::open("data/02-1.input")?;
    let reader = BufReader::new(f);

    let mut ids : Vec<Vec<char>> = Vec::new();
    let mut lnum = 0;

    'line: for line in reader.lines() {
        lnum += 1;
        let mut chars = Vec::new();
        for c in line?.chars() {
            chars.push(c);
        }

        'id: for id in ids.clone() {
            let mut ndiff = 0;
            for ii in 0..chars.len() {
                if chars[ii] != id[ii] {
                    ndiff += 1;

                    if ndiff > 2 {
                        continue 'id;
                    }
                }
            }

            if ndiff == 1 {
                println!("Found it, at: {}", lnum);
                for ii in 0..chars.len() {
                    if chars[ii] == id[ii] {
                        print!("{}", chars[ii]);
                    }
                }
                println!();
                break 'line;
            }
        }

        ids.push( chars );
    }

    Ok(())
}
