extern crate getopts;

use getopts::Options;
use std::env;

use std::io::prelude::*;
use std::fs;
use std::fs::File;

fn unarchive(root: &str, blocksize: u64, offset: u64) -> u8 {
    // TODO
    return 1;
}

fn read_hint(hint_path: &str) -> u64 {
    // TODO
    return 0;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

#[derive(PartialEq)]
enum Operation {
    Archive,
    Unarchive,
}

fn real_main() -> u8 {
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
        return 2;
    }

    if !(matches.opt_present("e") ^ matches.opt_present("u")) {
        print!("-e and -u don't make sense together\n");
        return 3;
    }

    let blocksize: u64 = match matches.opt_str("b") {
        Some(x) => x.parse().unwrap(),
        None => 1 * 1024 * 1024 * 1024
    };

    if blocksize <= 16 {
        print!("blocksize must be >16\n");
        return 3;
    }

    let op = if matches.opt_present("u") {
        Operation::Unarchive
    } else {
        Operation::Archive
    };

    if Operation::Unarchive == op {
        let offset: u64 = match matches.opt_str("u") {
            Some(x) => x.parse().unwrap(),
            None => panic!("unreachable"),
        };

        if 1 != matches.free.len() {
            print_usage(&program, opts);
            return 2;
        }

        return unarchive(matches.free[0].as_str(), blocksize, offset);
    }

    if 2 != matches.free.len() {
        print_usage(&program, opts);
        return 2;
    }

    let dest_root = matches.free[0].clone();
    let src_path = matches.free[1].as_str();

    let extra = match matches.opt_str("e") {
        Some(x) => x,
        None => String::from(""),
    };

    // read-only by default
    let mut src = match File::open(src_path) {
        Ok(x) => x,
        Err(e) => {
            print!("src file problem: {}: {}\n", src_path, e);
            return 4;
        }
    };

    let src_len: u64 = match fs::metadata(src_path) {
        Ok(x) => x.len(),
        Err(e) => {
            print!("src file doesn't stat: {}: {}\n", src_path, e);
            return 5;
        }
    };

    let hint_path = dest_root.clone() + ".hint";
    let hint: u64 = read_hint(hint_path.as_str());
    let mut skipped_due_to_locking = false;

    for target_num in 0..std::u64::MAX {
        let target_path = format!("{}.{:022}", dest_root, target_num);
        print!("{}\n", target_path);
        break;
    }

    print!("{} {}\n", blocksize, extra);
    return 0;
}

fn main() {
    std::process::exit(real_main() as i32);
}
