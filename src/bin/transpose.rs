use clap::Parser;
use std::io;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[clap(version, about = "split & transpose")]
struct Args {
    #[clap(short, long, default_value = "\t")]
    delimiter: String,
}

fn main() {
    let args = Args::parse();

    let mut lines: Vec<Vec<String>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let split_line = line.split(&args.delimiter).map(|s| s.to_string()).collect();
        lines.push(split_line);
    }

    for x in 0..lines[0].len() {
        let mut line: Vec<&str> = Vec::new();
        for y in 0..lines.len() {
            line.push(&lines[y][x]);
        }
        println!("{}", line.join(&args.delimiter));
    }
}
