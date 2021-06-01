use std::io;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "split & paste")]
struct Opt {
    #[structopt(short, long)]
    lines: usize,

    #[structopt(short, long, default_value = "\t")]
    delimiter: String,
}

fn main() {
    let opt = Opt::from_args();

    let mut lines: Vec<Vec<String>> = Vec::new();
    for _i in 0..opt.lines {
        lines.push(Vec::new());
    }

    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.expect("Failed to read line");
        lines[i % opt.lines].push(line);
    }

    for l in lines {
        println!("{}", l.join(&opt.delimiter));
    }
}
