#[macro_use]
extern crate serde_derive;
extern crate getopts;
extern crate rust_client;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

mod config;
mod engine;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "file", "set input file name", "FILE_PATH");
    opts.optopt("o", "operation", "set the operation", "VALUE");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let path: String = match matches.opt_str("f") {
        Some(s) => s,
        None => String::new(),
    };
    let op: String = match matches.opt_str("o") {
        Some(s) => s,
        None => String::new(),
    };

    match engine::initialize_engine(&path, &op).run() {
        Ok(()) => (),
        Err(e) => panic!(e),
    }
}
