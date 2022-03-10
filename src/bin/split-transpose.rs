use clap::Parser;
use std::io;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[clap(version, about = "split & transpose")]
struct Args {
    #[clap(short, long)]
    lines: usize,

    #[clap(short, long, default_value = "\t")]
    delimiter: String,
}

fn main() {
    let args = Args::parse();

    let mut lines: Vec<String> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        lines.push(line);

        if lines.len() == args.lines {
            println!("{}", lines.join(&args.delimiter));
            lines.clear();
        }
    }
}
