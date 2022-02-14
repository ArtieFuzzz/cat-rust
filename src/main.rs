use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let raw_arg = env::args().nth(1);
    let empty = String::from("");
    let arg = raw_arg.unwrap_or(empty);
    let f = File::open(&arg).expect("Please provide a path to the file!");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", &line.unwrap());
    }
}
