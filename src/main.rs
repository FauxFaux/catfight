extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("b", "block-size", "overflow point for file parts", "BYTES");
    opts.optopt("e", "extra", "extra metadata to include", "DATA");
    opts.optopt("u", "unarchive", "extract file from this offset", "OFFSET");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let blocksize: u64 = match matches.opt_str("b") {
        Some(x) => x.parse().unwrap(),
        None => 1 * 1024 * 1024 * 1024
    };

    print!("{}", blocksize)
}
