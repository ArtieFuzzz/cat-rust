use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_arg = env::args().nth(1);
    let arg = raw_arg.unwrap_or_default();

    if arg.is_empty() {
        println!("No path provided.");
        exit(0);
    }

    let f = match File::open(&arg) {
        Ok(f) => f,
        Err(_) => {
            println!("Could not find specified file.");
            exit(0)
        }
    };

    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", &line?);
    }

    Ok(())
}
