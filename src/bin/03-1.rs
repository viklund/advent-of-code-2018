extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::io;
use regex::Regex;

fn main() -> io::Result<()> {
    let f = File::open("data/03.input")?;
    let reader = BufReader::new(f);

    let mut cloth: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    let re = Regex::new(r"^#\d+\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();

    for line in reader.lines() {
        let l = line.unwrap();
        let cap = re.captures(&l).unwrap();

        let x = i32::from_str_radix(&cap[1], 10).unwrap();
        let y = i32::from_str_radix(&cap[2], 10).unwrap();
        let w = i32::from_str_radix(&cap[3], 10).unwrap();
        let h = i32::from_str_radix(&cap[4], 10).unwrap();
        //println!(">>> <{}> <{}> -> <{}> <{}>", x,y,w,h);

        for ii in x..x+w {
            let row = cloth.entry(ii).or_insert(HashMap::new());
            for jj in y..y+h {
                let counter = row.entry(jj).or_insert(0);
                *counter += 1;
            }
        }
    }


    let mut num = 0;

    for v in cloth.values() {
        for v2 in v.values() {
            if v2 > &1 {
                num += 1;
            }
        }
    }

    println!("{}", num);
    Ok(())
}
