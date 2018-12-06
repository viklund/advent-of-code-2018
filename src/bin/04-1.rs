extern crate regex;
extern crate chrono;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io;
use std::string::String;
use std::vec::Vec;
use regex::Regex;
use chrono::NaiveDateTime;

//  [1518-05-12 00:39] wakes up
fn main() -> io::Result<()> {
    let f = File::open("data/04.input")?;
    let reader = BufReader::new(f);

    let re = Regex::new(r"^\[(\d+-\d+-\d+ \d+:\d+)\]\s+(.*)$").unwrap();

    let mut events: Vec<(NaiveDateTime, std::string::String)> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let cap = re.captures(&l).unwrap();

        let time  = &cap[1];
        let event = &cap[2];

        let dt = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M").unwrap();

        // Seems unnecesary convoluted
        let mut e: String = String::new();
        e.push_str(event);

        let tupl = (dt, e);
        events.push( tupl );
    }
    events.sort();

    for ev in events {
        println!("D: {} {}", ev.0, ev.1);
    }

    Ok(())
}
