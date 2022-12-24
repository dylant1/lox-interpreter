use std::process;
use std::env;
use std::fs;
use std::str;
use std::io::prelude::*;
use std::io::BufReader;
//use the scanner module
use scanner::Scanner;

//use the scanner module
mod scanner;
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
        run_prompt();
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
    let mut scanner = Scanner::new(String::from(bytes));
    let tokens = scanner.scan_tokens();
    for token in tokens {
        //println!("{}", token.to_string());
        println!("{:?}", token);
    }
}

fn run_prompt() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        run(&line);
    }
}
