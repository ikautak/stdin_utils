use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "split & transpose")]
struct Opt {
    #[structopt(short, long)]
    lines: usize,

    #[structopt(short, long, default_value = "\t")]
    delimiter: String,
}

fn main() {
    let opt = Opt::from_args();

    let mut lines: Vec<String> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        lines.push(line);

        if lines.len() == opt.lines {
            println!("{}", lines.join(&opt.delimiter));
            lines.clear();
        }
    }
}
