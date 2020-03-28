use std::env;
use std::io;
use std::string::String;

extern crate bcrypt;
extern crate getopts;

use bcrypt::{hash, verify, DEFAULT_COST};
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
    let brief = format!(
        "Usage: {} [options] INPUT

        -c ROUNDS            -- set hashing cost in rounds (default: 12)
        --cost=ROUNDS

        -s                   -- set sime-silent mode (outputs only hash or verification result)
        --short

        -v HASH              -- switch to hash verification mode (will output only YES, NO or ERROR)
        --verify HASH",
        program
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "c",
        "cost",
        "set desired encrypting cost in rounds",
        "ROUNDS",
    );
    opts.optopt("v", "verify", "hash for verification", "HASH");
    opts.optflag(
        "s",
        "short",
        "print hashed string or verification result only",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

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

    let cost: u32 = match matches.opt_str("c") {
        Some(s) => match s.parse::<u32>() {
            Ok(r) => r,
            Err(r) => panic!(r),
        },
        None => DEFAULT_COST,
    };

    let re_output: String = match matches.opt_str("v") {
        Some(vh) => match verify(&input_str, &vh) {
            Ok(r) => match r {
                true => String::from("YES"),
                false => String::from("NO"),
            },
            Err(err) => format!("ERROR: {}", &err).to_string(),
        },
        None => hash(&input_str, cost).ok().unwrap(),
    };

    if matches.opt_present("s") {
        println!("{}", re_output);
    } else {
        match matches.opt_str("v") {
            Some(vh) => println!("[{}] =?= [{}] => [{}]", input_str, vh, re_output),
            None => println!("[{}] ({}) => [{}]", input_str, cost, re_output),
        }
    }
}
