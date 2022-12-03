use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use lazy_static::lazy_static;
use regex::Regex;
use std::env;

fn main() -> io::Result<()> {
    if env::args().len() < 2 {
        //handle pipeline i.e. echo data.txt | getip
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(len) => if len == 0 {} else {
                    // println!("{}", input);
                    get_ip(&input)
                }
                Err(error) => {
                    eprintln!("error: {}", error);
                }
            }
        }
    } else {
        //files as cmd arguments
        for (i, argument) in env::args().enumerate() {
            if i == 0 { continue; }
            // println!("{}: {}", i, argument);
            let file = File::open(argument)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                get_ip(&line.unwrap())
            }
        }
        Ok(())
    }
}

lazy_static! {
        static ref RE: Regex = Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}").unwrap();
    }

fn get_ip(line: &str) {
    for cap in RE.captures_iter(line) {
        println!("{}", &cap[0]);
    }
}

