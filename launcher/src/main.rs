use std::{io, fs};

use ::vm::vm::VM;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(short, long, value_parser)]
    file: String,
}

fn main() {
    let args = Args::parse();

    // read file
    let instructions = fs::read_to_string(args.file).expect("file doesn't exist");

    // create VM
    let vm = VM::default();

    todo!("execute instructions");
}
