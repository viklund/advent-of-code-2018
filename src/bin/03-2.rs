extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::io;
use std::vec::Vec;
use regex::Regex;

fn main() -> io::Result<()> {
    let f = File::open("data/03.input")?;
    let reader = BufReader::new(f);

    let mut cloth: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    let re = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();

    let mut patches: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let cap = re.captures(&l).unwrap();

        let id = i32::from_str_radix(&cap[1], 10).unwrap();
        let x = i32::from_str_radix(&cap[2], 10).unwrap();
        let y = i32::from_str_radix(&cap[3], 10).unwrap();
        let w = i32::from_str_radix(&cap[4], 10).unwrap();
        let h = i32::from_str_radix(&cap[5], 10).unwrap();

        patches.push( vec![id, x,y,w,h] );

        for ii in x..x+w {
            let row = cloth.entry(ii).or_insert(HashMap::new());
            for jj in y..y+h {
                let counter = row.entry(jj).or_insert(0);
                *counter += 1;
            }
        }
    }


    'patch: for patch in patches {
        let id = patch[0];
        let x  = patch[1];
        let y  = patch[2];
        let w  = patch[3];
        let h  = patch[4];

        for ii in x..x+w {
            let row = cloth.entry(ii).or_insert(HashMap::new());
            for jj in y..y+h {
                let counter = row.entry(jj).or_insert(0);
                if *counter > 1 {
                    continue 'patch;
                }
            }
        }

        println!("Found it: {}", id);
    }

    Ok(())
}
