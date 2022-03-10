use clap::Parser;
use std::io;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[clap(version, about = "split & paste")]
struct Args {
    #[clap(short, long)]
    lines: usize,

    #[clap(short, long, default_value = "\t")]
    delimiter: String,
}

fn main() {
    let args = Args::parse();

    let mut lines: Vec<Vec<String>> = Vec::new();
    for _i in 0..args.lines {
        lines.push(Vec::new());
    }

    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.expect("Failed to read line");
        lines[i % args.lines].push(line);
    }

    for l in lines {
        println!("{}", l.join(&args.delimiter));
    }
}
