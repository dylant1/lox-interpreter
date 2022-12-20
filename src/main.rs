use std::process;
use std::env;
use std::fs;
use std::str;

mod err;
mod token_type;
mod token;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } 
    else if args.len() == 2 {
        run_file(&args[1]);
    }
    else {
        //run_prompt();
        println!("{}", String::from("interactive mode"));
        //err::error(0, "Test error");
    }
}

fn run_file(path: &str)  {
    let buf: Vec<u8> = fs::read(path).unwrap();
    let bytes: &str = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
    };
    run(bytes)
}

fn run(bytes: &str) {
    for byte in bytes.chars() {
        println!("{}", byte)
    }
    // use err::function() to run a function from err.rs
}
