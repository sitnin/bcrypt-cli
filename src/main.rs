use std::env;
use std::io;
use std::string::String;

extern crate bcrypt;
extern crate getopts;

use bcrypt::{hash, DEFAULT_COST};
use getopts::Options;

fn grab_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let trimmed = input.as_str().trim().to_string();
            if trimmed.len() > 0 {
                trimmed
            } else {
                panic!("Empty input")
            }
        }
        Err(error) => panic!(error),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] INPUT", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "cost", "set desired encrypting cost in rounds", "COST");
    opts.optflag("s", "short", "print hashed string only");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let cost: u32 = match matches.opt_str("c") {
        Some(s) => match s.parse::<u32>() {
            Ok(r) => r,
            Err(r) => panic!(r),
        },
        None => DEFAULT_COST,
    };

    let input_str: String = if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    } else {
        let input_src = matches.free[0].clone();
        match input_src.as_str() {
            "-" => grab_input(),
            _ => input_src,
        }
    };

    let result = hash(&input_str, cost).ok().unwrap();

    if matches.opt_present("s") {
        println!("{}", result);
    } else {
        println!("[{}] ({}) => [{}]", input_str, cost, result);
    }
}
